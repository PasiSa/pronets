// Include the generated code from the Slint UI file
slint::include_modules!();

/// Append a message to the conversation text area in the UI
pub(crate) fn append_to_conversation(window: &AppWindow, message: &str) {
    let current = window.get_conversation();
    let conversation = if current.is_empty() {
        message.to_owned()
    } else {
        format!("{current}\n{message}")
    };
    window.set_conversation(conversation.into());
}
