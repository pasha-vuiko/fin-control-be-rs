pub trait Pagination {
    fn get_page(&self) -> u32;
    fn get_limit(&self) -> u32;
    fn get_offset(&self) -> u32 {
        (self.get_page() - 1) * self.get_limit()
    }
}
