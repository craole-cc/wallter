use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  fmt::{self, Display, Formatter}
};

/// Configuration for an individual wallpaper source API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Source {
  /// The name of the source.
  ///
  /// This should be a unique identifier for the source, like "wallhaven",
  /// "unsplash", or "pixabay". Example: `name: "wallhaven".into()`
  pub name: String,

  /// The API key used to access the source.
  ///
  /// This should be an `Option<String>`. Set to `None` if no key is needed, or
  /// `Some("YOUR_API_KEY".to_string())` if a key is provided.
  /// Example: `api_key: Some("abc123def456".to_string())`
  pub api_key: Option<String>,

  /// The base URL for the API endpoint.
  ///
  /// This is the root URL for API requests to this source.
  /// Example: `base_url: "https://wallhaven.cc/api/v1/".into()`
  pub base_url: String,

  /// A default search query to use when this source is selected.
  ///
  /// This is an `Option<String>`. Set to `None` if no default query should be
  /// used, or `Some("random".to_string())` for a specific default.
  /// Example: `default_query: Some("nature".to_string())`
  pub default_query: Option<String>,

  /// Specifies if this source requires an API key for access.
  ///
  /// Set to `true` if an API key is mandatory for using this source, `false`
  /// otherwise. Example: `requires_api_key: true`
  pub requires_api_key: bool,

  /// Indicates whether this source is currently enabled for use by the user.
  ///
  /// This is a user setting: `true` means the user intends to use this source.
  /// This does not necessarily mean the source is "valid" or ready to make API
  /// calls. Example: `enabled: true` or `enabled: false`
  pub enabled: bool,

  /// Indicates whether this source is currently valid and ready for API calls.
  ///
  /// This is a runtime status automatically determined based on
  /// `requires_api_key` and the presence of `api_key`. If `requires_api_key`
  /// is true and `api_key` is `None`, `valid` will be `false`. Otherwise, it
  /// will be `true`.
  pub valid: bool,

  /// A map of source-specific API parameters.
  ///
  /// Use this to define default query parameters that are unique to this API
  /// source and are not derived from the application's runtime context (like
  /// monitor resolution). Example:
  /// ```
  /// params: HashMap::from([
  ///     ("categories".into(), "general,anime".into()),
  ///     ("purity".into(), "sfw".into()),
  ///     ("sorting".into(), "random".into()),
  /// ])
  /// ```
  pub params: HashMap<String, String>
}

impl Default for Source {
  /// Provides a generic default `Source` configuration.
  ///
  /// The `enabled` status is `true` by default (user's intent).
  /// The `valid` status is calculated based on `requires_api_key` and `api_key`
  /// presence.
  fn default() -> Self {
    let default_requires_api_key = false; //? Generic default, overridden for specific APIs
    let default_api_key: Option<String> = None;

    Self {
      name: String::from("Unknown"),
      api_key: default_api_key.clone(),
      base_url: String::new(),
      default_query: None, //? Default to no specific query
      requires_api_key: default_requires_api_key,
      enabled: true, //? Default user intent is to enable
      //? Valid is true if no key is required OR if a key is required AND provided
      valid: !default_requires_api_key || default_api_key.is_some(),
      params: HashMap::new() //? Default to an empty HashMap for params
    }
  }
}

impl Source {
  /// Creates a new `Source` instance with essential fields, using `Default` for
  /// others.
  ///
  /// This constructor simplifies setting up a source by requiring only the
  /// `name`, `base_url`, and `requires_api_key` fields, and filling in the
  /// rest with default values from `Source::default()`.
  ///
  /// The `valid` status will be automatically determined based on
  /// `requires_api_key` and whether an API key is provided via `with_api_key`
  /// or directly in the config. The `enabled` status is `true` by default but
  /// can be changed with `with_enabled()`.
  ///
  /// # Arguments
  ///
  /// * `name` - The unique name of the source, which can be any type that
  ///   converts into a `String` (e.g., `String` or `&str`).
  /// * `base_url` - The base URL for the API endpoint, which can be any type
  ///   that converts into a `String` (e.g., `String` or `&str`).
  /// * `requires_api_key` - A boolean indicating if this source absolutely
  ///   needs an API key.
  ///
  /// # Returns
  ///
  /// A new `Source` instance, ready for further configuration via chaining
  /// methods.
  ///
  /// # Examples
  ///
  /// ```
  /// use std::collections::HashMap;
  /// # use wallter::config::api::Source; // Assuming `wallter` is your crate name
  ///
  /// let wallhaven = Source::new("wallhaven", "[https://wallhaven.cc/api/v1/](https://wallhaven.cc/api/v1/)", false);
  /// assert_eq!(wallhaven.name, "wallhaven");
  /// assert_eq!(wallhaven.base_url, "[https://wallhaven.cc/api/v1/](https://wallhaven.cc/api/v1/)");
  /// assert!(!wallhaven.requires_api_key);
  /// assert!(wallhaven.enabled); // Enabled by default
  /// assert!(wallhaven.valid); // Valid because no key is required
  /// assert!(wallhaven.api_key.is_none());
  /// assert!(wallhaven.params.is_empty());
  ///
  /// let unsplash = Source::new("unsplash", "[https://api.unsplash.com/](https://api.unsplash.com/)", true);
  /// assert_eq!(unsplash.name, "unsplash");
  /// assert!(unsplash.requires_api_key);
  /// assert!(unsplash.enabled); // Enabled by default (user intent)
  /// assert!(!unsplash.valid); // Not valid because it requires a key that's not provided yet
  /// assert!(unsplash.api_key.is_none());
  ///
  /// // Using the builder pattern to configure further:
  /// let mut unsplash_with_key = Source::new("unsplash", "[https://api.unsplash.com/](https://api.unsplash.com/)", true)
  ///     .with_api_key("my_secret_key") // Accepts &str
  ///     .with_default_query("ocean") // Accepts &str
  ///     .with_params(HashMap::from([("orientation", "portrait")])) // Accepts &str for both key and value
  ///     .with_enabled(true); // Explicitly enable if desired (already true by default)
  ///
  /// assert!(unsplash_with_key.enabled);
  /// assert!(unsplash_with_key.valid); // Now valid because key is provided
  /// assert_eq!(unsplash_with_key.default_query, Some("ocean".to_string()));
  /// assert_eq!(unsplash_with_key.params.get("orientation"), Some(&"portrait".to_string()));
  /// ```
  pub fn new(name: impl Into<String>, base_url: impl Into<String>, requires_api_key: bool) -> Self {
    let mut s = Self {
      name: name.into(),
      base_url: base_url.into(),
      requires_api_key,
      ..Default::default()
    };
    //{ Determine validity based on `requires_api_key` and `api_key`. }
    s.valid = !s.requires_api_key || s.api_key.is_some();
    s
  }

  /// Sets the API key for the source.
  ///
  /// When an API key is provided, the source's `valid` status is set to `true`.
  /// The `enabled` status (user preference) is not affected by this method.
  ///
  /// # Arguments
  /// * `api_key` - The API key, which can be any type that converts into a
  ///   `String` (e.g., `String` or `&str`).
  pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
    self.api_key = Some(api_key.into());
    //? If an API key is provided, the source is now valid.
    self.valid = true;
    self
  }

  /// Clears the API key for the source and updates its `valid` status.
  ///
  /// If the source `requires_api_key` is true, clearing the key will set
  /// `valid` to `false`. The `enabled` status (user preference) is not
  /// affected by this method.
  pub fn clear_api_key(mut self) -> Self {
    self.api_key = None;
    //{ Re-evaluate `valid` based on the API key being cleared. }
    //? It's only valid if no key is required (since the key is now absent).
    self.valid = !self.requires_api_key;
    self
  }

  /// Sets the default search query for the source.
  ///
  /// # Arguments
  /// * `query` - The default search query, which can be any type that converts
  ///   into a `String` (e.g., `String` or `&str`).
  pub fn with_default_query(mut self, query: impl Into<String>) -> Self {
    self.default_query = Some(query.into());
    self
  }

  /// Clears the default search query for the source.
  pub fn clear_default_query(mut self) -> Self {
    self.default_query = None;
    self
  }

  /// Sets the source-specific API parameters.
  ///
  /// # Arguments
  /// * `params` - A `HashMap` where both keys and values can be any type that
  ///   converts into a `String` (e.g., `HashMap<&str, &str>`, `HashMap<String,
  ///   &str>`, etc.).
  /// # Examples
  /// ```
  /// use std::collections::HashMap;
  /// # use wallter::config::api::Source;
  /// let mut source = Source::new("test".into(), "[http://example.com](http://example.com)".into(), false);
  /// source = source.with_params(HashMap::from([
  ///     ("param1", "value1"),
  ///     ("param2", "value2"),
  /// ]));
  /// assert_eq!(source.params.get("param1"), Some(&"value1".to_string()));
  /// ```
  pub fn with_params<K, V>(mut self, params: HashMap<K, V>) -> Self
  where
    K: Into<String>,
    V: Into<String>
  {
    self.params = params
      .into_iter()
      .map(|(k, v)| (k.into(), v.into()))
      .collect();
    self
  }

  /// Explicitly sets the enabled status of the source.
  ///
  /// Note: This method only affects the `enabled` field (user preference) and
  /// does not automatically re-evaluate the `valid` status.
  ///
  /// # Arguments
  /// * `enabled` - A boolean indicating whether the source is enabled.
  pub fn with_enabled(mut self, enabled: bool) -> Self {
    self.enabled = enabled;
    self
  }

  /// Enables the source.
  ///
  /// This is a convenience method that sets the `enabled` status to `true`.
  ///
  /// # Examples
  pub fn enable(mut self) -> Self {
    self.enabled = true;
    self
  }
}

impl Display for Source {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    const PAD: usize = 24;

    printf!(f, "Source Name", &self.name, PAD)?;
    printf!(f, "Base URL", &self.base_url, PAD)?;
    printf!(
      f,
      "API Key",
      self.api_key.as_deref().unwrap_or("[Not Set]"),
      PAD
    )?;
    printf!(f, "Requires API Key", self.requires_api_key, PAD)?;
    printf!(f, "Enabled (User)", self.enabled, PAD)?;
    printf!(f, "Valid (Runtime)", self.valid, PAD)?;
    printf!(
      f,
      "Default Query",
      self.default_query.as_deref().unwrap_or("[None]"),
      PAD
    )?;

    if !self.params.is_empty() {
      printf!(f, "Parameters", "", PAD)?;
      for (key, value) in &self.params {
        printf!(f, key, value, PAD + 2)?;
      }
    }
    Ok(())
  }
}
