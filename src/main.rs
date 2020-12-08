use gettextrs::*;
use gio::prelude::*;
use gtk::prelude::*;

mod config;
mod window;
//use crate::window::Window;

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain("pomodororust", config::LOCALEDIR);
    textdomain("pomodororust");

    let res = gio::Resource::load(config::PKGDATADIR.to_owned() + "/pomodororust.gresource")
        .expect("Could not load resources");
    gio::resources_register(&res);

    let app = gtk::Application::new(Some("org.example.App"), Default::default()).unwrap();
    app.connect_activate(move |app| {
        window::buildUI(&app);
    });


    let ret = app.run(&std::env::args().collect::<Vec<_>>());
    std::process::exit(ret);
}
