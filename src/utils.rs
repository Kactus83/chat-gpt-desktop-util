use gtk::prelude::*;

pub fn add_message_to_box(messages_box: &gtk::Box, message: &str) {
    let label = gtk::Label::new(Some(message));
    messages_box.pack_start(&label, false, false, 0);
    label.show_all();
}

pub fn clear_input_entry(input_entry: &gtk::Entry) {
    input_entry.set_text("");
    input_entry.grab_focus();
}

pub fn get_text_from_entry(entry: &gtk::Entry) -> String {
    entry.text().trim().to_owned()
}
