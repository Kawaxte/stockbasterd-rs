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
