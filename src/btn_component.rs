use gtk::glib::*;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::*;
use std::process::Command;
use webkit2gtk::{SettingsExt, WebContext, WebView, WebViewExt};

mod btn_imp {
    use super::*;

    #[derive(Default)]
    pub struct ButtonComponent {
        pub window: WeakRef<Window>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ButtonComponent {
        const NAME: &'static str = "ButtonComponent";
        type ParentType = gtk::Button;
        type Type = super::ButtonComponent;

        fn new() -> Self {
            ButtonComponent {
                window: WeakRef::<Window>::new(),
            }
        }
    }

    impl ObjectImpl for ButtonComponent {}

    impl WidgetImpl for ButtonComponent {}

    impl ButtonImpl for ButtonComponent {
        fn clicked(&self) {
            match self.window.upgrade() {
                Some(window) => {
                    window.show_all();
                }
                // If game_window is not set
                None => {
                    new_server();

                    let window = self.obj().new_window();
                    window.connect_delete_event(|_, _| {
                        crate::clear_process();
                        Inhibit(false)
                    });
                    self.window.set(Some(&window));

                    window.show_all();
                }
            }
        }
    }

    impl BinImpl for ButtonComponent {}

    impl ContainerImpl for ButtonComponent {}
}

impl ButtonComponent {
    fn new_window(&self) -> Window {
        let _port = std::fs::read_to_string("SERVE_PORT").unwrap();
        let link = format!("http://localhost:{}", _port.trim());

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
}

glib::wrapper! {
    pub struct ButtonComponent(ObjectSubclass<btn_imp::ButtonComponent>)
        @extends gtk::Widget, gtk::Button;
}

impl Default for ButtonComponent {
    fn default() -> Self {
        Object::new::<Self>(&[])
            .downcast()
            .expect("Error while downcasting ButtonComponent object")
    }
}

fn new_server() {
    Command::new("./target/debug/server").spawn().unwrap();
}
