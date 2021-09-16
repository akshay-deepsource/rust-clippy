#![warn(clippy::single_match)]

fn dummy() {}

fn single_match() {
    let x = Some(1u8);

    if let Some(y) = x {
        println!("{:?}", y);
    };

    let x = Some(1u8);
    if let Some(y) = x {
        println!("{:?}", y)
    }

    let z = (1u8, 1u8);
    if let (2..=3, 7..=9) = z {
        dummy()
    };

    // Not linted (pattern guards used)
    match x {
        Some(y) if y == 0 => println!("{:?}", y),
        _ => (),
    }

    // Not linted (no block with statements in the single arm)
    match z {
        (2..=3, 7..=9) => println!("{:?}", z),
        _ => println!("nope"),
    }
}

enum Foo {
    Bar,
    Baz(u8),
}
use std::borrow::Cow;
use Foo::*;

fn single_match_know_enum() {
    let x = Some(1u8);
    let y: Result<_, i8> = Ok(1i8);

    match x {
        Some(y) => dummy(),
        None => (),
    };

    match y {
        Ok(y) => dummy(),
        Err(..) => (),
    };

    let c = Cow::Borrowed("");

    match c {
        Cow::Borrowed(..) => dummy(),
        Cow::Owned(..) => (),
    };

    let z = Foo::Bar;
    // no warning
    match z {
        Bar => println!("42"),
        Baz(_) => (),
    }

    match z {
        Baz(_) => println!("42"),
        Bar => (),
    }
}

// issue #173
fn if_suggestion() {
    let x = "test";
    if let "test" = x {
        println!()
    }

    #[derive(PartialEq, Eq)]
    enum Foo {
        A,
        B,
        C(u32),
    }

    let x = Foo::A;
    if let Foo::A = x {
        println!()
    }

    const FOO_C: Foo = Foo::C(0);
    if let FOO_C = x {
        println!()
    }

    if let Foo::A = &&x {
        println!()
    }

    let x = &x;
    if let Foo::A = &x {
        println!()
    }

    enum Bar {
        A,
        B,
    }
    impl PartialEq for Bar {
        fn eq(&self, rhs: &Self) -> bool {
            matches!((self, rhs), (Self::A, Self::A) | (Self::B, Self::B))
        }
    }
    impl Eq for Bar {}

    let x = Bar::A;
    if let Bar::A = x {
        println!()
    }

    // issue #7038
    struct X;
    let x = Some(X);
    if let None = x {
        println!()
    };
}

macro_rules! single_match {
    ($num:literal) => {
        match $num {
            15 => println!("15"),
            _ => (),
        }
    };
}

fn main() {
    single_match!(5);
}
