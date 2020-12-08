use gtk::prelude::*;



//Builds application (gets gtk components and creates listeners)
pub fn buildUI(application: &gtk::Application) {

    let builder = gtk::Builder::new_from_resource("/org/example/App/window.ui");

    let window : gtk::ApplicationWindow = builder.get_object("window").expect("Failed to get window");

    let play_button: gtk::Button = builder
        .get_object("play_button")
        .expect("Failed to get playButton");

    play_button.connect_clicked(|_| {
        println!("Play button clicked");
    });

    let reset_button: gtk::Button = builder
        .get_object("reset_button")
        .expect("Failed to get resetButton");

    reset_button.connect_clicked(|_| {
        println!("Reset button clicked");
    });

    window.set_application(Some(application));
    application.add_window(&window);
    window.present();

}
