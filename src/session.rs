use quickfix::{
    ApplicationCallback, Message, SessionSettings, SessionId, QuickFixError,
    FileMessageStoreFactory, Application, SocketInitiator, ConnectionHandler,
    MsgToAppError, MsgFromAdminError, MsgFromAppError, LogFactory,
};
use crate::config::AppConfig;
use crate::handler::MessageHandler;
use crate::custom_logger::CustomLogger;
use std::io::Read;

pub fn start_session(config: AppConfig, handler: MessageHandler) -> Result<(), QuickFixError> {
    let settings = SessionSettings::try_from_path(&config.settings)?;
    let store_factory = FileMessageStoreFactory::try_new(&settings)?;
    let logger = CustomLogger::new("logs/raw.log", "logs/human_readable.log")?;
    let log_factory = LogFactory::try_new(&logger)?;
    let app_instance = DropCopyApplication { handler };
    let app = Application::try_new(&app_instance)?;

    // Inicialize o SocketInitiator
    let mut initiator = SocketInitiator::try_new(&settings, &app, &store_factory, &log_factory)?;
    initiator.start()?;

    println!(">> App running, press 'q' to quit");
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
    handler: MessageHandler,
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
