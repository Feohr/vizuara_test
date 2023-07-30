#![feature(panic_update_hook)]

mod btn_component;

use btn_component::ButtonComponent;
use gtk::prelude::*;
use gtk::*;
use std::process::Command;

fn main_layout() -> Box {
    // Icon holder
    let image = Image::builder()
        .file("data/Push_force/push_force.png")
        .height_request(10)
        .width_request(10)
        .build();

    // Button holder
    let btn = ButtonComponent::default();
    btn.set_height_request(10);
    btn.set_width_request(10);
    btn.set_label("Start push force");
    btn.set_image(Some(&image));

    let layout = gtk::Box::builder()
        .child(&btn)
        .orientation(Orientation::Vertical)
        .homogeneous(false)
        .build();

    layout
}

fn create_window(app: &Application) {
    let window = Window::builder()
        .application(app)
        .title("Vizuara example")
        .build();

    window.add(&main_layout());

    window.maximize();
    window.show_all();
}

fn clear_process() {
    let mut cmd = Command::new("./clear_process.sh").spawn().unwrap();
    cmd.wait().unwrap();
}

fn main() {
    std::panic::update_hook(move |prev, info| {
        clear_process();
        prev(info);
    });

    // Initialize GTK and create the window
    gtk::init().expect("Failed to initialize GTK.");

    // Show the window and start the GTK main loop
    let app = Application::builder()
        .application_id("com.test.vizuara.rehaan")
        .build();

    // Connect activate
    app.connect_activate(|app| create_window(app));

    app.run();
}
