use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use super::config::Config;

pub struct App {
    application: Application,
    config: Config,
}

impl App {
    pub fn new() -> Self {
        let config = Config::load();
        let application = Application::new(
            Some("com.example.app"),
            Default::default(),
        );

        let app = Self {
            application,
            config,
        };

        app.setup_gtk_application();
        app
    }

    fn setup_gtk_application(&self) {
        // Votre code pour configurer l'application GTK
    }

    pub fn run(&self) {
        self.application.run();
    }
}
