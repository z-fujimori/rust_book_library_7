use my_library::library::{book::Book, bookshelf::Bookshelf};

fn main() {
    let mut shelf = Bookshelf::new();
    let book1 = Book::new("GPTです", "山田ら浪");
    let book2 = Book::new("APIです", "山田ら浪");
    let book3 = Book::new("Rust学習本", "葛西太郎");
    shelf.add_book(book1);
    shelf.add_book(book2);
    shelf.add_book(book3);

    let found_books = shelf.search_books("GPT");
    println!("{:?}", found_books);
}
