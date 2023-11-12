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

pub struct Queue {
    pub urls: Vec<String>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { urls: Vec::new() }
    }

    pub fn add(&mut self, url: &str) {
        match url.starts_with("https://") {
            true => {
                if let Some(index) = self.urls.iter().position(|u| u == &url) {
                    let removed = self.urls.remove(index);
                    println!("'{}' is already in queue.", removed);
                }
                self.urls.push(url.to_string());
            }
            false => (),
        }
    }
}
