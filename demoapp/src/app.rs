use std::thread::sleep;
use std::time::Duration;
use crate::client::{Client, UpdateResult};

pub struct App {
    pub(crate) client: Box<dyn Client>,
    pub(crate) state: AppState,
}

pub struct AppState {
    pub(crate) title: String,
}

impl App {
    pub(crate) fn run(&mut self) {
        println!("=== You are now playing {} ===", self.state.title);

        loop {
            let res = self.client.update();
            self.client.render();

            match res {
                UpdateResult::None => {}
                UpdateResult::QuitApplication => {
                    return;
                }
            }

            sleep(Duration::from_secs(1));
        }
    }
}