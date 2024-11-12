pub mod library;

fn function_1(){
    let shelf = crate::library::bookshelf::Bookshelf::new();
}

fn function_2(){
    use library::bookshelf;
    let shelf = bookshelf::Bookshelf::new();
}

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
