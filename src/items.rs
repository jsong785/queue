use std::collections::VecDeque;

pub struct Items {
    items: VecDeque<String>,
}
impl Items {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn get(&mut self) -> Option<String> {
        self.items.pop_front()
    }

    pub fn add(&mut self, item: String) {
        self.items.push_back(item);
    }
}
