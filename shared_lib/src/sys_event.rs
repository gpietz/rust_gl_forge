//////////////////////////////////////////////////////////////////////////////
// - SystemEvent -
//////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

pub enum SystemEvent {
    KeyPressed { key_code: u32 },
    AppQuit,
}

//////////////////////////////////////////////////////////////////////////////
// - EventHandler -
//////////////////////////////////////////////////////////////////////////////

pub trait EventHandler {
    fn handle_event(&self, event: &SystemEvent);
    fn get_id(&self) -> String;
}

//////////////////////////////////////////////////////////////////////////////
// - EventManager -
//////////////////////////////////////////////////////////////////////////////

pub struct EventManager {
    handlers: HashMap<String, Box<dyn EventHandler>>,
}

impl EventManager {
    fn new() -> Self {
        EventManager {
            handlers: HashMap::new(),
        }
    }

    fn add_handler(&mut self, handler: Box<dyn EventHandler>) {
        let id = handler.get_id();
        self.handlers.insert(id, handler);
    }

    pub fn remove_handler(&mut self, id: &str) {
        self.handlers.remove(id);
    }
    
    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }
    
    pub fn broadcast(&self, event: &SystemEvent) {
        for handler in self.handlers.values() {
            handler.handle_event(event);
        }
    }
}
