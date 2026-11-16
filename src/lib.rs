// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-lamp: lamp component model.
//!
//! Real implementation: lamp_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LampItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LampError {
    EmptyLampItemName,
    DuplicateLampItem(String),
    EmptyEventSubject,
}

impl fmt::Display for LampError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLampItemName => write!(f, "lamp_item name must not be empty"),
            Self::DuplicateLampItem(id) => write!(f, "duplicate lamp_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LampError {}

pub struct Lamp {
    pub name: String,
    lamp_items: BTreeMap<String, LampItem>,
    events: BTreeMap<String, Event>,
}

impl Lamp {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            lamp_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_lamp_item(&mut self, item: LampItem) -> Result<(), LampError> {
        if item.name.trim().is_empty() {
            return Err(LampError::EmptyLampItemName);
        }
        let id = item.id.clone();
        if self.lamp_items.contains_key(&id) {
            return Err(LampError::DuplicateLampItem(id.clone()));
        }
        self.lamp_items.insert(id, item);
        Ok(())
    }

    pub fn lamp_item_count(&self) -> usize {
        self.lamp_items.len()
    }

    pub fn get_lamp_item(&self, id: &str) -> Option<&LampItem> {
        self.lamp_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LampError> {
        if event.subject.trim().is_empty() {
            return Err(LampError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_lamp_item(id: &str, name: &str) -> LampItem {
        LampItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Lamp::new("Lamp");
        assert_eq!(m.name, "Lamp");
        assert_eq!(m.lamp_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_lamp_item() {
        let mut m = Lamp::new("Lamp");
        m.add_lamp_item(sample_lamp_item("a1", "Alpha")).unwrap();
        assert_eq!(m.lamp_item_count(), 1);
        assert!(m.get_lamp_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Lamp::new("Lamp");
        assert!(m.add_lamp_item(sample_lamp_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Lamp::new("Lamp");
        m.add_lamp_item(sample_lamp_item("x", "One")).unwrap();
        assert!(m.add_lamp_item(sample_lamp_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Lamp::new("Lamp");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
