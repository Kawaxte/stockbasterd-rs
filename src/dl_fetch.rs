use regex::Regex;
use scraper::{Html, Selector};

const UD_TOOLXOX_URL_A: &str = "https://ud.toolxox.com/nmmghgjgjhgjcvbcfbcbcvbfggjhjghjgjhgdfsfrrtryrtaisxmzxvocsdwzzxvbnmdkaoeokgkkgkfkkdkvgndhdhcnnbjgjjkdkskskskffjfjaaal.php?nznmtm=";
const UD_TOOLXOX_URL_B: &str = "https://ud.toolxox.com/ssmmghgjgjhgjcvbcfbcbcvbfggjhjghjgjhgdfsfrrtryrtaisxmzxvocsdwzzxvbnmdkaoeokgkkgkfkkdkvgndhdhcnnbjgjjkdkskskskffjfjaaal.php?nznmtm=";
const OUO_URL: &str = "http://ouo.io/qs/jdaLdBC7?s=";

pub fn fetch_url_for_jpg_a(mut url: String, html: String) -> String {
    let url_re_a = regex::Regex::new(r#".php\?nznmtm=.*?\.php\?key=.*?\.jpg"#)
        .expect("Failed to compile regular expression");
    let url_re_b = regex::Regex::new(r#".php\?nznmtm=.*?\.jpg"#)
        .expect("Failed to compile regular expression");
    let url_re_c = regex::Regex::new(r#"download\.php\?file=.*?\.jpg"#)
        .expect("Failed to compile regular expression");

    let php_re = Regex::new("[^/]*\\.php$").expect("Failed to compile regular expression");

    let mut jpg_url = String::new();

    let doc = Html::parse_document(&html);
    let selector = Selector::parse("a").expect("Failed to parse selector for <a> tag");
    for element in doc.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if url_re_a.is_match(href) || url_re_b.is_match(href) || url_re_c.is_match(href) {
                if href.contains(UD_TOOLXOX_URL_A) {
                    jpg_url = href.trim_start_matches(UD_TOOLXOX_URL_A).to_owned();
                } else if href.contains(UD_TOOLXOX_URL_B) {
                    jpg_url = href.trim_start_matches(UD_TOOLXOX_URL_B).to_owned();
                } else {
                    url = php_re.replace_all(&url, "").to_string();
                    jpg_url = format!("{}{}", url, href);
                }

                if jpg_url.contains(OUO_URL) {
                    jpg_url = jpg_url.trim_start_matches(OUO_URL).to_owned();
                }
                break;
            }
        }
    }
    jpg_url
}

pub fn fetch_url_for_jpg_b(html: String) -> String {
    let url_re =
        Regex::new(r#".*?rm_wm/images/.*?\.jpg"#).expect("Failed to compile regular expression");

    let mut jpg_url = String::new();

    let doc = Html::parse_document(&html);
    let selector = Selector::parse("a").expect("Failed to parse selector for <a> tag");
    for element in doc.select(&selector) {
        match element.value().attr("href") {
            Some(href) if url_re.is_match(href) => {
                jpg_url = href.to_owned();
                break;
            }
            _ => (),
        }
    }
    jpg_url
}
