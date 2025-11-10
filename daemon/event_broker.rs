use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

pub struct EventBroker {
    events: Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>,
}

impl EventBroker {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_event(&self, event_type: &str) -> broadcast::Receiver<String> {
        let mut events = self.events.lock().unwrap();
        let (tx, rx) = broadcast::channel(100);
        events.insert(event_type.to_string(), tx);
        rx
    }

    pub fn send_event(&self, event_type: &str, message: &str) {
        let events = self.events.lock().unwrap();
        if let Some(tx) = events.get(event_type) {
            let _ = tx.send(message.to_string());
        }
    }
}

pub fn start_event_broker() {
    // Placeholder
}