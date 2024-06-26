use quickfix::{LogCallback, QuickFixError, SessionId};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};

// Struct to handle custom logging functionality
pub struct CustomLogger {
    raw_log_file: Arc<Mutex<File>>,
    human_readable_log_file: Arc<Mutex<File>>,
}

impl CustomLogger {
    // Creates a new CustomLogger with specified log file paths
    pub fn new(raw_log_path: &str, human_readable_log_path: &str) -> Result<Self, QuickFixError> {
        let raw_log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(raw_log_path)
            .map_err(|e| QuickFixError::invalid_argument(e.to_string()))?;
        let human_readable_log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(human_readable_log_path)
            .map_err(|e| QuickFixError::invalid_argument(e.to_string()))?;

        Ok(Self {
            raw_log_file: Arc::new(Mutex::new(raw_log_file)),
            human_readable_log_file: Arc::new(Mutex::new(human_readable_log_file)),
        })
    }

    // Logs raw FIX messages to the raw log file
    fn log_raw_message(&self, message: &str) -> Result<(), QuickFixError> {
        let mut raw_log_file = self.raw_log_file.lock().unwrap();
        writeln!(raw_log_file, "{}", message)
            .map_err(|e| QuickFixError::invalid_argument(e.to_string()))?;
        Ok(())
    }

    // Logs human-readable messages to the human-readable log file
    fn log_human_readable_message(
        &self,
        human_readable_message: &str,
    ) -> Result<(), QuickFixError> {
        let mut human_readable_log_file = self.human_readable_log_file.lock().unwrap();
        writeln!(human_readable_log_file, "{}", human_readable_message)
            .map_err(|e| QuickFixError::invalid_argument(e.to_string()))?;
        Ok(())
    }
}

// Implement the LogCallback trait for CustomLogger
impl LogCallback for CustomLogger {
    // Called when an incoming message is received
    fn on_incoming(&self, _session_id: Option<&SessionId>, msg: &str) {
        let human_readable_message = format!("Incoming: {}", msg.replace('\x01', "|"));
        let _ = self.log_raw_message(msg);
        let _ = self.log_human_readable_message(&human_readable_message);
    }

    // Called when an outgoing message is sent
    fn on_outgoing(&self, _session_id: Option<&SessionId>, msg: &str) {
        let human_readable_message = format!("Outgoing: {}", msg.replace('\x01', "|"));
        let _ = self.log_raw_message(msg);
        let _ = self.log_human_readable_message(&human_readable_message);
    }

    // Called when an event occurs
    fn on_event(&self, _session_id: Option<&SessionId>, msg: &str) {
        let human_readable_message = format!("Event: {}", msg.replace('\x01', "|"));
        let _ = self.log_human_readable_message(&human_readable_message);
    }
}
