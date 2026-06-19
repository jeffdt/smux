use crate::model::{Session, Window};

pub const FMT: &str = "#{session_name}\x1f#{session_activity}\x1f#{session_created}\x1f#{session_attached}\x1f#{window_index}\x1f#{window_name}\x1f#{window_active}";

pub fn parse_windows(raw: &str) -> Vec<Session> {
    let mut sessions: Vec<Session> = Vec::new();
    for line in raw.lines() {
        if line.is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split('\u{1f}').collect();
        if f.len() != 7 {
            continue;
        }
        let name = f[0].to_string();
        let window = Window {
            index: f[4].parse().unwrap_or(0),
            name: f[5].to_string(),
            active: f[6] == "1",
        };
        if let Some(s) = sessions.iter_mut().find(|s| s.name == name) {
            s.windows.push(window);
        } else {
            sessions.push(Session {
                name,
                activity: f[1].parse().unwrap_or(0),
                created: f[2].parse().unwrap_or(0),
                attached: f[3] == "1",
                windows: vec![window],
            });
        }
    }
    sessions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_two_sessions_grouping_windows_in_order() {
        // Fields separated by the unit separator \x1f; one line per window.
        let raw = "\
work\u{1f}100\u{1f}10\u{1f}1\u{1f}0\u{1f}editor\u{1f}1
work\u{1f}100\u{1f}10\u{1f}1\u{1f}1\u{1f}my logs\u{1f}0
scratch\u{1f}50\u{1f}5\u{1f}0\u{1f}0\u{1f}shell\u{1f}1
";
        let sessions = parse_windows(raw);
        assert_eq!(
            sessions,
            vec![
                Session {
                    name: "work".into(),
                    activity: 100,
                    created: 10,
                    attached: true,
                    windows: vec![
                        Window { index: 0, name: "editor".into(), active: true },
                        Window { index: 1, name: "my logs".into(), active: false },
                    ],
                },
                Session {
                    name: "scratch".into(),
                    activity: 50,
                    created: 5,
                    attached: false,
                    windows: vec![Window { index: 0, name: "shell".into(), active: true }],
                },
            ]
        );
    }
}
