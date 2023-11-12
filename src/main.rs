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
use util::file_utils::{open_file_dialog, open_file_dialog_for_dir, read_contents};

use crate::queue::Queue;

mod download;
mod queue;
mod util;
mod websites;

fn main() {
    let src = open_file_dialog();
    let mut queue = Queue::new();

    let txt = read_contents(src.to_owned())
        .expect(format!("Failed to read contents of '{:?}'", src).as_str());
    let txt_urls = txt.lines();
    for txt_url in txt_urls {
        queue.add(txt_url);
    }

    let dest = open_file_dialog_for_dir();
    let urls = queue.urls;

    run(urls, dest).expect("Failed to download");
}
