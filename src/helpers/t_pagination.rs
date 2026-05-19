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
    pub fn pages_to_vec(&self) -> Vec<i32> {
        let mut pages: Vec<i32> = vec![];
        pages.extend(self.previus_generate().iter().map(|x| *x as i32));
        pages.extend(self.next_generate().iter().map(|x| *x as i32));
        pages
    }

    pub fn previus_generate(&self) -> Vec<u32> {
        if self.page <= 4 {
            return (1..self.page).filter(|x| *x > 0).collect();
        }
        (self.page - 4..self.page).filter(|x| *x > 0).collect()
    }

    pub fn next_generate(&self) -> Vec<u32> {
        (self.page + 1..=self.page + 4)
            .filter(|x| *x <= self.total_pages)
            .collect()
    }
}
