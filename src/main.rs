use std::env;
use std::path::PathBuf;use gtk4::prelude::*;
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
    let image_path = asset_path("listening.png");
    println!("image_path: {:?}", image_path);
let image = Image::from_file(image_path);

    let (sender, receiver) = MainContext::channel(Priority::default());
    let sender = sender.clone();

    start_recording();

    // Assign a click listener
    let gesture = GestureClick::new();
    gesture.connect_released(move |gesture, _, _, _| {
        gesture.set_state(gtk4::EventSequenceState::Claimed);
        let sender = sender.clone();
        // The long running operation runs now in a separate thread
        gio::spawn_blocking(move || {
            // Stop recording and show that we are now transcribing
            sender.send(true).expect("Could not send through channel");
            stop_recording_do_transcribe();
            // let delay = Duration::from_secs(5);
            // thread::sleep(delay);
            // // Change the pic back
            // sender.send(false).expect("Could not send through channel");
        });
    });
    image.add_controller(gesture);

    // The main loop executes the closure as soon as it receives the message
    receiver.attach(
        None,
        clone!(@weak image => @default-return glib::ControlFlow::Break,
            move |transcribing| {
                if transcribing {
                    let image_path = asset_path("transcribing.png");
                    println!("image_path: {:?}", image_path);
                    image.set_from_file(Some(image_path));
                }
                else {
                    image.set_from_file(Some(asset_path("listening.png")));
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

fn asset_path(filename: &str) -> PathBuf {
    #[cfg(debug_assertions)]
    let in_development = true;
    
    #[cfg(not(debug_assertions))]
    let in_development = false;
    println!("in_development: {:?}", in_development);

    if in_development {
        // In development mode, load assets from the "assets/" directory
        PathBuf::from(format!("assets/{}", filename))
    } else {
        // In production mode, load assets from the Resources directory
        let resources_dir = env::current_exe()
            .expect("failed to get current executable")
            .parent().expect("failed to get parent directory of current executable")
            .join("../Resources");
        resources_dir.join(filename)
    }
}

// Function to start recording from the microphone, and saving to tmp file
//  This will just execute `rec my_audio_clip.wav` command-line script using bash, and keep it running
fn start_recording() {
    let rec_path = env!("REC_PATH");
    let _ = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!("rm -Rf /tmp/whisper-dictate ; mkdir -p /tmp/whisper-dictate ; {} /tmp/whisper-dictate/recording.wav", rec_path))
        .spawn()
        .expect("Could not start recording");
}

fn stop_recording_do_transcribe() {
    let whisper_path = env!("WHISPER_PATH");
    let pbcopy_path = env!("PBCOPY_PATH");
    let cat_path = env!("CAT_PATH");
    let killall_path = env!("KILLALL_PATH");
    let mut thread = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!("{} rec ; sleep 2 ; cd /tmp/whisper-dictate/ ; {} recording.wav --model small.en ; {} recording.txt | {}", killall_path, whisper_path, cat_path, pbcopy_path))
        .spawn()
        .expect("Could not stop recording");

    // wait until the thread is done
    thread.wait().expect("Could not wait for thread");

    // then kill the app, close the window
    std::process::exit(0);
}

