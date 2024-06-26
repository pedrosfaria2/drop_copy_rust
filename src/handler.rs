use quickfix::Message;

// Struct to handle messages
pub struct MessageHandler;

impl MessageHandler {
    // Creates a new instance of MessageHandler
    pub fn new() -> Self {
        MessageHandler
    }

    // Handles incoming messages
    pub fn on_message(&self, message: &Message) {
        // Process the message
        println!("Received message: {:?}", message);
    }
}
