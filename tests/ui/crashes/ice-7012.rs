#![allow(clippy::all)]

enum _MyOption {
    None,
    Some(()),
}

impl _MyOption {
    fn _foo(&self) {
        if let &Self::Some(_) = self {}
    }
}

fn main() {}
