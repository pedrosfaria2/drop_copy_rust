use quickfix::{
    ApplicationCallback, Message, SessionSettings, SessionId, QuickFixError,
    FileMessageStoreFactory, Application, SocketInitiator, ConnectionHandler,
    MsgToAppError, MsgFromAdminError, MsgFromAppError, LogFactory, FieldMap,
    send_to_target,
};
use crate::config::AppConfig;
use crate::handler::MessageHandler;
use crate::custom_logger::CustomLogger;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::io::Read;
use std::thread;

pub fn start_sessions(config: AppConfig, handler: Arc<MessageHandler>) -> Result<(), QuickFixError> {
    let mut handles = vec![];

    for settings_file in config.settings {
        let handler_clone = Arc::clone(&handler);
        let handle = thread::spawn(move || {
            if let Err(e) = start_single_session(settings_file, handler_clone) {
                eprintln!("Error starting session: {}", e);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Failed to join thread");
    }

    Ok(())
}

fn start_single_session(settings_file: String, handler: Arc<MessageHandler>) -> Result<(), QuickFixError> {
    let settings = SessionSettings::try_from_path(&settings_file)?;
    let store_factory = FileMessageStoreFactory::try_new(&settings)?;
    let logger = CustomLogger::new("logs/raw.log", "logs/human_readable.log")?;
    let log_factory = LogFactory::try_new(&logger)?;
    let app_instance = DropCopyApplication {
        handler,
        message_store: Arc::new(Mutex::new(HashMap::new())),
    };
    let app = Application::try_new(&app_instance)?;

    let mut initiator = SocketInitiator::try_new(&settings, &app, &store_factory, &log_factory)?;
    initiator.start()?;

    println!(">> App running for session {:?}, press 'q' to quit", settings_file);
    let mut stdin = std::io::stdin().lock();
    let mut stdin_buf = [0];
    loop {
        let _ = stdin.read_exact(&mut stdin_buf);
        if stdin_buf[0] == b'q' {
            break;
        }
    }

    initiator.stop()?;
    Ok(())
}

struct DropCopyApplication {
    handler: Arc<MessageHandler>,
    message_store: Arc<Mutex<HashMap<u64, Message>>>,
}

impl ApplicationCallback for DropCopyApplication {
    fn on_create(&self, session_id: &SessionId) {
        println!("Session created: {:?}", session_id);
    }

    fn on_logon(&self, session_id: &SessionId) {
        println!("Logon: {:?}", session_id);
    }

    fn on_logout(&self, session_id: &SessionId) {
        println!("Logout: {:?}", session_id);
    }

    fn on_msg_to_admin(&self, msg: &mut Message, _session_id: &SessionId) {
        println!("Message to admin: {:?}", msg);
    }

    fn on_msg_to_app(&self, msg: &mut Message, _session_id: &SessionId) -> Result<(), MsgToAppError> {
        println!("Message to app: {:?}", msg);
        Ok(())
    }

    fn on_msg_from_admin(&self, msg: &Message, session_id: &SessionId) -> Result<(), MsgFromAdminError> {
        println!("Message from admin: {:?}", msg);
        self.store_message(msg);
        self.handler.on_message(msg);
        self.handle_message(msg, session_id);
        Ok(())
    }

    fn on_msg_from_app(&self, msg: &Message, session_id: &SessionId) -> Result<(), MsgFromAppError> {
        println!("Message from app: {:?}", msg);
        self.store_message(msg);
        self.handler.on_message(msg);
        self.handle_message(msg, session_id);
        Ok(())
    }
}

impl DropCopyApplication {
    fn store_message(&self, msg: &Message) {
        let seq_no: u64 = msg.get_field(34).unwrap().parse().unwrap();
        let mut store = self.message_store.lock().unwrap();
        store.insert(seq_no, msg.clone());
    }

    fn handle_message(&self, msg: &Message, session_id: &SessionId) {
        match msg.get_field(35).as_deref() {
            Some("2") => {
                self.handle_resend_request(msg, session_id);
            }
            _ => {
                println!("Received message: {:?}", msg);
            }
        }
    }

    fn handle_resend_request(&self, msg: &Message, session_id: &SessionId) {
        let begin_seq_no: u64 = msg.get_field(7).unwrap().parse().unwrap();
        let end_seq_no: u64 = msg.get_field(16).unwrap().parse().unwrap();

        println!("Received Resend Request from {} to {}", begin_seq_no, end_seq_no);

        for seq_no in begin_seq_no..=end_seq_no {
            if let Some(stored_message) = self.get_message(seq_no) {
                let mut resend_message = stored_message.clone();
                resend_message.set_field(43, "Y").unwrap();
                send_to_target(resend_message, session_id).unwrap();
            }
        }
    }

    fn get_message(&self, seq_no: u64) -> Option<Message> {
        let store = self.message_store.lock().unwrap();
        store.get(&seq_no).cloned()
    }
}
