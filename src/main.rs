use std::thread;
use std::time::Duration;

use gtk4::prelude::*;
use gtk4::{self, glib, Application, ApplicationWindow, Image, GestureClick};
use glib::{clone, MainContext, Priority};

const APP_ID: &str = "org.gtk_rs.MainEventLoop1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create an image
    let image = Image::from_file("listening.png");

    let (sender, receiver) = MainContext::channel(Priority::default());
    let sender = sender.clone();

    // Assign a click listener
    let gesture = GestureClick::new();
    gesture.connect_released(move |gesture, _, _, _| {
        gesture.set_state(gtk4::EventSequenceState::Claimed);
        let sender = sender.clone();
        // The long running operation runs now in a separate thread
        gio::spawn_blocking(move || {
            // Change the pic
            sender.send(true).expect("Could not send through channel");
            let delay = Duration::from_secs(5);
            thread::sleep(delay);
            // Change the pic back
            sender.send(false).expect("Could not send through channel");
        });
    });
    image.add_controller(gesture);

    // The main loop executes the closure as soon as it receives the message
    receiver.attach(
        None,
        clone!(@weak image => @default-return glib::ControlFlow::Break,
            move |transcribing| {
                if transcribing {
                    image.set_from_file(Some("transcribing.png"));
                }
                else {
                    image.set_from_file(Some("listening.png"));
                }
                glib::ControlFlow::Continue
            }
        ),
    );


    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Listener")
        .child(&image)
        .build();

    window.set_default_size(200, 200);


    // Present window
    window.present();
}
