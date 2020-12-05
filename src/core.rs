use crate::{model::*, utilities::input_piper::*, view::*};
use anyhow::Result;
use std::cell::RefCell;

pub struct Core {
    view: ViewWrapper,
    model: RefCell<DataModel>,
}

impl Default for Core {
    fn default() -> Self {
        Core {
            view: ViewWrapper::default(),
            model: RefCell::new(DataModel::default()),
        }
    }
}

impl Core {
    pub async fn init(&self) -> Result<()> {
        self.model.borrow_mut().load().await?;
        self.view.apply_evolution(self.model.borrow().get_evolution())?;
        Ok(())
    }

    pub async fn dead_loop(&self) -> Result<()> {
        let result = self.priv_dead_loop().await;
        if let Err(e) = result {
            log::error!("error in dead loop: {:?}", e);
            return Err(anyhow::Error::msg(
                "unexpected error occurred, please check log for detail information",
            ));
        }
        Ok(())
    }

    async fn priv_dead_loop(&self) -> Result<()> {
        loop {
            let effect: Option<Evolution>;

            // read input
            let input = tick_or_event()?;

            // process input
            if let Input::Event(event) = input {
                if should_exit(&event) {
                    // TODO: save data

                    break;
                }
                effect = self.view.quick_update(&event)?;
            } else {
                effect = None;
            }

            // apply data patch
            let mut data = self.model.borrow_mut();
            let view_patch = data.evolute(effect).await?;

            // apply view patch
            if let Some(view_patch) = view_patch {
                self.view.apply_evolution(view_patch)?;
            }
        }
        Ok(())
    }
}
