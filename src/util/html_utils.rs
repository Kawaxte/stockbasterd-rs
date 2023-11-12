use regex::Regex;
use scraper::{Html, Selector};

const UD_TOOLXOX_A: &str = "https://ud.toolxox.com/nmmghgjgjhgjcvbcfbcbcvbfggjhjghjgjhgdfsfrrtryrtaisxmzxvocsdwzzxvbnmdkaoeokgkkgkfkkdkvgndhdhcnnbjgjjkdkskskskffjfjaaal.php?nznmtm=";
const UD_TOOLXOX_B: &str = "https://ud.toolxox.com/ssmmghgjgjhgjcvbcfbcbcvbfggjhjghjgjhgdfsfrrtryrtaisxmzxvocsdwzzxvbnmdkaoeokgkkgkfkkdkvgndhdhcnnbjgjjkdkskskskffjfjaaal.php?nznmtm=";
const OUO: &str = "http://ouo.io/qs/jdaLdBC7?s=";

pub fn fetch_url_from_href_a(mut res_url: String, html: String) -> String {
    let url_re_a = regex::Regex::new(r#".php\?nznmtm=.*?\.php\?key=.*?\.jpg"#)
        .expect("Failed to compile regular expression");
    let url_re_b = regex::Regex::new(r#".php\?nznmtm=.*?\.jpg"#)
        .expect("Failed to compile regular expression");
    let url_re_c = regex::Regex::new(r#"download\.php\?file=.*?\.jpg"#)
        .expect("Failed to compile regular expression");
    let url_re_d = Regex::new("[^/]*\\.php$").expect("Failed to compile regular expression");

    let mut url = String::new();

    let doc = Html::parse_document(&html);
    let selector = Selector::parse("a").expect("Failed to parse selector for <a> tag");
    for element in doc.select(&selector) {
        let href = element.value().attr("href");
        if let Some(href) = href {
            if url_re_a.is_match(href) || url_re_b.is_match(href) || url_re_c.is_match(href) {
                if href.contains(UD_TOOLXOX_A) {
                    url = href.trim_start_matches(UD_TOOLXOX_A).to_owned();
                } else if href.contains(UD_TOOLXOX_B) {
                    url = href.trim_start_matches(UD_TOOLXOX_B).to_owned();
                } else {
                    res_url = url_re_d.replace_all(&res_url, "").to_string();
                    url = format!("{}{}", res_url, href);
                }

                if url.contains(OUO) {
                    url = url.trim_start_matches(OUO).to_owned();
                }
                break;
            }
        }
    }
    url
}

pub fn fetch_url_from_href_b(html: String) -> String {
    let url_re =
        Regex::new(r#".*?rm_wm/images/.*?\.jpg"#).expect("Failed to compile regular expression");

    let mut url = String::new();

    let document = Html::parse_document(&html);
    let anchor = Selector::parse("a").expect("Failed to parse selector");
    document.select(&anchor).for_each(|element| {
        let href = element.value().attr("href");
        match href {
            Some(href) => {
                if url_re.is_match(href) {
                    url = href.to_owned();
                }
            }
            _ => (),
        }
    });
    url.to_string()
}
