use crate::error::{VppError, VppResult};
use crate::lexer::{Lexer, TokenKind};

/// Basic v++ formatter: normalizes spacing and indents braced blocks.
pub fn format(source: &str) -> VppResult<String> {
    let tokens = Lexer::new(source).tokenize()?;
    let mut out = String::new();
    let mut indent = 0usize;
    let mut prev_was_newline = true;
    let mut paren_depth = 0usize;

    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        let kind = &token.kind;

        if matches!(kind, TokenKind::Eof) {
            break;
        }

        if matches!(kind, TokenKind::Newline) {
            if !out.ends_with('\n') {
                out.push('\n');
            }
            prev_was_newline = true;
            i += 1;
            continue;
        }

        if prev_was_newline && !matches!(kind, TokenKind::RBrace) {
            out.push_str(&"    ".repeat(indent));
            prev_was_newline = false;
        }

        match kind {
            TokenKind::LBrace => {
                out.push('{');
                indent += 1;
                if tokens.get(i + 1).is_some_and(|t| !matches!(t.kind, TokenKind::Newline | TokenKind::RBrace)) {
                    out.push('\n');
                    prev_was_newline = true;
                }
            }
            TokenKind::RBrace => {
                indent = indent.saturating_sub(1);
                if !prev_was_newline {
                    out.push('\n');
                    out.push_str(&"    ".repeat(indent));
                }
                out.push('}');
                if tokens.get(i + 1).is_some_and(|t| !matches!(t.kind, TokenKind::Eof | TokenKind::RBrace | TokenKind::Newline)) {
                    out.push('\n');
                    prev_was_newline = true;
                }
            }
            TokenKind::Comma => {
                out.push(',');
                if paren_depth == 0 {
                    out.push(' ');
                }
            }
            TokenKind::Colon => out.push(':'),
            TokenKind::Arrow => out.push_str(" -> "),
            TokenKind::DotDot => out.push_str(".."),
            TokenKind::LParen => {
                paren_depth += 1;
                out.push('(');
            }
            TokenKind::RParen => {
                paren_depth = paren_depth.saturating_sub(1);
                out.push(')');
            }
            TokenKind::LBracket => out.push('['),
            TokenKind::RBracket => out.push(']'),
            TokenKind::Eq => out.push_str(" = "),
            TokenKind::EqEq => out.push_str(" == "),
            TokenKind::BangEq => out.push_str(" != "),
            TokenKind::Lt => out.push_str(" < "),
            TokenKind::LtEq => out.push_str(" <= "),
            TokenKind::Gt => out.push_str(" > "),
            TokenKind::GtEq => out.push_str(" >= "),
            TokenKind::AndAnd => out.push_str(" && "),
            TokenKind::OrOr => out.push_str(" || "),
            TokenKind::Plus => out.push_str(" + "),
            TokenKind::Minus => out.push_str(" - "),
            TokenKind::Star => out.push_str(" * "),
            TokenKind::Slash => out.push_str(" / "),
            TokenKind::Percent => out.push_str(" % "),
            TokenKind::Bang => out.push('!'),
            TokenKind::StringLit(s) => {
                out.push('"');
                out.push_str(s);
                out.push('"');
            }
            other => {
                out.push_str(&other.to_string());
                if needs_space_after(other) {
                    out.push(' ');
                }
            }
        }

        i += 1;
    }

    if !out.ends_with('\n') {
        out.push('\n');
    }

    Ok(out)
}

fn needs_space_after(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Let
            | TokenKind::Fn
            | TokenKind::If
            | TokenKind::Else
            | TokenKind::While
            | TokenKind::For
            | TokenKind::In
            | TokenKind::Return
            | TokenKind::True
            | TokenKind::False
            | TokenKind::IntType
            | TokenKind::FloatType
            | TokenKind::BoolType
            | TokenKind::StringType
            | TokenKind::IntLit(_)
            | TokenKind::FloatLit(_)
            | TokenKind::Ident(_)
    )
}

pub fn format_file(path: &std::path::Path) -> VppResult<()> {
    let source = std::fs::read_to_string(path).map_err(|source| VppError::Io { source })?;
    let formatted = format(&source)?;
    std::fs::write(path, formatted).map_err(|source| VppError::Io { source })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_block_indentation() {
        let out = format("if x > 0 { print(x) }").unwrap();
        assert!(out.contains("if x"));
        assert!(out.contains("{\n"));
    }
}
