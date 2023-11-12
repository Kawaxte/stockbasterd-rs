/*
 * Copyright (C) 2023 Kawaxte
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the
 * GNU Lesser General Public License as published by the Free Software Foundation, either version 3
 * of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License along with this
 * program. If not, see <https://www.gnu.org/licenses/>.
 */

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
