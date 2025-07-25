//@ build-fail
//@ compile-flags: -Copt-level=0

fn main() {
    rec(Empty);
}

struct Empty;

impl Iterator for Empty {
    type Item = ();
    fn next<'a>(&'a mut self) -> core::option::Option<()> {
        None
    }
}

fn identity<T>(x: T) -> T {
    x
}

fn rec<T>(mut it: T)
where
    T: Iterator,
{
    if () == () {
        T::count(it);
    } else {
        rec(identity(&mut it))
        //~^ ERROR reached the recursion limit while instantiating
    }
}

// https://github.com/rust-lang/rust/issues/67552
