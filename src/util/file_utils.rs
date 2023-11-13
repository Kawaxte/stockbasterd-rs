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

use rfd::FileDialog;

pub fn read_contents_of(path: std::path::PathBuf) -> std::io::Result<String> {
    let contents = std::fs::read_to_string(path.to_owned())
        .expect(format!("Failed to read contents of '{:?}'", path).as_str());
    Ok(contents)
}

pub fn select_text_file() -> std::path::PathBuf {
    let src = match FileDialog::new()
        .add_filter("TEXT (*.txt)", &["txt"])
        .pick_file()
    {
        Some(file) => file,
        None => {
            return std::path::PathBuf::new();
        }
    };

    println!("Reading URL(s) from '{}'", src.display());
    src
}

pub fn select_dir() -> std::path::PathBuf {
    let dest = if let Some(dir) = FileDialog::new().pick_folder() {
        dir
    } else {
        let mut user_home = dirs::home_dir().expect("Failed to get user home directory");
        user_home.push(".royalty");
        if !user_home.exists() {
            std::fs::create_dir(user_home.to_owned()).expect("Failed to create directory");
        }
        user_home
    };

    println!("Downloading file(s) to '{}'", dest.display());
    dest
}
