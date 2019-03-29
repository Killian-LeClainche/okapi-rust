mod okapi;

use okapi::{Application, App, Settings};

fn main() {
    println!("Hello, world!");

    let app = Application::new(false, App { settings: Settings::new() });

}