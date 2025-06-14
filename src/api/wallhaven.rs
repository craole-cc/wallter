// src/api/wallhaven.rs
use crate::{Error, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
  pub data: Wallpaper
}

#[derive(Debug, Deserialize)]
pub struct Wallpaper {
  pub id: String,
  pub url: String,
  pub path: String,
  pub resolution: String,
  pub dimension_x: u32,
  pub dimension_y: u32,
  pub colors: Vec<String>,
  pub purity: String,
  pub category: String
}

pub struct Api {
  client: reqwest::Client,
  api_key: Option<String>
}

impl Api {
  pub fn new(api_key: Option<String>) -> Self {
    Self {
      client: reqwest::Client::new(),
      api_key
    }
  }

  pub async fn search_wallpapers(
    &self,
    query: &str,
    resolution: Option<&str>,
    theme: Option<&str>
  ) -> Result<Vec<Wallpaper>> {
    let mut url = format!(
      "https://wallhaven.cc/api/v1/search?q={}&sorting=random",
      query
    );

    if let Some(res) = resolution {
      url.push_str(&format!("&resolutions={}", res));
    }

    let response = self
      .client
      .get(&url)
      .send()
      .await
      .map_err(|e| Error::NetworkError(e))?;

    let data: Response = response
      .json()
      .await
      .map_err(|e| Error::ApiError(e.to_string()))?;

    Ok(vec![data.data])
  }

  pub async fn download_wallpaper(&self, url: &str, path: &std::path::Path) -> Result<()> {
    let response = self
      .client
      .get(url)
      .send()
      .await
      .map_err(|e| Error::NetworkError(e))?;

    let bytes = response.bytes().await.map_err(|e| Error::NetworkError(e))?;

    tokio::fs::write(path, bytes)
      .await
      .map_err(|e| Error::IoError(e))?;

    Ok(())
  }
}
