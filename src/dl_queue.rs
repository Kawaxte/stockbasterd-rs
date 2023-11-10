pub struct Queue {
    pub urls: Vec<String>,
}

impl Queue {
    pub fn new() -> Self {
        Queue { urls: Vec::new() }
    }

    pub fn push(&mut self, url: String) {
        if url.starts_with("https://") {
            match self.urls.iter().position(|u| u == &url) {
                Some(index) => {
                    let removed = self.urls.remove(index);
                    println!("'{}' is already in queue.", removed);
                }
                None => {}
            }
            self.urls.push(url);
        } else {
            println!("'{}' is not valid URL.", url);
        }
    }
}
