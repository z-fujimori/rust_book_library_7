pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod library {
    mod book{
        pub struct Book {
            title: String,
            author: String,
        }
        impl Book {
            fn new(title: &str, author: &str) -> Self {
                Self {
                    title: title.to_string(),
                    author: author.to_string(),
                }
            }
        }
        pub struct Page {
            pub content: String,
        }

    }
    mod magazin {
        pub struct Page {
            pub content: String,
        }
    }
    mod bookshelf {
        use super::book::Book;
        struct Bookshelf {
            books: Vec<Book>,
            // 相対パスでBookを指定 superは1つ親のmoduleであるlibrayを指すself::superとしてもよし
            // 絶対パスの場合はmy_library::library::book::Book; 
        }
        impl Bookshelf {
            fn new() -> Self {
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
            // 本棚から取り出す
            pub fn remove_book (&mut self, book: &Book) -> Option<&Book> {
                todo!("Implement `Bookshelf::remove_book`");
            }
            // 本全て
            pub fn take_all_books(&mut self) -> Vec<Book> {
                todo!("Implement `Bookshelf::take_all_books`");
            }
        }
    }
}
