#![feature(panic_update_hook)]


use once_cell::sync::Lazy;
use gtk::prelude::*;
use gtk::*;
use std::process::Command;
use gtk::glib::*;
use webkit2gtk::{SettingsExt, WebContext, WebView, WebViewExt};

static PORT: Lazy<String> = Lazy::new(|| {
    format!("{}", std::fs::read_to_string("SERVE_PORT").unwrap().trim())
});

fn main_layout() -> Box {
    // Icon holder
    let image = Image::builder()
        .file("data/Push_force/push_force.png")
        .height_request(10)
        .width_request(10)
        .build();

    // Button holder
    let btn = Button::builder()
        .height_request(10)
        .width_request(10)
        .label("Start push force")
        .image(&image)
        .build();

    // The main window of the game
    let window = WeakRef::<Window>::new();

    btn.connect_clicked(move |_| {
        match window.upgrade() {
            // If game_window is not set
            None => {
                start_server();

                let window_inner = new_game_window();
                window_inner.connect_delete_event(|_, _| {
                    stop_server();
                    Inhibit(false)
                });
                window.set(Some(&window_inner));

                window_inner.show_all();
            }
            _ => {},
        }
    });

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

fn new_game_window() -> Window {
    let link = dbg!(format!("http://localhost:{}", PORT.to_string()));

    let context = WebContext::default().unwrap();
    let webview = WebView::with_context(&context);
    webview.load_uri(link.as_str());

    if let Some(settings) = WebViewExt::settings(&webview) {
        settings.set_enable_media(true);
        settings.set_enable_webaudio(true);
    }

    dbg!(webview.can_show_mime_type("application/wasm"));

    let window = Window::builder().title("Push force").build();
    window.add(&webview);
    window.maximize();

    window
}

fn stop_server() {
    let mut cmd = Command::new("./clear_process.sh").spawn().unwrap();
    cmd.wait().unwrap();
}

fn start_server() {
    Command::new("target/debug/server").spawn().unwrap();
}

fn main() {
    std::panic::update_hook(move |prev, info| {
        stop_server();
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
