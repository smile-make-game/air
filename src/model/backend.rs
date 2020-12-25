use anyhow::Result;

struct LocalBackend {}

impl LocalBackend {
    async fn run(self) -> Result<()> {
        Ok(())
    }
}

pub struct BackendEvent {}
