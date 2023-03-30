use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        let mut result = InitializeResult::default();
        result.capabilities.text_document_sync =
            Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL));
        result.capabilities.completion_provider = Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: None,
            work_done_progress_options: Default::default(),
            all_commit_characters: None,
            ..Default::default()
        });
        Ok(result)
    }

    async fn initialized(&self, params: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, format!("{:?}", &params))
            .await;
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, format!("{:?}", &params))
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, format!("{:?}", &params))
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        self.client
            .log_message(MessageType::INFO, format!("{:?}", &params))
            .await;

        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
        ])))
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("You're hovering!".to_string())),
            range: None,
        }))
    }
}

#[tokio::main]
async fn main() {
    let read = tokio::io::stdin();
    let write = tokio::io::stdout();

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:12345")
    //     .await
    //     .unwrap();
    // let (stream, _) = listener.accept().await.unwrap();
    // let (read, write) = tokio::io::split(stream);
    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(read, write, socket).serve(service).await;
}
