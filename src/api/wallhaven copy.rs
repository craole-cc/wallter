// src/api/wallhaven.rs
use crate::{Error, Result};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap; // Import HashMap for the new SearchParams

/// Represents the top-level response structure from Wallhaven API search.
#[derive(Debug, Deserialize)]
pub struct Response {
  /// List of wallpapers
  pub data: Vec<Wallpaper>,

  /// Metadata for pagination, etc.
  pub meta: Option<Meta>
}

/// Meta data from Wallhaven API response.
#[derive(Debug, Deserialize)]
pub struct Meta {
  pub current_page: u32,
  pub last_page: u32,
  pub per_page: u32,
  pub total: u32,
  pub query: Option<String>, // Added: The search query used for the request
  pub seed: Option<String>   // Added: The seed used for random sorting
}

/// Represents a single wallpaper from the Wallhaven API.
#[derive(Debug, Deserialize, Clone)]
pub struct Wallpaper {
  /// The unique ID of the wallpaper
  pub id: String,

  /// The URL of the wallpaper page
  pub url: String,

  /// The direct image URL
  pub path: String,

  /// The horizontal dimension of the wallpaper, e.g., 1920
  pub dimension_x: u32,

  /// The vertical dimension of the wallpaper, e.g., 1080
  pub dimension_y: u32,

  /// Colors tags of the wallpaper, e.g., ["#FF0000", "#00FF00", "#0000FF"]
  pub colors: Vec<String>,

  /// The rating of the wallpaper, e.g., ["sfw", "sketchy", "nsfw"]
  pub purity: String,

  /// The category of the wallpaper, e.g., "general"
  pub category: String,

  /// The number of views of the wallpaper
  views: u32,

  /// The number of favorites of the wallpaper
  favorites: u32
}

/// Represents the parameters for a Wallhaven API search.
/// This struct consolidates all the options for searching wallpapers.
#[derive(Debug, Default, Clone)]
pub struct SearchParams {
  pub query: Option<String>,
  pub resolutions: Option<String>,
  pub ratios: Option<String>,
  pub categories: Option<String>,
  pub purity: Option<String>, /* Wallhaven purity filter (e.g., "110" for sfw+sketchy, or
                               * "sfw,sketchy") */
  pub sorting: Option<String>,
  pub page: Option<u32>,
  pub per_page: Option<u32>,
  pub colors: Option<String>, // Wallhaven has a 'colors' param for hex codes
  pub changes: Option<String>, /* Wallhaven has a 'changes' param for ID after which to get new
                               * wallpapers */
  pub top_range: Option<String>, /* Wallhaven has 'top_range' for sorting by top (e.g., "1M",
                                  * "3M", "1y") */
  pub atleast: Option<String>, // Wallhaven has 'atleast' for minimum resolution
  pub appeal: Option<String>,  // Wallhaven has 'appeal' for the "hot" sort
  pub ai_art_id: Option<String>, // Wallhaven parameter for authenticated users
  pub user: Option<String>,    // Wallhaven parameter for authenticated users
  // You can add more Wallhaven-specific parameters here as needed
  // based on their API documentation.
  pub custom_params: HashMap<String, String> // For any other less common or dynamic params
}

impl SearchParams {
  /// Creates a new, empty `SearchParams` instance.
  pub fn new() -> Self {
    SearchParams::default()
  }

  // Builder methods for each parameter for a fluent API
  pub fn with_query(mut self, query: impl Into<String>) -> Self {
    self.query = Some(query.into());
    self
  }

  pub fn with_resolutions(mut self, resolutions: impl Into<String>) -> Self {
    self.resolutions = Some(resolutions.into());
    self
  }

  pub fn with_ratios(mut self, ratios: impl Into<String>) -> Self {
    self.ratios = Some(ratios.into());
    self
  }

  pub fn with_categories(mut self, categories: impl Into<String>) -> Self {
    self.categories = Some(categories.into());
    self
  }

  pub fn with_purity(mut self, purity: impl Into<String>) -> Self {
    self.purity = Some(purity.into());
    self
  }

  pub fn with_sorting(mut self, sorting: impl Into<String>) -> Self {
    self.sorting = Some(sorting.into());
    self
  }

  pub fn with_page(mut self, page: u32) -> Self {
    self.page = Some(page);
    self
  }

  pub fn with_per_page(mut self, per_page: u32) -> Self {
    self.per_page = Some(per_page);
    self
  }

  pub fn with_colors(mut self, colors: impl Into<String>) -> Self {
    self.colors = Some(colors.into());
    self
  }

  pub fn with_ai_art_id(mut self, ai_art_id: impl Into<String>) -> Self {
    self.ai_art_id = Some(ai_art_id.into());
    self
  }

  pub fn with_user(mut self, user: impl Into<String>) -> Self {
    self.user = Some(user.into());
    self
  }

  pub fn with_custom_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
    self.custom_params.insert(key.into(), value.into());
    self
  }
}

/// Wallhaven API client.
pub struct Api {
  client: Client,
  base_url: String, //? Stores the base URL dynamically
  api_key: Option<String>
}

impl Api {
  /// Creates a new Wallhaven API client.
  ///
  /// # Arguments
  /// * `base_url` - The base URL for Wallhaven API (e.g., "https://wallhaven.cc/api/v1/").
  /// * `api_key` - Optional API key for authenticated requests.
  pub fn new(base_url: String, api_key: Option<String>) -> Self {
    Self {
      client: Client::new(),
      base_url,
      api_key
    }
  }

  /// Searches for wallpapers on Wallhaven based on the provided search
  /// parameters. Returns a vector of Wallpapers.
  ///
  /// This method now accepts a `SearchParams` struct, making the function
  /// signature much cleaner and more extensible.
  pub async fn search_wallpapers(&self, params: &SearchParams) -> Result<Vec<Wallpaper>> {
    let mut url = format!("{}search?", self.base_url);
    let has_api_key = self.api_key.is_some();

    // Append query if present, it's often the first parameter
    if let Some(q) = &params.query {
      url.push_str(&format!("q={q}&"));
    } else {
      // Wallhaven often defaults to latest if no query is given, but some sorts
      // require it. If query is None, ensure sorting is not 'relevance'
      // or similar For simplicity here, we'll ensure a query is present
      // or a default sorting is used.
    }

    // Default sorting if not provided in params
    let sorting = params.sorting.as_deref().unwrap_or("random"); // Default to "random" if not specified
    url.push_str(&format!("sorting={sorting}"));

    if let Some(res) = &params.resolutions {
      url.push_str(&format!("&resolutions={res}"));
    }
    if let Some(rat) = &params.ratios {
      url.push_str(&format!("&ratios={rat}"));
    }
    if let Some(cat) = &params.categories {
      url.push_str(&format!("&categories={cat}"));
    }

    // Handle purity based on API key presence
    if let Some(mut purity_val) = params.purity.clone() {
      if !has_api_key {
        // If no API key, filter out NSFW purity option
        // Wallhaven purity can be "sfw,sketchy,nsfw" or "100", "010", "001"
        // The 'nsfw' part (or '001') requires an API key.
        let original_purity = purity_val.clone(); // Keep original for warning

        // Attempt to remove "nsfw" from comma-separated string
        purity_val = purity_val
          .split(',')
          .filter(|s| s.trim().to_lowercase() != "nsfw")
          .collect::<Vec<&str>>()
          .join(",");

        // Attempt to remove "001" from numeric string
        if purity_val.contains("001") {
          purity_val = purity_val.replace("001", ""); // Remove "001"
          // If purity becomes "11" from "111", ensure it's still valid
          if purity_val.is_empty() {
            // If "001" was the only or last part, ensure it's removed cleanly
            purity_val = original_purity
              .replace("001", "")
              .split(',')
              .filter(|s| !s.is_empty())
              .collect::<Vec<&str>>()
              .join(",");
            if purity_val.is_empty() {
              purity_val = "110".to_string();
            } // Default to SFW+Sketchy
          }
        }

        if purity_val != original_purity {
          eprintln!(
            "Warning: 'nsfw' purity was requested without an API key. Filtering to '{purity_val}'."
          );
        }
      }
      url.push_str(&format!("&purity={purity_val}"));
    }

    if let Some(p) = params.page {
      url.push_str(&format!("&page={p}"));
    }
    if let Some(pp) = params.per_page {
      url.push_str(&format!("&per_page={pp}"));
    }
    if let Some(col) = &params.colors {
      url.push_str(&format!("&colors={col}"));
    }
    if let Some(chg) = &params.changes {
      url.push_str(&format!("&changes={chg}"));
    }
    if let Some(tr) = &params.top_range {
      url.push_str(&format!("&top_range={tr}"));
    }
    if let Some(atl) = &params.atleast {
      url.push_str(&format!("&atleast={atl}"));
    }
    if let Some(app) = &params.appeal {
      url.push_str(&format!("&appeal={app}"));
    }

    // Add authenticated parameters only if API key is present
    if has_api_key {
      if let Some(ai_art_id) = &params.ai_art_id {
        url.push_str(&format!("&ai_art_id={ai_art_id}"));
      }
      if let Some(user) = &params.user {
        url.push_str(&format!("&user={user}"));
      }
    } else {
      // Explicitly warn if these were set without an API key (though the conditional
      // above prevents sending them)
      if params.ai_art_id.is_some() {
        eprintln!(
          "Warning: 'ai_art_id' was set but no API key is provided. This parameter will be ignored."
        );
      }
      if params.user.is_some() {
        eprintln!(
          "Warning: 'user' filter was set but no API key is provided. This parameter will be ignored."
        );
      }
    }

    // Add any custom parameters
    for (key, value) in &params.custom_params {
      url.push_str(&format!("&{key}={value}"));
    }

    // Ensure the URL doesn't end with '&' or '?' if no parameters were added
    if url.ends_with('&') {
      url.pop();
    }
    if url.ends_with('?') && url.ends_with("search?") {
      // Only remove if it's just 'search?'
      url.pop();
    }

    let mut request = self.client.get(&url);
    if let Some(key) = &self.api_key {
      request = request.header("X-API-Key", key);
    }

    let response = request.send().await.map_err(Error::NetworkError)?;

    //{ Check for non-success status codes from Wallhaven API }
    if !response.status().is_success() {
      let status = response.status();
      let text = response.text().await.unwrap_or_default();
      return Err(Error::ApiError(format!(
        "Wallhaven API error: Status {status}, Response: {text}"
      )));
    }

    let data: Response = response
      .json()
      .await
      .map_err(|e| Error::ApiError(e.to_string()))?;

    Ok(data.data)
  }

  /// Downloads a wallpaper image from its direct URL.
  pub async fn download_wallpaper(&self, url: &str, path: &std::path::Path) -> Result<()> {
    let response = self
      .client
      .get(url)
      .send()
      .await
      .map_err(Error::NetworkError)?;

    if !response.status().is_success() {
      let status = response.status();
      let text = response.text().await.unwrap_or_default();
      return Err(Error::ApiError(format!(
        "Failed to download wallpaper: Status {status}, Response: {text}"
      )));
    }

    let bytes = response.bytes().await.map_err(Error::NetworkError)?;

    tokio::fs::write(path, bytes)
      .await
      .map_err(Error::IoError)?;

    Ok(())
  }
}
