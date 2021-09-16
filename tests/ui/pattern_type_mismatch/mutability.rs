#![allow(clippy::all)]
#![warn(clippy::pattern_type_mismatch)]

fn main() {}

fn should_lint() {
    let value = &Some(23);
    if let Some(_) = value {
        ()
    }

    let value = &mut Some(23);
    if let Some(_) = value {
        ()
    }
}

fn should_not_lint() {
    let value = &Some(23);
    if let &Some(_) = value {
        ()
    }
    if let Some(_) = *value {
        ()
    }

    let value = &mut Some(23);
    if let &mut Some(_) = value {
        ()
    }
    if let Some(_) = *value {
        ()
    }
}
