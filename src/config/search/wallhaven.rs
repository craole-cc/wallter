use crate::api::wallhaven::{Order, Sorting, ToplistRange};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Wallhaven-specific search parameters for the configuration.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Params {
  /// Default search query. Example: "nature", "id:123"
  pub default_query: Option<String>,

  /// Categories (General, Anime, People).
  pub categories: Option<(bool, bool, bool)>,

  /// Purity (SFW, Sketchy, NSFW).
  pub purity: Option<(bool, bool, bool)>,

  /// Default sorting method.
  pub sorting: Option<Sorting>,

  /// Default sorting order.
  pub order: Option<Order>,

  /// Time range for toplist sorting.
  pub top_range: Option<ToplistRange>,

  /// Minimum resolution. Example: "1920x1080".
  pub atleast: Option<String>,

  /// List of exact resolutions. Example: "1920x1080,2560x1440".
  pub resolutions: Option<String>,

  /// List of aspect ratios. Example: "16x9".
  pub ratios: Option<String>,

  /// Search by color hex code. Example: "663399".
  pub colors: Option<String>
}

impl Display for Params {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    printf!(
      f,
      "Default Query",
      self.default_query.as_deref().unwrap_or("[None]")
    )?;

    if let Some(cats) = self.categories {
      let cat_str = format!(
        "G:{} A:{} P:{}",
        if cats.0 { "✓" } else { "✗" },
        if cats.1 { "✓" } else { "✗" },
        if cats.2 { "✓" } else { "✗" }
      );
      printf!(f, "Categories", cat_str)?;
    }
    if let Some(purs) = self.purity {
      let pur_str = format!(
        "SFW:{} Sketchy:{} NSFW:{}",
        if purs.0 { "✓" } else { "✗" },
        if purs.1 { "✓" } else { "✗" },
        if purs.2 { "✓" } else { "✗" }
      );
      printf!(f, "Purity", pur_str)?;
    }
    if let Some(sorting) = self.sorting {
      printf!(f, "Sorting", format!("{sorting:?}"))?;
    }
    if let Some(order) = self.order {
      printf!(f, "Order", format!("{order:?}"))?;
    }
    if let Some(range) = self.top_range {
      printf!(f, "Top Range", format!("{range:?}"))?;
    }
    if let Some(res) = &self.atleast {
      printf!(f, "Min Resolution", res)?;
    }
    if let Some(res) = &self.resolutions {
      printf!(f, "Exact Resolutions", res)?;
    }
    if let Some(ratio) = &self.ratios {
      printf!(f, "Aspect Ratios", ratio)?;
    }
    if let Some(color) = &self.colors {
      printf!(f, "Color", color)?;
    }

    Ok(())
  }
}
