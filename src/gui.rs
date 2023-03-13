use gtk::prelude::*;

pub fn run() {
    // Initialise GTK
    if let Err(err) = gtk::init() {
        println!("Failed to initialize GTK: {}", err);
        return;
    }

    // Create the main window
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Chat GPT");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(500, 500);

    // Create a vertical box to hold the chat messages
    let messages_box = gtk::Box::new(gtk::Orientation::Vertical, 5);

    // Create a scrolled window to hold the messages box
    let scrolled_window = gtk::ScrolledWindow::new(None, None);
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_window.add(&messages_box);

    // Create a text field to input messages
    let input_entry = gtk::Entry::new();
    input_entry.set_placeholder_text("Type a message...");

    // Create a button to send messages
    let send_button = gtk::Button::new_with_label("Send");

    // Create a horizontal box to hold the input field and send button
    let input_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    input_box.pack_start(&input_entry, true, true, 0);
    input_box.pack_start(&send_button, false, false, 0);

    // Create a vertical box to hold the scrolled window and input box
    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
    main_box.pack_start(&scrolled_window, true, true, 0);
    main_box.pack_start(&input_box, false, false, 0);

    // Add the main box to the window
    window.add(&main_box);

    // Show the window and run the GTK main loop
    window.show_all();
    gtk::main();
}
