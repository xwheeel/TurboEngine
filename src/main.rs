mod application;

use application::Application;

fn main() {
    let application = Application::new();
    application.run();
}
