mod core;
mod logger;
mod utilities;
mod view;
mod model;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::panic::set_hook(Box::new(|pi| {
        log::error!("panic: {:?}", pi);
    }));

    // init logger
    logger::setup();
    log::debug!("logger configured");

    // run core
    let core = core::Core::default();
    core.init().await?;
    core.dead_loop().await?;

    Ok(())
}
