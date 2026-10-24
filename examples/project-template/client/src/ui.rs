// Include the Rust code generated from ui/app-window.slint, including AppWindow
// and the getters/setters for its properties.
slint::include_modules!();

/// Append a message to the conversation text area in the UI
pub(crate) fn append_to_conversation(window: &AppWindow, message: &str) {
    // This Slint-generated getter function reads the `conversation` property
    // declared in Slint.
    let current = window.get_conversation();
    
    // Build the full updated text, separating messages with a newline.
    let conversation = if current.is_empty() {
        message.to_owned() // copy the message into new owned String
    } else {
        format!("{current}\n{message}")
    };
    // Convert the Rust String into Slint's SharedString (using .into()) and update
    // the Slint's "conversation" property.
    window.set_conversation(conversation.into());
}
