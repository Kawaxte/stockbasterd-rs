use rfd::FileDialog;

mod dl;
mod dl_fetch;
mod dl_queue;
mod dl_website;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Opening file dialog... (1 of 2)");

    let src = match FileDialog::new()
        .add_filter("TEXT (*.txt)", &["txt"])
        .pick_file()
    {
        Some(file) => file,
        None => {
            return Ok(());
        }
    };

    let queue = dl::create_queue(&src);
    for url in &queue {
        println!("Adding '{}' to queue", url);
    }
    println!("Added {} URL(s) to queue", queue.len());

    println!("Opening file dialog... (2 of 2)");
    let dest = match FileDialog::new().pick_folder() {
        Some(dir) => dir,
        None => {
            return Ok(());
        }
    };

    let res = dl::process_queue(queue, &dest);
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
