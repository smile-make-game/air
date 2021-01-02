mod core;
mod logger;
mod model;
mod view;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init logger
    logger::setup();
    log::debug!("logger configured");

    // config panic hook
    std::panic::set_hook(Box::new(|pi| {
        log::error!("panic: {:?}", pi);
    }));

    // run core
    let mut core = core::Core::default();
    core.init().await?;
    core.dead_loop().await?;

    Ok(())
}
