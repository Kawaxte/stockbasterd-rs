use download::run;
use util::file_utils::{open_file_dialog, open_file_dialog_for_dir, read_contents};

use crate::queue::Queue;

mod download;
mod queue;
mod util;
mod websites;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let src = open_file_dialog();

    let mut queue = Queue::new();

    let txt = read_contents(src)?;
    let txt_urls = txt.lines();
    for txt_url in txt_urls {
        queue.add(txt_url);
    }

    let dest = open_file_dialog_for_dir();

    let urls = queue.urls;
    let res = run(urls, dest)?;
    Ok(res)
}
