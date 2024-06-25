use quickfix::Message;

pub struct MessageHandler;

impl MessageHandler {
    pub fn new() -> Self {
        MessageHandler
    }

    pub fn on_message(&self, message: &Message) {
        // Processar a mensagem
        println!("Received message: {:?}", message);
    }
}
