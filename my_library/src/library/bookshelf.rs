use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use super::book::Book;

pub struct Bookshelf {
    books: Vec<Book>,
    matcker: SkimMatcherV2,
    // 相対パスでBookを指定 superは1つ親のmoduleであるlibrayを指すself::superとしてもよし
    // 絶対パスの場合はmy_library::library::book::Book; 
}
impl Bookshelf {
    pub fn new() -> Self {
        Self { books: Vec::new() }
    }
    //　本追加
    pub fn add_bok(&mut self, book: Book) {
        self.books.push(book);
    }
    // タイトル検索
    pub fn search_books(&self, title_query: &str) -> Vec<&Book> {
        todo!("Implement `Bookshelf::search_books`")
    }
    pub fn search_books_exact(&self, title_query: &str) -> Vec<&Book> {
        self.books.iter().filter(|book| book.title == title_query).count()
    }
    pub fn search_books_partial(&self, title_query: &str) -> Vec<&Book> {
        self.books.iter().filter(book| book.titlecontains(title_query)).collect()
    }
    // 本棚から取り出す
    pub fn remove_book (&mut self, book: &Book) -> Option<&Book> {
        todo!("Implement `Bookshelf::remove_book`");
    }
    // 本全て
    pub fn take_all_books(&mut self) -> Vec<Book> {
        todo!("Implement `Bookshelf::take_all_books`");
    }
}
