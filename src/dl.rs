use futures::future::join_all;
use reqwest::header::USER_AGENT;
use reqwest::Client;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::time::Instant;
use tokio::runtime::Runtime;

use crate::dl_fetch::{fetch_url_for_jpg_a, fetch_url_for_jpg_b};
use crate::dl_queue::Queue;
use crate::dl_website::BaseWebsite;

pub fn create_queue(path: &dyn AsRef<std::path::Path>) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut queue = Queue::new();
    for line in reader.lines() {
        let line = line.unwrap();
        queue.push(line)
    }
    queue.urls
}

pub fn process_queue(
    urls: Vec<String>,
    dest: &dyn AsRef<std::path::Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = Vec::new();

    let owned_urls = urls.to_owned();
    owned_urls.into_iter().for_each(|url| {
        tasks.push(download_a(url, dest));
    });

    let now = Instant::now();

    let runtime = Runtime::new()?;
    runtime.block_on(async {
        join_all(tasks).await;
    });

    let elapsed = now.elapsed();
    println!("Downloaded {} JPG(s) in {:.3?}", urls.len(), elapsed);
    Ok(())
}

async fn download_a(
    url: String,
    dest: &dyn AsRef<std::path::Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut data = HashMap::new();

    let base = BaseWebsite::to_base(url.to_owned());
    match base {
        BaseWebsite::EStockPhoto
        | BaseWebsite::GettyImages
        | BaseWebsite::IStock
        // | BaseWebsite::OneTwoThreeRf
        | BaseWebsite::ShutterStock => {
            data.insert("get_url", url.as_str());
            data.insert("download", "");
        }
        _ => {
            data.insert("url", url.as_str());
            data.insert("token", "5f1c6979a54c99e1398296826675621a");
            data.insert("send", "");
        }
    };

    println!("Sending POST request to '{}'", base.as_str());

    let client = Client::new();
    let res = client.post(base.as_str())
    .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36")
    .form(&data)
    .send().await?;
    let res_status = res.status();
    if res_status.is_success() {
        let res_url = res.url().to_string();
        let res_text = res.text().await?;

        let mut jpg_url = fetch_url_for_jpg_a(res_url, res_text);
        if jpg_url.contains("capture.php") {
            download_b(&mut jpg_url).await?;
        }
        if !jpg_url.is_empty() {
            println!("Sending GET request to '{}'", jpg_url);

            let jpg_res = client.get(jpg_url.as_str()).send().await?;
            let jpg_res_url = jpg_res.url().to_string();

            download(dest, jpg_res_url, jpg_res).await?;
        }
        return Ok(());
    }
    Ok(())
}

async fn download_b(jpg_url: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    println!("Sending GET request to '{}'", jpg_url);

    let res = client.get(jpg_url.as_str()).send().await?;
    let res_status = res.status();
    Ok(if res_status.is_success() {
        let res_text = res.text().await?;

        *jpg_url = fetch_url_for_jpg_b(res_text);
    })
}

async fn download(
    dest: &dyn AsRef<std::path::Path>,
    jpg_res_url: String,
    jpg_res: reqwest::Response,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(if dest.as_ref().is_dir() {
        let jpg_name = jpg_res_url
            .split(&['?', '=', '/'][..])
            .last()
            .unwrap()
            .to_string();

        let mut jpg_dest = dest.as_ref().to_path_buf();
        jpg_dest.push(jpg_name.to_string());
        println!(
            "Downloading '{}' to '{}'",
            jpg_name,
            dest.as_ref().to_str().unwrap()
        );

        let mut file = File::create(jpg_dest)?;
        let content = jpg_res.bytes().await?;

        std::io::copy(&mut content.as_ref(), &mut file)?;
    } else {
        return Err("'{}' is not a directory, stopping...".to_string().into());
    })
}
