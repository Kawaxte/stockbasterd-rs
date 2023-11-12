use futures::future::join_all;
use reqwest::header::USER_AGENT;
use reqwest::Client;

use std::collections::HashMap;
use std::fs::File;

use tokio::runtime::Runtime;

use crate::util::html_utils::{fetch_url_from_href_a, fetch_url_from_href_b};
use crate::websites::Website;

fn get_payload(url: String) -> HashMap<String, String> {
    let url_owned = url.to_owned();

    let mut data = HashMap::new();

    let site = Website::from(url_owned);
    match site {
        Website::EStockPhoto | Website::GettyImages | Website::IStock => {
            data.insert(String::from("get_url"), url);
            data.insert(String::from("download"), String::new());
        }
        Website::BigStockPhoto => {
            data.insert(String::from("url"), url);
            data.insert(
                String::from("token"),
                String::from("5f1c6979a54c99e1398296826675621a"),
            );
            data.insert(String::from("send"), String::new());
        }
    }
    data
}

async fn download(
    url: String,
    dest: &dyn AsRef<std::path::Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let site = Website::from(url.to_owned());
    let data = get_payload(url.to_owned());

    let client = Client::new();

    println!("Sending POST request to '{}'", site.as_str());

    let res = client.post(site.as_str())
    .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36")
    .form(&data)
    .send().await?;
    if res.status().is_success() {
        let res_url = res.url().to_string();
        let res_text = res.text().await?;

        let mut jpeg_url = fetch_url_from_href_a(res_url, res_text);
        if jpeg_url.contains("capture.php") {
            download_b(&mut jpeg_url).await?;
        }
        if !jpeg_url.is_empty() {
            println!("Sending GET request to '{}'", jpeg_url);

            let jpeg_res = client.get(jpeg_url.as_str()).send().await?;
            let jpeg_res_url = jpeg_res.url().to_string();

            let dest_ref = dest.as_ref();

            let file_name = jpeg_res_url
                .split(&['?', '=', '/'][..])
                .last()
                .unwrap()
                .to_string();
            let file_path = dest_ref.join(file_name);
            let mut file = File::create(file_path)?;

            let res_bytes = jpeg_res.bytes().await?;
            let mut res_bytes_ref = res_bytes.as_ref();

            std::io::copy(&mut res_bytes_ref, &mut file)?;
        }
    }

    Ok(())
}

async fn download_b(url: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    println!("Sending GET request to '{}'", url);

    let res = client.get(url.as_str()).send().await?;
    let res_status = res.status();
    Ok(if res_status.is_success() {
        let res_text = res.text().await?;

        *url = fetch_url_from_href_b(res_text);
    })
}

pub fn run(urls: Vec<String>, dest: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = Vec::new();

    urls.to_owned().into_iter().for_each(|url| {
        tasks.push(download(url, &dest));
    });

    let runtime = Runtime::new()?;
    runtime.block_on(async {
        join_all(tasks).await;
    });

    Ok(())
}
