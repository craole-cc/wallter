use super::{Monitor, Path, PathType, Slideshow};
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    fs::{read_to_string, write},
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub path: Path,
    pub monitor: Vec<Monitor>,
    pub slideshow: Slideshow,
    // pub api: Api,
}

impl Config {
    /// Initializes the config: creates all dirs, creates config file if missing, and loads or saves config.
    pub fn init(path_config: &mut Path) -> Result<Self> {
        //{ Ensure all necessary paths exist }
        path_config.create_all()?;

        //{ Try to load config from file, or fall back to default and save it }
        let mut config = match Self::load(path_config) {
            Ok(cfg) => cfg,
            Err(_) => {
                let default_cfg = Self::default();
                default_cfg.save(path_config)?;
                default_cfg
            }
        };

        //{ Always enumerate current monitors and update the config }
        let detected_monitors = Monitor::enumerate();
        config.monitor = detected_monitors;
        config.save(path_config)?;

        //{ Return the initialized config }
        Ok(config)
    }

    /// Loads the configuration from the config file if it exists, otherwise returns default.
    pub fn load(path_config: &Path) -> Result<Self> {
        //{ Retrieve the contents of the config file }
        let content = read_to_string(&path_config.config_file)?;

        //{ Parse the contents of the config file based on the defined format }
        match path_config.config_type {
            PathType::Toml => {
                toml::from_str(&content).map_err(|e| Error::ConfigError(e.to_string()))
            }
            PathType::Json => {
                serde_json::from_str(&content).map_err(|e| Error::ConfigError(e.to_string()))
            }
        }
    }

    /// Saves the configuration to the config file
    pub fn save(&self, path_config: &Path) -> Result<()> {
        //{ Ensure all directories exist }
        path_config.create_all()?;

        //{ Serialize to appropriate format }
        let contents = match path_config.config_type {
            PathType::Toml => {
                toml::to_string(self).map_err(|e| Error::ConfigError(e.to_string()))?
            }
            PathType::Json => {
                serde_json::to_string_pretty(self).map_err(|e| Error::ConfigError(e.to_string()))?
            }
        };

        //{ Update the configuration file }
        write(&path_config.config_file, contents)?;
        Ok(())
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Configuration:")?;

        //~@ Paths section
        writeln!(f, "  Paths:\n{}", self.path)?;

        //~@ Monitors section
        if self.monitor.is_empty() {
            writeln!(f, "  Monitors: No monitors detected")?;
        } else {
            writeln!(f, "  Monitors:")?;
            for monitor in &self.monitor {
                writeln!(f, "{}", monitor)?;
            }
        }

        //~@ Slideshow section
        if self.slideshow.sources.is_empty() {
            writeln!(f, "  Slideshow: No wallpaper sources configured")?;
        } else {
            writeln!(f, "  Slideshow:")?;
            writeln!(f, "{}", self.slideshow)?;
        }

        Ok(())
    }
}

/// Helper function to initialize the configuration with default path config.
pub fn init() -> crate::Result<Config> {
    let mut path_config = Path::default();
    Config::init(&mut path_config)
}
