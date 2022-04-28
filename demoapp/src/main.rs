use crate::app::{App, AppState};
use crate::client::MyClient;

mod client;
mod app;

fn main() {
    println!("Hello, world!");

    let client = MyClient { ticks_left: 4 };

    let mut app = App {
        state: AppState {
            title: "Jack in the box".to_string(),
        },

        client: Box::new(client),
    };

    app.run();
}
