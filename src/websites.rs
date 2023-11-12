fn trim_start_of_url(url: String) -> String {
    let url_start_trimmed = url
        .trim_start_matches("https://")
        .trim_start_matches("www.");
    url_start_trimmed.to_string()
}

#[derive(Debug)]
pub enum Website {
    EStockPhoto,
    BigStockPhoto,
    GettyImages,
    IStock,
}

impl Website {
    pub fn as_str(&self) -> &str {
        match self {
            Self::BigStockPhoto => "https://fetchpik.com/bigstockphoto-downloader.php",
            Self::EStockPhoto => "https://toolxox.com/dl/e_stock/getx.php",
            Self::GettyImages => "https://toolxox.com/dl/2/dlgrab_1/",
            Self::IStock => "https://toolxox.com/dl/1/is_1.0/",
        }
    }
}

impl From<String> for Website {
    fn from(url: String) -> Self {
        let url_start_trimmed = trim_start_of_url(url);
        match url_start_trimmed {
            url if url.starts_with("bigstockphoto.") => Self::BigStockPhoto,
            url if url.starts_with("estockphoto.") => Self::EStockPhoto,
            url if url.starts_with("gettyimages.") => Self::GettyImages,
            url if url.starts_with("istockphoto.") => Self::IStock,
            _ => panic!("'{}' is not supported", url_start_trimmed),
        }
    }
}

impl std::fmt::Display for Website {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url = self.as_str();
        write!(f, "{}", url)
    }
}
