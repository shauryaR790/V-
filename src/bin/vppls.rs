#[cfg(feature = "lsp")]
#[tokio::main]
async fn main() {
    vpp::lsp::run_server().await;
}

#[cfg(not(feature = "lsp"))]
fn main() {
    eprintln!("vppls requires `--features lsp`. Build with: cargo build --features lsp --bin vppls");
    std::process::exit(1);
}
