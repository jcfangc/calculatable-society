use super::event_type::EventType;
use std::collections::HashMap;

// 注册表，用于存储事件类型到监听者的映射
pub struct EventRegistry {
    listeners: HashMap<EventType, Vec<Box<dyn FnMut()>>>,
}

impl EventRegistry {
    pub fn new() -> Self {
        EventRegistry {
            listeners: HashMap::new(),
        }
    }

    // 订阅事件
    pub fn subscribe(&mut self, event_type: EventType, listener: Box<dyn FnMut()>) {
        self.listeners
            .entry(event_type)
            .or_insert(Vec::new())
            .push(listener);
    }

    // 发布事件
    pub fn trigger(&mut self, event_type: &EventType) {
        if let Some(listeners) = self.listeners.get_mut(event_type) {
            for listener in listeners.iter_mut() {
                listener();
            }
        }
    }
}
