use quickfix::{SessionId, QuickFixError, LogCallback};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};

pub struct CustomLogger {
    raw_log_file: Arc<Mutex<File>>,
    human_readable_log_file: Arc<Mutex<File>>,
}

impl CustomLogger {
    pub fn new(raw_log_path: &str, human_readable_log_path: &str) -> Result<Self, QuickFixError> {
        let raw_log_file = OpenOptions::new().append(true).create(true).open(raw_log_path)
            .map_err(|e| QuickFixError::invalid_argument(e.to_string()))?;
        let human_readable_log_file = OpenOptions::new().append(true).create(true).open(human_readable_log_path)
            .map_err(|e| QuickFixError::invalid_argument(e.to_string()))?;
        
        Ok(Self {
            raw_log_file: Arc::new(Mutex::new(raw_log_file)),
            human_readable_log_file: Arc::new(Mutex::new(human_readable_log_file)),
        })
    }

    fn log_message(&self, message: &str, human_readable_message: &str) -> Result<(), QuickFixError> {
        let mut raw_log_file = self.raw_log_file.lock().unwrap();
        writeln!(raw_log_file, "{}", message)
            .map_err(|e| QuickFixError::invalid_argument(e.to_string()))?;

        let mut human_readable_log_file = self.human_readable_log_file.lock().unwrap();
        writeln!(human_readable_log_file, "{}", human_readable_message)
            .map_err(|e| QuickFixError::invalid_argument(e.to_string()))?;

        Ok(())
    }
}

impl LogCallback for CustomLogger {
    fn on_incoming(&self, _session_id: Option<&SessionId>, msg: &str) {
        let human_readable_message = format!("Incoming: {}", msg.replace('\x01', "|"));
        let _ = self.log_message(msg, &human_readable_message);
    }

    fn on_outgoing(&self, _session_id: Option<&SessionId>, msg: &str) {
        let human_readable_message = format!("Outgoing: {}", msg.replace('\x01', "|"));
        let _ = self.log_message(msg, &human_readable_message);
    }

    fn on_event(&self, _session_id: Option<&SessionId>, msg: &str) {
        let human_readable_message = format!("Event: {}", msg.replace('\x01', "|"));
        let _ = self.log_message(msg, &human_readable_message);
    }
}
