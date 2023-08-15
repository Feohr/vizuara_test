#![feature(panic_update_hook)]

use anyhow::{bail, Result};
use gtk::glib::*;
use gtk::prelude::*;
use gtk::*;
use once_cell::sync::Lazy;
use std::fs::*;
use std::process::Child;
use std::process::Command;
use std::rc::Rc;
use webkit2gtk::{SettingsExt, WebContext, WebView, WebViewExt};

static PORT: Lazy<String> = Lazy::new(|| match std::fs::read_to_string("SERVE_PORT") {
    Ok(port) => format!("{}", port.trim()),
    Err(err) => {
        panic!("Cannot read SERVE_PORT value due to error: {}", err);
    }
});

fn create_button(dir_name: String, image_path: String) -> Button {
    // Icon holder
    let image = Image::builder()
        .file(&"data/{}".replace("{}", image_path.as_ref()))
        .height_request(10)
        .width_request(10)
        .build();

    let label = dir_name.replace("_", " ");
    // Button holder
    let btn = Button::builder()
        .height_request(10)
        .width_request(10)
        .label(&"Start {}".replace("{}", label.as_ref()))
        .image(&image)
        .name(dir_name.as_str())
        .build();

    btn.connect_clicked(move |_| {
        // The main window of the game
        let window = WeakRef::<Window>::new();

        match window.upgrade() {
            // If game_window is not set
            None => {
                start_server(dir_name.as_str());

                let window_inner = new_game_window(label.as_ref());
                window_inner.connect_delete_event(|_, _| {
                    stop_server();
                    Inhibit(false)
                });
                window.set(Some(&window_inner));

                window_inner.show_all();
            }
            _ => {}
        }
    });

    btn
}

fn main_layout() -> Result<Grid> {
    let buttons = scan_games()?;

    let layout = gtk::Grid::builder()
        .orientation(Orientation::Horizontal)
        .build();

    for button in buttons {
        layout.add(&button);
    }

    Ok(layout)
}

fn scan_games() -> Result<Vec<Button>> {
    let mut button_vec = Vec::<Button>::new();
    let dir = read_dir("data")?;
    for item in dir {
        let inner_dir = item?;
        button_vec.push(create_game_button(&inner_dir)?);
    }
    Ok(button_vec)
}

fn create_game_button(dir: &DirEntry) -> Result<Button> {
    let dir_name = dbg!(dir.file_name());
    let Ok(name) = dir_name.clone().into_string() else {
        bail!("The name of the directory {dir_name:?} contains invalid unicode characters");
    };
    let image_name = name.clone() + "/icon.png";

    Ok(create_button(name, image_name))
}

fn create_window(app: &Application) -> Result<()> {
    let window = Window::builder()
        .application(app)
        .title("Vizuara example")
        .build();

    window.add(&main_layout()?);
    window.connect_delete_event(move |_, _| {
        stop_server();
        Inhibit(false)
    });
    window.maximize();

    window.show_all();
    Ok(())
}

fn new_game_window(name: &str) -> Rc<Window> {
    let link = format!("http://localhost:{}", PORT.to_string());

    // When you're too bored, uncomment this and run the program.
    // let link = "https://silentspacemarine.com/".to_string();

    let context = WebContext::default().unwrap();
    let webview = WebView::with_context(&context);
    webview.load_uri(link.as_str());

    if let Some(settings) = WebViewExt::settings(&webview) {
        settings.set_enable_webgl(true);
        settings.set_enable_javascript(true);
        settings.set_enable_developer_extras(true);
        settings.set_enable_page_cache(false);
        settings.set_enable_html5_local_storage(false);
    }

    let titlebar = HeaderBar::builder().title(name).name(name).build();
    let close_btn = Button::from_icon_name(Some("window-close"), IconSize::Button);

    let window = Rc::new(Window::builder()
        .type_(WindowType::Toplevel)
        .title(name)
        .modal(true)
        .resizable(true)
        .build());

    let window_ref = Rc::clone(&window);
    close_btn.connect_clicked(move |_| {
        window_ref.close();
    });

    titlebar.pack_end(&close_btn);

    window.add(&webview);
    window.set_titlebar(Some(&titlebar));
    window.maximize();

    window
}

fn stop_server() {
    let command_wait = |cmd: &mut Child| {
        if let Err(err) = cmd.wait() {
            log::error!(
                "Error while waiting for the cleaning process to execute: {}",
                err
            );
        }
    };
    match Command::new("./clear_process.sh").spawn() {
        Ok(mut cmd) => command_wait(&mut cmd),
        Err(err) => {
            log::error!("Failed to run cleaning process script due to: {}", err);
        }
    }
}

fn start_server(name: &str) {
    log::info!("Starting server");
    if let Err(err) = Command::new("target/debug/server")
        .arg(name)
        .env("RUST_LOG", "info")
        .spawn()
    {
        log::error!("Failed to run the server due to: {}", err);
    }
}

fn main() {
    std::panic::update_hook(move |prev, info| {
        stop_server();
        prev(info);
    });

    env_logger::init();

    if let Err(err) = gtk::init() {
        log::error!("Failed to initialize GTK due to error: {}", err);
    }

    let Some(settings) = Settings::default() else {
        log::error!("Failed to get gtk settings");
        return
    };
    gtk::traits::SettingsExt::set_gtk_theme_name(&settings, Some("Adwaita"));

    let app = Application::builder()
        .application_id("com.test.vizuara.rehaan")
        .build();

    app.connect_activate(|app| {
        if let Err(err) = create_window(app) {
            log::error!("Error: {}", err);
            return;
        }
        log::info!("App started successfully");
    });

    app.run();
}
