use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    fs::{File, create_dir_all},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub enum Type {
    #[default]
    Toml,
    Json,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Home directory for the config and wallpapers
    pub home_dir: PathBuf,

    /// All wallpaper downloads
    pub downloads_dir: PathBuf,

    /// User-defined wallpaper favorites
    pub favorites_dir: PathBuf,

    /// Current wallpaper for each monitor
    pub wallpaper_dir: PathBuf,

    /// The name of the configuration file
    pub config_name: String,

    /// The format type of the configuration file
    pub config_type: Type,

    /// The constructed path to the config file
    pub config_file: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let home_dir = directories::UserDirs::new()
            .expect("Could not determine home directory")
            .home_dir()
            .to_path_buf()
            .join("Pictures")
            .join(env!("CARGO_PKG_NAME"));
        let downloads_dir = home_dir.join("downloads");
        let favorites_dir = home_dir.join("favorites");
        let wallpaper_dir = home_dir.join("wallpaper");
        let config_name = "config".to_string();
        let config_type = Type::default();
        let config_file = home_dir.join(format!("{}.{}", config_name, config_type.extension()));

        Self {
            home_dir,
            downloads_dir,
            favorites_dir,
            wallpaper_dir,
            config_name,
            config_file,
            config_type,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create all necessary paths
    pub fn create_all(&self) -> Result<()> {
        self.create_config_dirs()?;
        self.create_config_file(None)?;
        Ok(())
    }

    /// Create all necessary directories (home, downloads, favorites, wallpaper).
    pub fn create_config_dirs(&self) -> Result<()> {
        create_dir_all(&self.home_dir)?;
        create_dir_all(&self.downloads_dir)?;
        create_dir_all(&self.favorites_dir)?;
        create_dir_all(&self.wallpaper_dir)?;
        Ok(())
    }

    /// Create the config file if it does not exist.
    pub fn create_config_file(&self, default_content: Option<&str>) -> Result<()> {
        if !self.config_exists() {
            let mut file = File::create(&self.config_file)?;
            if let Some(content) = default_content {
                file.write_all(content.as_bytes())?;
            }
        }
        Ok(())
    }

    /// Check if the config file exists.
    pub fn config_exists(&self) -> bool {
        self.config_file.exists()
    }

    /// Builder method to set the config file name.
    pub fn with_config_name<S: Into<String>>(mut self, name: S) -> Self {
        self.config_name = name.into();
        self.update_config_file();
        self
    }

    /// Builder method to set the config file type.
    pub fn with_type(mut self, config_type: Type) -> Self {
        self.config_type = config_type;
        self.update_config_file();
        self
    }

    /// Private helper to update the config_file path.
    fn update_config_file(&mut self) {
        self.config_file = self.home_dir.join(format!(
            "{}.{}",
            self.config_name,
            self.config_type.extension()
        ));
    }
}

impl Type {
    /// Returns the file extension for this config type (without dot).
    pub fn extension(self) -> &'static str {
        match self {
            Type::Toml => "toml",
            Type::Json => "json",
        }
    }

    /// Detect config type from file extension
    pub fn from_extension(path: &Path) -> Result<Self> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| match ext.to_lowercase().as_str() {
                "toml" => Type::Toml,
                "json" => Type::Json,
                _ => Type::default(),
            })
            .ok_or_else(|| Error::ConfigError("Unknown config file format".into()))
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        //{ Set the padding width for alignment }
        const PAD: usize = 24;

        printf!(f, "Home Directory", self.home_dir.display(), PAD)?;
        printf!(
            f,
            "Downloads Directory",
            self.downloads_dir.display(),
            PAD
        )?;
        printf!(
            f,
            "Favorites Directory",
            self.favorites_dir.display(),
            PAD
        )?;
        printf!(
            f,
            "Wallpaper Directory",
            self.wallpaper_dir.display(),
            PAD
        )?;
        printf!(f, "Config File", self.config_file.display(), PAD)?;
        Ok(())
    }
}
