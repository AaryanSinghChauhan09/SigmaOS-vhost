// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-vhost: vhost component model.
//!
//! Real implementation: vhost_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VhostItem {
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
pub enum VhostError {
    EmptyVhostItemName,
    DuplicateVhostItem(String),
    EmptyEventSubject,
}

impl fmt::Display for VhostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyVhostItemName => write!(f, "vhost_item name must not be empty"),
            Self::DuplicateVhostItem(id) => write!(f, "duplicate vhost_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for VhostError {}

pub struct Vhost {
    pub name: String,
    vhost_items: BTreeMap<String, VhostItem>,
    events: BTreeMap<String, Event>,
}

impl Vhost {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            vhost_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_vhost_item(&mut self, item: VhostItem) -> Result<(), VhostError> {
        if item.name.trim().is_empty() {
            return Err(VhostError::EmptyVhostItemName);
        }
        let id = item.id.clone();
        if self.vhost_items.contains_key(&id) {
            return Err(VhostError::DuplicateVhostItem(id.clone()));
        }
        self.vhost_items.insert(id, item);
        Ok(())
    }

    pub fn vhost_item_count(&self) -> usize {
        self.vhost_items.len()
    }

    pub fn get_vhost_item(&self, id: &str) -> Option<&VhostItem> {
        self.vhost_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), VhostError> {
        if event.subject.trim().is_empty() {
            return Err(VhostError::EmptyEventSubject);
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

    fn sample_vhost_item(id: &str, name: &str) -> VhostItem {
        VhostItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Vhost::new("Vhost");
        assert_eq!(m.name, "Vhost");
        assert_eq!(m.vhost_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_vhost_item() {
        let mut m = Vhost::new("Vhost");
        m.add_vhost_item(sample_vhost_item("a1", "Alpha")).unwrap();
        assert_eq!(m.vhost_item_count(), 1);
        assert!(m.get_vhost_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Vhost::new("Vhost");
        assert!(m.add_vhost_item(sample_vhost_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Vhost::new("Vhost");
        m.add_vhost_item(sample_vhost_item("x", "One")).unwrap();
        assert!(m.add_vhost_item(sample_vhost_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Vhost::new("Vhost");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
