use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, Box, ScrolledWindow, ProgressBar, Orientation};
use webkit2gtk::{WebView, WebContext, WebViewExt, LoadEvent}; 
use glib::clone;
use gtk::Adjustment;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    // Initialize GTK application
    let app = Application::new(Some("com.hashim.vib"), Default::default());

    app.connect_activate(|app| {
        // Create a main application window
        let window = ApplicationWindow::new(app);
        window.set_title("Vib");
        window.set_default_size(800, 600);

        // Create a vertical box to hold the UI elements (search bar, buttons, web view, loading bar)
        let vbox = Box::new(gtk::Orientation::Vertical, 5);
        
        // Create a horizontal box to hold the buttons and the URL entry
        let hbox = Box::new(gtk::Orientation::Horizontal, 5);

        // Create navigation buttons
        let back_button = Button::with_label("Back");
        let forward_button = Button::with_label("Forward");
        let refresh_button = Button::with_label("Refresh");

        // Create URL entry (the search bar)
        let url_entry = Entry::new();
        url_entry.set_placeholder_text(Some("Enter URL or Search Query"));
        
        // Add the buttons to the left of the search bar
        hbox.pack_start(&back_button, false, false, 0);
        hbox.pack_start(&forward_button, false, false, 0);
        hbox.pack_start(&refresh_button, false, false, 0);
        hbox.pack_start(&url_entry, true, true, 0);
        
        // Pack the horizontal box into the vertical box
        vbox.pack_start(&hbox, false, false, 0);

        // Create a WebView
        let web_context = WebContext::default().expect("Failed to create WebContext");
        let web_view = WebView::with_context(&web_context);
        let scrolled_window = ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
        scrolled_window.add(&web_view);
        vbox.pack_start(&scrolled_window, true, true, 0);

        // Create a progress bar for page loading
        let progress_bar = ProgressBar::new();
        vbox.pack_start(&progress_bar, false, false, 0);

        // Add the vertical box to the window
        window.add(&vbox);

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

        // Handle URL input and searching
        url_entry.connect_activate(clone!(@strong web_view, @strong url_entry => move |_| {
            let mut input = url_entry.text();
            
            // Define a list of common domain extensions (as a HashSet for fast lookup)
            let domain_extensions: HashSet<&str> = vec![".com", ".org", ".net", ".edu", ".gov", ".io", ".dev", ".ai", ".co"]
                .into_iter().collect();
            
            // Regular expression for basic URL validation
            let url_regex = Regex::new(r"^(https?://)?([a-zA-Z0-9.-]+\.[a-zA-Z]{2,})(/.*)?$").unwrap();
            
            // Check if input matches the URL regex
            let is_valid_url = url_regex.is_match(&input);
            
            if is_valid_url {
                // If input is a valid URL but doesn't start with "http://" or "https://", add "http://"
                if !input.starts_with("http://") && !input.starts_with("https://") {
                    input = format!("http://{}", input).into();  // Convert String to GString
                }
            } else if input.starts_with('/') {
                // If input starts with "/", treat it as a search query
                let query = &input[1..]; // Remove the "/" character
                input = format!("https://www.google.com/search?q={}", query).into();  // Convert String to GString
            } else if domain_extensions.iter().any(|ext| input.ends_with(ext)) {
                // If input has a known domain extension, treat it as a URL and prepend "http://"
                if !input.starts_with("http://") && !input.starts_with("https://") {
                    input = format!("http://{}", input).into();  // Convert String to GString
                }
            } else {
                // Otherwise, treat it as a Google search query
                input = format!("https://www.google.com/search?q={}", input).into();  // Convert String to GString
            }

            // Load the constructed URL or search query
            web_view.load_uri(&input);
        }));

        // Update the URL in the search bar when navigating to new pages
        web_view.connect_load_changed(clone!(@strong url_entry, @strong web_view => move |_, _| {
            if let Some(uri) = web_view.uri() {
                url_entry.set_text(&uri);
            }
        }));

        // Update the progress bar during page load
        web_view.connect_load_changed(clone!(@strong progress_bar, @strong web_view => move |_, event| {
            match event {
                LoadEvent::Started => {
                    progress_bar.set_fraction(0.0); // Reset the progress bar at the start of a new load
                    progress_bar.show();
                },
                LoadEvent::Committed => {
                    progress_bar.set_fraction(0.5); // Halfway when page resources are committed
                },
                LoadEvent::Finished => {
                    progress_bar.set_fraction(1.0); // Full progress when the page load is complete
                    progress_bar.hide(); // Hide the progress bar after loading is finished
                },
                _ => {}
            }
        }));

        // Load an initial page (like Google) on startup
        web_view.load_uri("https://www.google.com");

        // Show all components
        window.show_all();
    });

    // Run the application
    app.run();
}
