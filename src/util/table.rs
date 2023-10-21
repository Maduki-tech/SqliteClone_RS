// TODO: CHECK HOW THIS WORKS
// INDEX
const ID_SIZE: usize = 4;
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;
const ID_OFFSET: usize = 0;
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
pub const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

const PAGE_SIZE: usize = 4096;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_PAGES: usize = 100;
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;


#[derive(Debug)]
pub struct Row {
    id: u32,
    username: String,
    email: String,
}

pub struct Table {
    pub num_rows: usize,
    pub pages: Vec<Vec<u8>>,
}

impl Default for Row {
    fn default() -> Self {
        Self {
            id: 0,
            username: String::new(),
            email: String::new(),
        }
    }
}

pub fn serialize_row(source: &Row, destination: &mut [u8]) {
    destination[ID_OFFSET..ID_OFFSET + ID_SIZE].copy_from_slice(&source.id.to_le_bytes());
    let username_len = source.username.len();
    destination[USERNAME_OFFSET..USERNAME_OFFSET + username_len]
        .copy_from_slice(source.username.as_bytes());
    let email_len = source.email.len();
    destination[EMAIL_OFFSET..EMAIL_OFFSET + email_len].copy_from_slice(source.email.as_bytes());
}

fn deserialize_row(source: &[u8], destination: &mut Row) {
    destination.id = u32::from_le_bytes(source[ID_OFFSET..ID_OFFSET + ID_SIZE].try_into().unwrap());
    destination.username =
        String::from_utf8(source[USERNAME_OFFSET..USERNAME_OFFSET + USERNAME_SIZE].to_vec())
            .unwrap();
    destination.email =
        String::from_utf8(source[EMAIL_OFFSET..EMAIL_OFFSET + EMAIL_SIZE].to_vec()).unwrap();
}

pub fn row_slot(table: &mut Table, row_num: &mut usize) -> &mut [u8] {
    let page_num = *row_num / ROWS_PER_PAGE;
    let page = &mut table.pages[page_num];
    if page.is_empty() {
        page.resize(PAGE_SIZE, 0);
    }
    let row_offset = *row_num % ROWS_PER_PAGE;
    let byte_offset = row_offset * ROW_SIZE;
    &mut page[byte_offset..byte_offset + ROW_SIZE]
}
