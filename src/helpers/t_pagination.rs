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
        if self.total_pages == 0 {
            return vec![1];
        }

        let mut result = (self.page.checked_sub(1).unwrap_or(1)..=self.page + 1)
            .filter(|x| *x <= self.total_pages && *x > 0)
            .collect::<Vec<u32>>();

        result.extend_from_slice(&[1, self.total_pages]);
        result.sort();
        result.dedup();
        result
    }
}
