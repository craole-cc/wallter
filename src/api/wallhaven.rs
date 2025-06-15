use crate::{Error, Result};
use reqwest::Client;
use serde::Deserialize;

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
  pub total: u32 // Add other fields from meta if needed (e.g., query, seed)
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

  /// Searches for wallpapers on Wallhaven based on query, resolution, and other
  /// parameters. Returns a vector of Wallpapers.
  pub async fn search_wallpapers(
    &self,
    query: &str,               //? Search query (e.g., "nature", "abstract")
    resolutions: Option<&str>, //? Comma-separated resolutions (e.g., "1920x1080,2560x1440")
    ratios: Option<&str>,      //? Comma-separated ratios (e.g., "16x9,16x10")
    categories: Option<&str>,  //? Comma-separated categories (e.g., "general,anime")
    purity: Option<&str>,      //? Comma-separated purity (e.g., "sfw,sketchy")
    sorting: &str,             //? Sorting order (e.g., "random", "relevance", "date_added")
    page: Option<u32>,         //? Page number for results
    per_page: Option<u32>      //? Number of results per page (max 100)
  ) -> Result<Vec<Wallpaper>> {
    let mut url = format!(
      "{}search?q={}&sorting={}", //? Use self.base_url here
      self.base_url, query, sorting
    );

    if let Some(res) = resolutions {
      url.push_str(&format!("&resolutions={}", res));
    }
    if let Some(rat) = ratios {
      url.push_str(&format!("&ratios={}", rat));
    }
    if let Some(cat) = categories {
      url.push_str(&format!("&categories={}", cat));
    }
    if let Some(pur) = purity {
      url.push_str(&format!("&purity={}", pur));
    }
    if let Some(p) = page {
      url.push_str(&format!("&page={}", p));
    }
    if let Some(pp) = per_page {
      url.push_str(&format!("&per_page={}", pp));
    }

    let mut request = self.client.get(&url);
    if let Some(key) = &self.api_key {
      request = request.header("X-API-Key", key);
    }

    let response = request.send().await.map_err(|e| Error::NetworkError(e))?;

    // Check for non-success status codes from Wallhaven API
    if !response.status().is_success() {
      let status = response.status();
      let text = response.text().await.unwrap_or_default();
      return Err(Error::ApiError(format!(
        "Wallhaven API error: Status {}, Response: {}",
        status, text
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
      .map_err(|e| Error::NetworkError(e))?;

    if !response.status().is_success() {
      let status = response.status();
      let text = response.text().await.unwrap_or_default();
      // Changed from NetworkError to ApiError as this is a formatted error message.
      return Err(Error::ApiError(format!(
        "Failed to download wallpaper: Status {}, Response: {}",
        status, text
      )));
    }

    let bytes = response.bytes().await.map_err(|e| Error::NetworkError(e))?;

    tokio::fs::write(path, bytes)
      .await
      .map_err(|e| Error::IoError(e))?;

    Ok(())
  }
}
