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
    .send().await.expect(format!("Failed to send POST request to '{}'", site.as_str()).as_str());
    if res.status().is_success() {
        let res_url = res.url().to_string();
        let res_text = res
            .text()
            .await
            .expect(format!("Failed to retrieve HTML from '{}'", res_url).as_str());

        let mut jpeg_url = fetch_url_from_href_a(res_url, res_text);
        if jpeg_url.contains("capture.php") {
            download_b(&mut jpeg_url)
                .await
                .expect(format!("Failed to download entity from '{}'", jpeg_url).as_str());
        }
        if !jpeg_url.is_empty() {
            println!("Sending GET request to '{}'", jpeg_url);

            let jpeg_res = client
                .get(jpeg_url.as_str())
                .send()
                .await
                .expect(format!("Failed to send GET request to '{}'", jpeg_url).as_str());
            if jpeg_res.status().is_success() {
                let jpeg_res_url = jpeg_res.url().to_string();

                println!("Downloading '{}'", jpeg_res_url);

                let dest_ref = dest.as_ref();

                let file_name = jpeg_res_url
                    .split(&['?', '=', '/'][..])
                    .last()
                    .expect(
                        format!("Failed to retrieve file name from '{}'", jpeg_res_url).as_str(),
                    )
                    .to_string();
                let file_path = dest_ref.join(file_name);

                let mut file = File::create(file_path).expect("Failed to create file");
                let res_bytes = jpeg_res
                    .bytes()
                    .await
                    .expect(format!("Failed to retrieve bytes from '{}'", jpeg_res_url).as_str());
                let mut res_bytes_ref = res_bytes.as_ref();

                std::io::copy(&mut res_bytes_ref, &mut file).expect(
                    format!("Failed to copy bytes from '{}' to file", jpeg_res_url).as_str(),
                );
            }
        } else {
            println!("Failed to download from '{}'", url);
        }
    } else {
        println!("Failed to download from '{}'", site.as_str());
    }

    Ok(())
}

async fn download_b(url: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    println!("Sending GET request to '{}'", url);

    let res = client
        .get(url.as_str())
        .send()
        .await
        .expect(format!("Failed to send GET request to '{}'", url).as_str());
    let res_status = res.status();
    Ok(if res_status.is_success() {
        let res_text = res
            .text()
            .await
            .expect(format!("Failed to retrieve HTML from '{}'", url).as_str());

        *url = fetch_url_from_href_b(res_text);
    })
}

pub fn run(
    urls: Vec<String>,
    dest: std::path::PathBuf,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut tasks = Vec::new();

    for url in &urls {
        tasks.push(download(url.to_owned(), &dest));
    }

    let runtime = Runtime::new().expect("Failed to create runtime");
    runtime.block_on(async {
        let task = join_all(tasks).await;
        for res in task {
            if let Err(e) = res {
                panic!("{}", e);
            }
        }
    });

    Ok(format!("Downloaded {} entities", urls.len()))
}
