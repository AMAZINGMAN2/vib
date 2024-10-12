
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, Box, ScrolledWindow, Orientation, Align, CssProvider, StyleContext};
use webkit2gtk::{WebView, WebContext, WebViewExt};
use glib::clone;
use std::collections::HashSet;
use regex::Regex;
use gdk::EventMask;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let app = Application::new(Some("com.hashim.vib"), Default::default());

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_default_size(800, 600);
        window.set_decorated(false);

        // Enable resizing and moving
        window.add_events(EventMask::BUTTON_PRESS_MASK | EventMask::BUTTON_RELEASE_MASK | EventMask::BUTTON1_MOTION_MASK);

        // Apply CSS for curved edges and button styling
        let css = CssProvider::new();
        css.load_from_data(b"
            window {
                border-radius: 15px;
            }
            button {
                border: none;
                background: none;
                padding: 5px;
            }
            entry {
                border-radius: 20px;
                padding: 5px 10px;
            }
        ").expect("Failed to load CSS");
        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Error initializing gtk css provider."),
            &css,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Main vertical box
        let main_vbox = Box::new(Orientation::Vertical, 0);
        main_vbox.set_vexpand(true); // Allow vertical expansion
        main_vbox.set_hexpand(true); // Allow horizontal expansion

        // Top bar for buttons and dragging
        let top_bar = Box::new(Orientation::Horizontal, 5);
        top_bar.set_halign(Align::Fill);

        // Create navigation buttons
        let minimize_button = Button::with_label("—");
        let maximize_button = Button::with_label("🗖");
        let close_button = Button::with_label("X");

        // Connect button signals
        minimize_button.connect_clicked(clone!(@strong window => move |_| {
            window.iconify();
        }));

        maximize_button.connect_clicked(clone!(@strong window => move |_| {
            if window.is_maximized() {
                window.unmaximize();
            } else {
                window.maximize();
            }
        }));

        close_button.connect_clicked(clone!(@strong window => move |_| {
            window.close();
        }));

        // Pack buttons into the top bar
        top_bar.pack_end(&close_button, false, false, 0);
        top_bar.pack_end(&maximize_button, false, false, 0);
        top_bar.pack_end(&minimize_button, false, false, 0);

        // Add top bar to main vertical box
        main_vbox.pack_start(&top_bar, false, false, 5);

        // Create URL entry (the search bar)
        let url_entry = Entry::new();
        url_entry.set_placeholder_text(Some("Enter URL or Search Query"));
        main_vbox.pack_start(&url_entry, false, false, 5);

        // Create a WebView
        let web_context = WebContext::default().expect("Failed to create WebContext");
        let web_view = WebView::with_context(&web_context);
        
        // Create a ScrolledWindow and allow it to expand
        let scrolled_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        scrolled_window.set_vexpand(true); // Allow vertical expansion
        scrolled_window.set_hexpand(true); // Allow horizontal expansion
        scrolled_window.add(&web_view);
        main_vbox.pack_start(&scrolled_window, true, true, 0);

        // Handle URL input and searching
        url_entry.connect_activate(clone!(@strong web_view, @strong url_entry => move |entry| {
            let mut input = entry.text();

            let domain_extensions: HashSet<&str> = vec![".com", ".org", ".net", ".edu", ".gov", ".io", ".dev", ".ai", ".co"]
                .into_iter().collect();

            let url_regex = Regex::new(r"^(https?://)?([a-zA-Z0-9.-]+\.[a-zA-Z]{2,})(/.*)?$").unwrap();

            let is_valid_url = url_regex.is_match(&input);

            if is_valid_url {
                if !input.starts_with("http://") && !input.starts_with("https://") {
                    input = format!("http://{}", input).into();
                }
            } else if input.starts_with('/') {
                let query = &input[1..];
                input = format!("https://www.google.com/search?q={}", query).into();
            } else if domain_extensions.iter().any(|ext| input.ends_with(ext)) {
                if !input.starts_with("http://") && !input.starts_with("https://") {
                    input = format!("http://{}", input).into();
                }
            } else {
                input = format!("https://www.google.com/search?q={}", input).into();
            }

            web_view.load_uri(&input);
            
            // Move cursor to the left after pressing enter
            entry.set_position(0);
        }));

        // Update the URL in the search bar when navigating to new pages
        web_view.connect_load_changed(clone!(@strong url_entry, @strong web_view => move |_, _| {
            if let Some(uri) = web_view.uri() {
                url_entry.set_text(&uri);
                url_entry.set_position(0); // Set cursor to the left
            }
        }));

        // Load an initial page (like Google) on startup
        web_view.load_uri("https://www.google.com");

        // Enable window dragging
        let drag_pos = Rc::new(RefCell::new((0, 0)));
        let is_dragging = Rc::new(RefCell::new(false));

        top_bar.connect_button_press_event(clone!(@strong window, @strong drag_pos, @strong is_dragging => move |_, event| {
            if event.button() == 1 { // Left mouse button
                let (x, y) = event.root();
                drag_pos.replace((x as i32, y as i32));
                is_dragging.replace(true);
            }
            false.into()
        }));

        top_bar.connect_button_release_event(clone!(@strong is_dragging => move |_, event| {
            if event.button() == 1 { // Left mouse button
                is_dragging.replace(false);
            }
            false.into()
        }));

        top_bar.connect_motion_notify_event(clone!(@strong window, @strong drag_pos, @strong is_dragging => move |_, event| {
            if *is_dragging.borrow() {
                let (x, y) = event.root();
                let (old_x, old_y) = *drag_pos.borrow();
                let dx = x as i32 - old_x;
                let dy = y as i32 - old_y;
                let (wx, wy) = window.position();
                window.move_(wx + dx, wy + dy);
                drag_pos.replace((x as i32, y as i32));
            }
            false.into()
        }));

        // Show all components
        window.add(&main_vbox);
        window.show_all();
    });

    app.run();
}
