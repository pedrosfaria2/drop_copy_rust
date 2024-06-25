use quickfix::{
    ApplicationCallback, Message, SessionSettings, SessionId, QuickFixError,
    FileMessageStoreFactory, Application, SocketInitiator, ConnectionHandler,
    MsgToAppError, MsgFromAdminError, MsgFromAppError, LogFactory,
};
use crate::config::AppConfig;
use crate::handler::MessageHandler;
use crate::custom_logger::CustomLogger;
use std::io::Read;
use std::sync::Arc;
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
    let app_instance = DropCopyApplication { handler };
    let app = Application::try_new(&app_instance)?;

    // Inicialize o SocketInitiator
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

    fn on_msg_from_admin(&self, msg: &Message, _session_id: &SessionId) -> Result<(), MsgFromAdminError> {
        println!("Message from admin: {:?}", msg);
        self.handler.on_message(msg);
        Ok(())
    }

    fn on_msg_from_app(&self, msg: &Message, _session_id: &SessionId) -> Result<(), MsgFromAppError> {
        println!("Message from app: {:?}", msg);
        self.handler.on_message(msg);
        Ok(())
    }
}
