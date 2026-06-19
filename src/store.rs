use crate::model::SortKey;
use serde::Deserialize;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Config {
    pub pinned: Vec<String>,
    pub sort: SortKey,
}

#[derive(Deserialize, Default)]
struct RawConfig {
    #[serde(default)]
    pinned: Vec<String>,
    #[serde(default)]
    sort: Option<String>,
}

#[derive(serde::Serialize)]
struct OutConfig {
    pinned: Vec<String>,
    sort: String,
}

impl Config {
    pub fn load_from(path: &Path) -> Config {
        let raw: RawConfig = std::fs::read_to_string(path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default();
        Config {
            pinned: raw.pinned,
            sort: raw
                .sort
                .map(|s| SortKey::from_config_str(&s))
                .unwrap_or_default(),
        }
    }

    pub fn save_to(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let out = OutConfig {
            pinned: self.pinned.clone(),
            sort: match self.sort {
                SortKey::Activity => "activity".into(),
                SortKey::Created => "created".into(),
            },
        };
        let body = toml::to_string(&out).map_err(io::Error::other)?;
        std::fs::write(path, body)
    }
}

#[allow(dead_code)]
pub fn config_path() -> PathBuf {
    if let Ok(x) = std::env::var("XDG_CONFIG_HOME") {
        if !x.is_empty() {
            return PathBuf::from(x).join("smux").join("config.toml");
        }
    }
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(home).join(".config").join("smux").join("config.toml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_yields_defaults() {
        let cfg = Config::load_from(Path::new("/nonexistent/smux/config.toml"));
        assert!(cfg.pinned.is_empty());
        assert_eq!(cfg.sort, SortKey::Activity);
    }

    #[test]
    fn load_then_save_round_trips_pins_and_sort() {
        let dir = std::env::temp_dir().join(format!("smux-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.toml");
        std::fs::write(
            &path,
            "pinned = [\"pr-review\", \"my session\"]\nsort = \"created\"\n",
        )
        .unwrap();

        let cfg = Config::load_from(&path);
        assert_eq!(cfg.pinned, vec!["pr-review".to_string(), "my session".to_string()]);
        assert_eq!(cfg.sort, SortKey::Created);

        let out = dir.join("out.toml");
        cfg.save_to(&out).unwrap();
        let reloaded = Config::load_from(&out);
        assert_eq!(reloaded.pinned, cfg.pinned);
        assert_eq!(reloaded.sort, SortKey::Created);

        std::fs::remove_dir_all(&dir).ok();
    }
}
