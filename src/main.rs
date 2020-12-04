mod logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init logger
    logger::setup();
    log::debug!("logger configured");

    Ok(())
}
