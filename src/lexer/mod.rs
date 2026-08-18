mod token;

pub use token::{Token, TokenKind};

use crate::error::{span_to_source, VppError, VppResult};
use crate::span::Span;

pub struct Lexer<'source> {
    source: &'source str,
    chars: std::iter::Peekable<std::str::CharIndices<'source>>,
    current: usize,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            current: 0,
        }
    }

    pub fn tokenize(mut self) -> VppResult<Vec<Token>> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token.kind, TokenKind::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }

    fn next_token(&mut self) -> VppResult<Token> {
        self.skip_whitespace_and_comments();

        let start = self.current;
        let ch = match self.peek_char() {
            Some(c) => c,
            None => return Ok(Token::new(TokenKind::Eof, Span::new(start, start))),
        };

        let kind = match ch {
            '(' => {
                self.advance();
                TokenKind::LParen
            }
            ')' => {
                self.advance();
                TokenKind::RParen
            }
            '{' => {
                self.advance();
                TokenKind::LBrace
            }
            '}' => {
                self.advance();
                TokenKind::RBrace
            }
            '[' => {
                self.advance();
                TokenKind::LBracket
            }
            ']' => {
                self.advance();
                TokenKind::RBracket
            }
            ',' => {
                self.advance();
                TokenKind::Comma
            }
            ':' => {
                self.advance();
                TokenKind::Colon
            }
            '+' => {
                self.advance();
                TokenKind::Plus
            }
            '-' => {
                self.advance();
                if self.match_char('>') {
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                }
            }
            '*' => {
                self.advance();
                TokenKind::Star
            }
            '/' => {
                self.advance();
                TokenKind::Slash
            }
            '%' => {
                self.advance();
                TokenKind::Percent
            }
            '=' => {
                self.advance();
                if self.match_char('=') {
                    TokenKind::EqEq
                } else if self.match_char('>') {
                    TokenKind::FatArrow
                } else {
                    TokenKind::Eq
                }
            }
            '!' => {
                self.advance();
                if self.match_char('=') {
                    TokenKind::BangEq
                } else {
                    TokenKind::Bang
                }
            }
            '<' => {
                self.advance();
                if self.match_char('=') {
                    TokenKind::LtEq
                } else {
                    TokenKind::Lt
                }
            }
            '>' => {
                self.advance();
                if self.match_char('=') {
                    TokenKind::GtEq
                } else {
                    TokenKind::Gt
                }
            }
            '&' => {
                self.advance();
                if self.match_char('&') {
                    TokenKind::AndAnd
                } else {
                    return Err(VppError::InvalidCharacter {
                        ch: '&',
                        span: span_to_source(self.source, Span::new(start, self.current)),
                    });
                }
            }
            '|' => {
                self.advance();
                if self.match_char('|') {
                    TokenKind::OrOr
                } else {
                    return Err(VppError::InvalidCharacter {
                        ch: '|',
                        span: span_to_source(self.source, Span::new(start, self.current)),
                    });
                }
            }
            '.' => {
                self.advance();
                if self.match_char('.') {
                    TokenKind::DotDot
                } else {
                    TokenKind::Dot
                }
            }
            '"' => self.string_literal()?,
            c if c.is_ascii_digit() => self.number_literal()?,
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier_or_keyword(),
            '\n' => {
                self.advance();
                TokenKind::Newline
            }
            c => {
                return Err(VppError::InvalidCharacter {
                    ch: c,
                    span: span_to_source(self.source, Span::new(start, start + c.len_utf8())),
                });
            }
        };

        Ok(Token::new(kind, Span::new(start, self.current)))
    }

    fn string_literal(&mut self) -> VppResult<TokenKind> {
        self.advance(); // opening "
        let start = self.current;
        let mut value = String::new();

        while let Some(ch) = self.peek_char() {
            if ch == '"' {
                self.advance();
                return Ok(TokenKind::StringLit(value));
            }
            if ch == '\n' {
                return Err(VppError::UnterminatedString {
                    span: span_to_source(self.source, Span::new(start - 1, self.current)),
                });
            }
            if ch == '\\' {
                self.advance();
                let escaped = self.peek_char().ok_or_else(|| VppError::UnterminatedString {
                    span: span_to_source(self.source, Span::new(start - 1, self.current)),
                })?;
                self.advance();
                let decoded = match escaped {
                    'n' => '\n',
                    't' => '\t',
                    '\\' => '\\',
                    '"' => '"',
                    other => other,
                };
                value.push(decoded);
            } else {
                self.advance();
                value.push(ch);
            }
        }

        Err(VppError::UnterminatedString {
            span: span_to_source(self.source, Span::new(start - 1, self.current)),
        })
    }

    fn number_literal(&mut self) -> VppResult<TokenKind> {
        let start = self.current;
        while self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }

        if self.peek_char() == Some('.') {
            let peek_next = self.peek_char_at(1);
            if peek_next.is_some_and(|c| c.is_ascii_digit()) {
                self.advance(); // consume '.'
                while self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                    self.advance();
                }
                let text = &self.source[start..self.current];
                let value: f64 = text.parse().map_err(|_| VppError::InvalidCharacter {
                    ch: '.',
                    span: span_to_source(self.source, Span::new(start, self.current)),
                })?;
                return Ok(TokenKind::FloatLit(value));
            }
        }

        let text = &self.source[start..self.current];
        let value: i64 = text
            .parse()
            .map_err(|_| VppError::InvalidCharacter {
                ch: text.chars().next().unwrap_or('0'),
                span: span_to_source(self.source, Span::new(start, self.current)),
            })?;
        Ok(TokenKind::IntLit(value))
    }

    fn identifier_or_keyword(&mut self) -> TokenKind {
        let start = self.current;
        while self
            .peek_char()
            .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            self.advance();
        }
        let text = &self.source[start..self.current];
        match text {
            "let" => TokenKind::Let,
            "fn" => TokenKind::Fn,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "return" => TokenKind::Return,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "int" => TokenKind::IntType,
            "float" => TokenKind::FloatType,
            "bool" => TokenKind::BoolType,
            "string" => TokenKind::StringType,
            "struct" => TokenKind::Struct,
            "enum" => TokenKind::Enum,
            "import" => TokenKind::Import,
            "pub" => TokenKind::Pub,
            "match" => TokenKind::Match,
            "test" => TokenKind::Test,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "Option" => TokenKind::Option,
            "Result" => TokenKind::Result,
            _ => TokenKind::Ident(text.to_string()),
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            while self.peek_char().is_some_and(|c| c == ' ' || c == '\t' || c == '\r') {
                self.advance();
            }

            if self.peek_char() == Some('/') && self.peek_char_at(1) == Some('/') {
                while self.peek_char().is_some_and(|c| c != '\n') {
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, c)| *c)
    }

    fn peek_char_at(&mut self, offset: usize) -> Option<char> {
        self.source[self.current..].chars().nth(offset)
    }

    fn advance(&mut self) -> Option<char> {
        let (_, ch) = self.chars.next()?;
        self.current += ch.len_utf8();
        Some(ch)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek_char() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(source: &str) -> Vec<TokenKind> {
        Lexer::new(source)
            .tokenize()
            .unwrap()
            .into_iter()
            .map(|t| t.kind)
            .collect()
    }

    #[test]
    fn lexes_hello() {
        let tokens = kinds("print(\"Hello\")");
        assert!(matches!(tokens[0], TokenKind::Ident(_)));
        assert!(matches!(tokens[1], TokenKind::LParen));
        assert!(matches!(tokens[2], TokenKind::StringLit(_)));
    }

    #[test]
    fn lexes_keywords() {
        let tokens = kinds("let fn if else while for in return true false");
        assert_eq!(tokens.len(), 11);
    }

    #[test]
    fn lexes_range_operator() {
        let tokens = kinds("0..10");
        assert!(matches!(tokens[1], TokenKind::DotDot));
    }

    #[test]
    fn skips_comments() {
        let tokens = kinds("// comment\nlet x = 1");
        assert!(tokens.iter().any(|k| matches!(k, TokenKind::Let)));
    }
}
