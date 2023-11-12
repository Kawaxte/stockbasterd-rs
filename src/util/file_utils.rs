use rfd::FileDialog;

pub fn read_contents(path: std::path::PathBuf) -> std::io::Result<String> {
    println!("Reading contents of '{}'...", path.display());

    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}

pub fn open_file_dialog() -> std::path::PathBuf {
    println!("Opening file dialog... (1 of 2)");

    let src = match FileDialog::new()
        .add_filter("TEXT (*.txt)", &["txt"])
        .pick_file()
    {
        Some(file) => file,
        None => {
            return std::path::PathBuf::new();
        }
    };
    src
}

pub fn open_file_dialog_for_dir() -> std::path::PathBuf {
    println!("Opening file dialog... (2 of 2)");

    let dest = match FileDialog::new().pick_folder() {
        Some(dir) => dir,
        None => {
            return std::path::PathBuf::new();
        }
    };
    dest
}
