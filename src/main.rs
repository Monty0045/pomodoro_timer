use gettextrs::*;
use gio::prelude::*;
use gtk::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};

mod config;
//mod window;
mod timer;
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
        buildUI(&app);
        //timer::testing();
    });


    let ret = app.run(&std::env::args().collect::<Vec<_>>());
    std::process::exit(ret);
}




fn buildUI(application: &gtk::Application)
{
    let builder = gtk::Builder::new_from_resource("/org/example/App/window.ui");

    let window : gtk::ApplicationWindow = builder.get_object("window").expect("Failed to get window");

    let description : gtk::Label = builder
        .get_object("descriptionLabel")
        .expect("Failed to get descriptionLabel");

    let play_button: gtk::Button = builder
        .get_object("play_button")
        .expect("Failed to get playButton");

    let reset_button: gtk::Button = builder
        .get_object("reset_button")
        .expect("Failed to get resetButton");

    createListeners(&play_button, &reset_button);


    window.set_application(Some(application));
    application.add_window(&window);
    window.present();
}

fn createListeners(play_button : &gtk::Button, reset_button : &gtk::Button)
{
    let (messenger, receiver) = mpsc::channel();   //Will send messages to timer when play, reset

    let messenger_clone = mpsc::Sender::clone(&messenger);

    play_button.connect_clicked(move |_| {

        messenger_clone.send("play");
        //desc_clone.set_text("From play_button");
    });


    reset_button.connect_clicked(move |_| {
        messenger.send("reset");
        //desc_clone.set_text("From reset _button");
    });

    thread::spawn(move || {

        let mut counter = Arc::new(Mutex::new(timer::Timer::new()));
        //counter.initiateTimer();
        timer::timerStart(counter.clone());

        for message in receiver {

            let mut timer_copy = counter.lock().unwrap();

            timer_copy.messageSent(message);
            //println!("This happens {}", message);

        }

    });

    println!("There");
}
