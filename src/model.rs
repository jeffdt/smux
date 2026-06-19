#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum SortKey {
    #[default]
    Activity,
    Created,
}

#[allow(dead_code)]
impl SortKey {
    pub fn from_config_str(s: &str) -> SortKey {
        match s {
            "created" => SortKey::Created,
            _ => SortKey::Activity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Window {
    pub index: u32,
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Session {
    pub name: String,
    pub activity: i64,
    pub created: i64,
    pub attached: bool,
    pub windows: Vec<Window>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Action {
    SwitchSession(String),
    SwitchWindow(String, u32),
}

use crate::store::Config;
use std::collections::HashSet;

#[allow(dead_code)]
pub struct PickerState {
    all: Vec<Session>,
    pub pinned: Vec<String>,
    pub sort: SortKey,
    expanded: HashSet<String>,
    pub cursor: usize,
    pub dirty: bool,
}

fn sort_value(s: &Session, key: SortKey) -> i64 {
    match key {
        SortKey::Activity => s.activity,
        SortKey::Created => s.created,
    }
}

impl PickerState {
    pub fn build(sessions: Vec<Session>, config: &Config) -> PickerState {
        PickerState {
            all: sessions,
            pinned: config.pinned.clone(),
            sort: config.sort,
            expanded: HashSet::new(),
            cursor: 0,
            dirty: false,
        }
    }

    pub fn is_pinned(&self, name: &str) -> bool {
        self.pinned.iter().any(|p| p == name)
    }

    pub fn ordered(&self) -> Vec<&Session> {
        let mut out: Vec<&Session> = Vec::new();
        for name in &self.pinned {
            if let Some(s) = self.all.iter().find(|s| &s.name == name) {
                out.push(s);
            }
        }
        let mut rest: Vec<&Session> = self
            .all
            .iter()
            .filter(|s| !self.is_pinned(&s.name))
            .collect();
        rest.sort_by(|a, b| {
            sort_value(b, self.sort)
                .cmp(&sort_value(a, self.sort))
                .then(a.name.cmp(&b.name))
        });
        out.extend(rest);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::Config;

    fn s(name: &str, activity: i64, created: i64) -> Session {
        Session {
            name: name.into(),
            activity,
            created,
            attached: false,
            windows: vec![Window { index: 0, name: "w".into(), active: true }],
        }
    }

    #[test]
    fn sort_key_parses_with_default_fallback() {
        assert_eq!(SortKey::from_config_str("created"), SortKey::Created);
        assert_eq!(SortKey::from_config_str("activity"), SortKey::Activity);
        assert_eq!(SortKey::from_config_str("garbage"), SortKey::Activity);
        assert_eq!(SortKey::default(), SortKey::Activity);
    }

    #[test]
    fn ordered_puts_pinned_first_then_unpinned_by_activity() {
        let sessions = vec![s("a", 10, 1), s("b", 30, 2), s("c", 20, 3)];
        let cfg = Config { pinned: vec!["c".into()], sort: SortKey::Activity };
        let state = PickerState::build(sessions, &cfg);
        let names: Vec<&str> = state.ordered().iter().map(|s| s.name.as_str()).collect();
        // c is pinned (first); then b (activity 30) before a (activity 10)
        assert_eq!(names, vec!["c", "b", "a"]);
        assert!(state.is_pinned("c"));
        assert!(!state.is_pinned("a"));
    }

    #[test]
    fn ordered_unpinned_by_created_when_configured() {
        let sessions = vec![s("a", 10, 100), s("b", 30, 50)];
        let cfg = Config { pinned: vec![], sort: SortKey::Created };
        let state = PickerState::build(sessions, &cfg);
        let names: Vec<&str> = state.ordered().iter().map(|s| s.name.as_str()).collect();
        // created desc: a (100) before b (50)
        assert_eq!(names, vec!["a", "b"]);
    }
}
