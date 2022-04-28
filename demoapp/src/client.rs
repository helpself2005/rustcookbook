pub trait Client {
    fn update(&mut self) -> UpdateResult;

    fn render(&self);
}

pub struct MyClient {
    pub(crate) ticks_left: usize,
}

pub enum UpdateResult {
    None,
    QuitApplication,
}

impl Client for MyClient {
    fn update(&mut self) -> UpdateResult {
        self.ticks_left -= 1;
        if self.ticks_left == 0 {
            UpdateResult::QuitApplication
        } else {
            UpdateResult::None
        }
    }

    fn render(&self) {
        if self.ticks_left > 0 {
            println!("You turn the crank...");
        } else {
            println!("Jack POPS OUT OF THE BOX");
        }
    }
}