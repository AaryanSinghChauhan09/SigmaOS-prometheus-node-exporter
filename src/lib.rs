// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-prometheus-node-exporter: prometheus node exporter component model.
//!
//! Real implementation: exporters, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exporter {
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
pub enum PrometheusNodeExporterError {
    EmptyExporterName,
    DuplicateExporter(String),
    EmptyEventSubject,
}

impl fmt::Display for PrometheusNodeExporterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyExporterName => write!(f, "exporter name must not be empty"),
            Self::DuplicateExporter(id) => write!(f, "duplicate exporter {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for PrometheusNodeExporterError {}

pub struct PrometheusNodeExporter {
    pub name: String,
    exporters: BTreeMap<String, Exporter>,
    events: BTreeMap<String, Event>,
}

impl PrometheusNodeExporter {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            exporters: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_exporter(&mut self, item: Exporter) -> Result<(), PrometheusNodeExporterError> {
        if item.name.trim().is_empty() {
            return Err(PrometheusNodeExporterError::EmptyExporterName);
        }
        let id = item.id.clone();
        if self.exporters.contains_key(&id) {
            return Err(PrometheusNodeExporterError::DuplicateExporter(id.clone()));
        }
        self.exporters.insert(id, item);
        Ok(())
    }

    pub fn exporter_count(&self) -> usize {
        self.exporters.len()
    }

    pub fn get_exporter(&self, id: &str) -> Option<&Exporter> {
        self.exporters.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), PrometheusNodeExporterError> {
        if event.subject.trim().is_empty() {
            return Err(PrometheusNodeExporterError::EmptyEventSubject);
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

    fn sample_exporter(id: &str, name: &str) -> Exporter {
        Exporter {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = PrometheusNodeExporter::new("PrometheusNodeExporter");
        assert_eq!(m.name, "PrometheusNodeExporter");
        assert_eq!(m.exporter_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_exporter() {
        let mut m = PrometheusNodeExporter::new("PrometheusNodeExporter");
        m.add_exporter(sample_exporter("a1", "Alpha")).unwrap();
        assert_eq!(m.exporter_count(), 1);
        assert!(m.get_exporter("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = PrometheusNodeExporter::new("PrometheusNodeExporter");
        assert!(m.add_exporter(sample_exporter("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = PrometheusNodeExporter::new("PrometheusNodeExporter");
        m.add_exporter(sample_exporter("x", "One")).unwrap();
        assert!(m.add_exporter(sample_exporter("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = PrometheusNodeExporter::new("PrometheusNodeExporter");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
