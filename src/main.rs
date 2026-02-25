#[tokio::main]
async fn main() -> anyhow::Result<()> {
    femtoclaw_cli::run().await
}
