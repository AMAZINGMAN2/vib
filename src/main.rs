use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, Box, HeaderBar, ScrolledWindow};
use webkit2gtk::{WebView, WebContext};
use webkit2gtk::WebViewExt; 
use glib::clone;
use gtk::Adjustment;

fn main() {
    // Initialize GTK application
    let app = Application::new(Some("com.hashim.vib"), Default::default());

    app.connect_activate(|app| {
        // Create a main application window
        let window = ApplicationWindow::new(app);
        window.set_title("Vib");
        window.set_default_size(800, 600);

        // Create a header bar
        let header = HeaderBar::new();
        header.set_title(Some("Vib"));
        header.set_show_close_button(true);
        window.set_titlebar(Some(&header));

        // Create a horizontal box for the URL entry and buttons
        let hbox = Box::new(gtk::Orientation::Horizontal, 5);

        // Create URL entry
        let url_entry = Entry::new();
        hbox.pack_start(&url_entry, true, true, 0);

        // Create a WebView

        let web_context = WebContext::default().expect("Failed to create WebContext");
        let web_view = WebView::with_context(&web_context);
        let scrolled_window = ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
        scrolled_window.add(&web_view);
        window.add(&scrolled_window);

        // Create navigation buttons
        let back_button = Button::with_label("Back");
        let forward_button = Button::with_label("Forward");
        let refresh_button = Button::with_label("Refresh");

        // Connect button signals
        back_button.connect_clicked(clone!(@strong web_view => move |_| {
            if web_view.can_go_back() {
                web_view.go_back();
            }
        }));

        forward_button.connect_clicked(clone!(@strong web_view => move |_| {
            if web_view.can_go_forward() {
                web_view.go_forward();
            }
        }));

        refresh_button.connect_clicked(clone!(@strong web_view => move |_| {
            web_view.reload();
        }));

        // Pack buttons into the header
        header.pack_start(&back_button);
        header.pack_start(&forward_button);
        header.pack_start(&refresh_button);

        // Load the initial URL when Enter is pressed
        url_entry.connect_activate(clone!(@strong web_view, @strong url_entry => move |_| {
            let url = url_entry.text();
            if !url.starts_with("http://") && !url.starts_with("https://") {
                url_entry.set_text(&format!("http://{}", url));
            }
            web_view.load_uri(&url_entry.text());
        }));

        // Show all components
        window.show_all();
    });

    // Run the application
    app.run();
}
