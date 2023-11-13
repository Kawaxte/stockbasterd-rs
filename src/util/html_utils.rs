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

use regex::Regex;
use scraper::{Html, Selector};

const URL_RE_A: &str = r#".php\?nznmtm=.*?\.php\?key=.*?\.jpg"#;
const URL_RE_B: &str = r#".php\?nznmtm=.*?\.jpg"#;
const URL_RE_C: &str = r#"download\.php\?file=.*?\.jpg"#;
const URL_RE_D: &str = "[^/]*\\.php$";
const URL_RE_E: &str = r#".*?rm_wm/images/.*?\.jpg"#;

const URL_BLOAT_A: &str = "https://ud.toolxox.com/nmmghgjgjhgjcvbcfbcbcvbfggjhjghjgjhgdfsfrrtryrtaisxmzxvocsdwzzxvbnmdkaoeokgkkgkfkkdkvgndhdhcnnbjgjjkdkskskskffjfjaaal.php?nznmtm=";
const URL_BLOAT_B: &str = "https://ud.toolxox.com/ssmmghgjgjhgjcvbcfbcbcvbfggjhjghjgjhgdfsfrrtryrtaisxmzxvocsdwzzxvbnmdkaoeokgkkgkfkkdkvgndhdhcnnbjgjjkdkskskskffjfjaaal.php?nznmtm=";
const URL_BLOAT_C: &str = "http://ouo.io/qs/jdaLdBC7?s=";

pub fn fetch_url_from_href_a(mut res_url: String, html: String) -> String {
    let url_re = regex::Regex::new(&format!("{}|{}|{}", URL_RE_A, URL_RE_B, URL_RE_C))
        .expect("Failed to compile regular expression");
    let url_re_d = regex::Regex::new(URL_RE_D).expect("Failed to compile regular expression");

    let mut url = String::new();

    let doc = Html::parse_document(&html);
    let anchor = Selector::parse("a").expect("Failed to parse selector");
    for element in doc.select(&anchor) {
        let href = element.value().attr("href");
        if let Some(href) = href {
            if url_re.is_match(href) {
                if href.contains(URL_BLOAT_A) {
                    url = href.trim_start_matches(URL_BLOAT_A).to_owned();
                } else if href.contains(URL_BLOAT_B) {
                    url = href.trim_start_matches(URL_BLOAT_B).to_owned();
                } else {
                    res_url = url_re_d.replace_all(&res_url, "").to_string();
                    url = format!("{}{}", res_url, href);
                }

                if url.contains(URL_BLOAT_C) {
                    url = url.trim_start_matches(URL_BLOAT_C).to_owned();
                }
                break;
            }
        }
    }
    url
}

pub fn fetch_url_from_href_b(html: String) -> String {
    let url_re = Regex::new(URL_RE_E).expect("Failed to compile regular expression");

    let mut url = String::new();

    let doc = Html::parse_document(&html);
    let anchor = Selector::parse("a").expect("Failed to parse selector");
    for element in doc.select(&anchor) {
        let href = element.value().attr("href");
        if let Some(href) = href {
            if url_re.is_match(href) {
                url = href.to_owned();
            }
        }
    }
    url
}
