use std::ops::Sub;

pub struct TPaginationCore {
    pub total_pages: u32,
    pub page: u32,
}

impl TPaginationCore {
    pub fn new(total_pages: u32, page: u32) -> Self {
        Self { total_pages, page }
    }
}

impl TPaginationCore {
    pub fn pages_to_vec(&self) -> Vec<u32> {
        (self.page.checked_sub(3).unwrap_or(0)..=self.page + 3)
            .filter(|x| *x <= self.total_pages && *x > 0)
            .collect()
    }
}
