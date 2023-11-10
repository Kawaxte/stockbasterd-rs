pub enum BaseWebsite {
    BigStockPhoto,
    EStockPhoto,
    GettyImages,
    IStock,
    // OneTwoThreeRf,
    ShutterStock,
}

impl BaseWebsite {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::BigStockPhoto => "https://fetchpik.com/bigstockphoto-downloader.php",
            Self::EStockPhoto => "https://toolxox.com/dl/e_stock/getx.php",
            Self::GettyImages => "https://toolxox.com/dl/2/dlgrab_1/",
            Self::IStock => "https://toolxox.com/dl/1/is_1.0/",
            // Self::OneTwoThreeRf => "https://toolxox.com/dl/123rf_ads/index.php",
            Self::ShutterStock => "https://snapwordz.com/dl/ss/index2.php",
        }
    }

    pub fn to_base(url: String) -> Self {
        let trim_url = url
            .trim_start_matches("https://")
            .trim_start_matches("www.");
        match trim_url {
            url if url.starts_with("bigstockphoto.") => Self::BigStockPhoto,
            url if url.starts_with("estockphoto.") => Self::EStockPhoto,
            url if url.starts_with("gettyimages.") => Self::GettyImages,
            url if url.starts_with("istockphoto.") => Self::IStock,
            // url if url.starts_with("123rf.") => Self::OneTwoThreeRf,
            url if url.starts_with("shutterstock.") => Self::ShutterStock,
            _ => panic!("'{}' is not supported", trim_url),
        }
    }
}

impl std::fmt::Display for BaseWebsite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base_url = self.as_str();
        write!(f, "{}", base_url)
    }
}
