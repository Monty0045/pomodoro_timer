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

    let countdown : gtk::Label = builder
        .get_object("timeLabel")
        .expect("Failed to get timeLabel");

    let play_button: gtk::Button = builder
        .get_object("play_button")
        .expect("Failed to get playButton");

    let reset_button: gtk::Button = builder
        .get_object("reset_button")
        .expect("Failed to get resetButton");


    createListeners(&play_button, &reset_button, &description, &countdown);


    window.set_application(Some(application));
    application.add_window(&window);
    window.present();
}

//Creates instance of a timer object as well as connect GUI objects to events and listens to timer
fn createListeners(play_button : &gtk::Button,
                  reset_button : &gtk::Button,
                  description : &gtk::Label,
                  countdown : &gtk::Label)
{
    let (messenger, receiver) = mpsc::channel();   //Will send messages to timer when play, reset

    let messenger_clone = mpsc::Sender::clone(&messenger);
    let play_clone = Mutex::new(play_button.clone());
    //let reset_clone = reset_button.clone();
    let descr_clone = description.clone();
    let countdown_clone = countdown.clone();


    //play/pause button clicked
    play_button.connect_clicked(move |_| {

        messenger_clone.send("play");
    });

    //reset button clicked
    reset_button.connect_clicked(move |_| {
        messenger.send("reset");
    });


    let mut counter = Arc::new(Mutex::new(timer::Timer::new()));    //timer 'object'

    timerListener(counter.clone(), countdown_clone);    //handles listening for events from timer


    //Thread sends GUI click events to timer.
    thread::spawn(move || {

        //message is event from user clicking a button
        for message in receiver {

            let mut timer_copy = counter.lock().unwrap();

            timer_copy.messageSent(message);    //Sends whether the user selected 'play' or 'reset'

        }

    });
}



fn timerListener(clock_timer : Arc<Mutex<timer::Timer>>,
    countdown_clone : gtk::Label
)
{
    let (sender, receiver) = glib::MainContext::channel::
        <std::string::String>(glib::PRIORITY_DEFAULT);


    timer::timerStart(clock_timer, sender);


    //receives value to update GUI
    receiver.attach(None, move |msg| {
        countdown_clone.set_label(msg.as_str());
        glib::Continue(true)
    });

}

