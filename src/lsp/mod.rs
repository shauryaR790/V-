#![cfg(feature = "lsp")]

use tower_lsp::jsonrpc::Result;
use tower_lsp::jsonrpc::Error as JsonRpcError;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::driver::check_with_index;
use crate::error::VppError;
use crate::span::Span;

struct VppLanguageServer {
    client: Client,
}

impl VppLanguageServer {
    fn new(client: Client) -> Self {
        Self { client }
    }

    async fn publish_diagnostics(&self, uri: Url, source: &str, path: &std::path::Path) {
        let diagnostics = match check_with_index(source, path) {
            Ok(_) => Vec::new(),
            Err(err) => vec![error_to_diagnostic(&err, source)],
        };

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for VppLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "vpp-language-server".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                definition_provider: Some(OneOf::Left(true)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "v++ language server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let source = params.text_document.text;
        if let Ok(path) = uri.to_file_path() {
            self.publish_diagnostics(uri, &source, &path).await;
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        if let Some(change) = params.content_changes.into_iter().next() {
            if let Ok(path) = uri.to_file_path() {
                self.publish_diagnostics(uri, &change.text, &path).await;
            }
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        if let Ok(path) = uri.to_file_path() {
            if let Ok(source) = std::fs::read_to_string(&path) {
                self.publish_diagnostics(uri, &source, &path).await;
            }
        }
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let path = uri.to_file_path().map_err(|_| {
            JsonRpcError::invalid_params("only file:// URIs are supported")
        })?;
        let source = std::fs::read_to_string(&path).map_err(|e| {
            JsonRpcError::invalid_params(format!("cannot read file: {e}"))
        })?;

        let typed = check_with_index(&source, &path).map_err(|e| {
            JsonRpcError::invalid_params(format!("{e}"))
        })?;

        let word = word_at_position(&source, params.text_document_position_params.position);
        if let Some(name) = word {
            if let Some(def) = typed.symbols.lookup(&name) {
                let loc = Location {
                    uri: Url::from_file_path(&def.file).unwrap_or(uri),
                    range: span_to_range(&source, def.span),
                };
                return Ok(Some(GotoDefinitionResponse::Scalar(loc)));
            }
        }

        Ok(None)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let path = uri.to_file_path().map_err(|_| {
            JsonRpcError::invalid_params("only file:// URIs are supported")
        })?;
        let source = std::fs::read_to_string(&path).map_err(|e| {
            JsonRpcError::invalid_params(format!("cannot read file: {e}"))
        })?;

        let mut items = vec![
            completion_item("let", "let ${1:name} = ${2:value}"),
            completion_item("fn", "fn ${1:name}(${2:params}) -> ${3:type} {\n\t$0\n}"),
            completion_item("struct", "struct ${1:Name} {\n\t${2:field}: ${3:type}\n}"),
            completion_item("enum", "enum ${1:Name} {\n\t${2:Variant}\n}"),
            completion_item("match", "match ${1:expr} {\n\t${2:pattern} => {\n\t\t$0\n\t}\n}"),
            completion_item("import", "import \"${1:file.vpp}\""),
            completion_item("print", "print(${1:value})"),
        ];

        if let Ok(typed) = check_with_index(&source, &path) {
            for name in typed.symbols.defs.keys() {
                items.push(CompletionItem {
                    label: name.clone(),
                    kind: Some(CompletionItemKind::FUNCTION),
                    ..Default::default()
                });
            }
        }

        Ok(Some(CompletionResponse::Array(items)))
    }
}

fn completion_item(label: &str, insert: &str) -> CompletionItem {
    CompletionItem {
        label: label.to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        insert_text: Some(insert.to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..Default::default()
    }
}

fn error_to_diagnostic(err: &VppError, source: &str) -> Diagnostic {
    let range = err
        .source_span()
        .map(|span| span_to_range(source, Span::new(span.offset(), span.offset() + span.len())))
        .unwrap_or_default();

    Diagnostic {
        range,
        severity: Some(DiagnosticSeverity::ERROR),
        source: Some("vpp".to_string()),
        message: err.to_string(),
        ..Default::default()
    }
}

fn span_to_range(source: &str, span: Span) -> Range {
    Range {
        start: offset_to_position(source, span.start),
        end: offset_to_position(source, span.end),
    }
}

fn offset_to_position(source: &str, offset: usize) -> Position {
    let mut line = 0u32;
    let mut character = 0u32;
    for (i, ch) in source.char_indices() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            character = 0;
        } else {
            character += 1;
        }
    }
    Position { line, character }
}

fn word_at_position(source: &str, position: Position) -> Option<String> {
    let offset = source
        .lines()
        .take(position.line as usize)
        .map(|l| l.len() + 1)
        .sum::<usize>()
        + position.character as usize;

    if offset >= source.len() {
        return None;
    }

    let bytes = source.as_bytes();
    let mut start = offset;
    while start > 0
        && ((bytes[start - 1] as char).is_ascii_alphanumeric() || bytes[start - 1] == b'_')
    {
        start -= 1;
    }
    let mut end = offset;
    while end < bytes.len()
        && ((bytes[end] as char).is_ascii_alphanumeric() || bytes[end] == b'_')
    {
        end += 1;
    }

    if start == end {
        None
    } else {
        Some(source[start..end].to_string())
    }
}

pub async fn run_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(VppLanguageServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
