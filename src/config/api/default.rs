use super::Source;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Global API configuration for all wallpaper sources.
/// This acts as the main configuration struct for the `api` module.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
  /// List of configured wallpaper sources
  pub sources: Vec<Source>,

  /// The ordered list of source names by priority. When fetching, the
  /// application will attempt to use sources in this order until a wallpaper
  /// is successfully retrieved.
  pub rank: Vec<String>
}

impl Default for Config {
  /// Creates a new `Config` instance with default values.
  /// By default, it initializes with a common set of wallpaper sources.
  fn default() -> Self {
    //{ Define default sources directly here, including specific parameters }
    let wallhaven_source = Source {
      name: "wallhaven".into(),
      base_url: "".into(),
      requires_api_key: false, //? It can function without a key
      wallhaven: Some(super::wallhaven::Params {
        categories: Some((true, true, false)), // General & Anime
        purity: Some((true, false, false)),    // SFW only
        sorting: Some(crate::api::wallhaven::Sorting::Random),
        ..Default::default()
      }),
      ..Default::default()
    };

    let unsplash_source = Source {
      name: "unsplash".into(),
      base_url: "https://api.unsplash.com/".into(),
      requires_api_key: true,
      ..Default::default()
    };

    let pixabay_source = Source {
      name: "pixabay".into(),
      base_url: "https://pixabay.com/api/".into(),
      requires_api_key: true,
      ..Default::default()
    };

    let default_sources =
      vec![wallhaven_source, unsplash_source, pixabay_source];

    //{ Define default rank order based on the default sources' names }
    let default_rank_names: Vec<String> = default_sources
      .iter()
      .map(|source| source.name.clone())
      .collect();

    Self {
      sources: default_sources,
      rank: default_rank_names
    }
  }
}

impl Config {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    //{ Set the padding width for alignment for main source details }
    const PAD: usize = 24;

    //{ Display the list of sources and their details }
    for source in &self.sources {
      //{ Determine and display rank }
      let rank_display = self
        .rank
        .iter()
        .position(|name| name == &source.name)
        .map(|rank| (rank + 1).to_string())
        .unwrap_or_else(|| "[N/A]".to_string()); 
      printf!(f, "Rank", rank_display, PAD)?;

      //{ Display source information }
      writeln!(f, "{source}")?;
    }
    Ok(())
  }
}
