/// The borrow checker is a feature of the Rust language that checks that references are valid.
///
/// Example:
///
/// `foo` is a mutable variable, and `bar` is a reference to `foo`. `baz` is a mutable reference to `foo`.
///
/// But `bar` and `baz` cannot coexist because `bar` is a reference to `foo` and `baz` is a mutable reference to `foo`.
///
/// The code below works right now because `bar` is not used.
///
/// If we uncomment the line `println!("bar: {bar}");`, we will get the following error:
///
/// cannot borrow `foo` as mutable because it is also borrowed as immutable
#[allow(dead_code, unused, clippy::disallowed_names)]
fn borrow_checker() {
    let mut foo = 5;

    let bar = &foo;
    let baz = &mut foo;

    // println!("bar: {bar}");
    println!("baz: {}", baz);
}

fn main() {
    borrow_checker();
}
