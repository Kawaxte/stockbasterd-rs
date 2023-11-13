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

use download::run;
use util::file_utils::{read_contents_of, select_dir, select_text_file};

use crate::queue::Queue;

mod download;
mod queue;
mod util;
mod websites;

fn main() {
    let src = select_text_file();
    let mut queue = Queue::new();

    let txt = read_contents_of(src.to_owned())
        .expect(format!("Failed to read contents of '{:?}'", src).as_str());

    let txt_urls = txt.lines();
    for txt_url in txt_urls {
        println!("Adding '{}'", txt_url);
        queue.add(txt_url);
    }

    let queue_size = queue.urls.len();
    println!("Added {} URLs", queue_size);

    let dest = select_dir();
    let urls = queue.urls;
    run(urls, dest).expect("Failed to download file(s)");
}
