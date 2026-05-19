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
    pub fn pages_to_vec(&self) -> Vec<i32> {}

    fn previus_generate(&self) -> Vec<i32> {
        (1..self.page).
    }

    fn next_generate(&self) -> Vec<i32> {}
}
