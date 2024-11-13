mod page_header {
    pub struct Hex(usize);
    pub struct Header {
        page_type: str,
        page_number: i64,
        page_size: i16,
        free_space: i16,
        row_count: i64,
        previous_page: hex,
        next_page: hex,
    }
    fn write() {}
    fn read() {}
    fn insert() {}
}
