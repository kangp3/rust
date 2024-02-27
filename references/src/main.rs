fn main() {
    let mut s = String::from("hello");
    let s1 = &mut s;

    s1.push_str(", world");

    println!("s: {s}, s1: {s1}");
}

// error[E0502]: cannot borrow `s` as immutable because it is also borrowed as mutable
//  --> src/main.rs:7:18
//   |
// 3 |     let s1 = &mut s;
//   |              ------ mutable borrow occurs here
// ...
// 7 |     println!("s: {s}, s1: {s1}");
//   |                  ^^^      ---- mutable borrow later used here
//   |                  |
//   |                  immutable borrow occurs here
//   |
//   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// For more information about this error, try `rustc --explain E0502`.
// error: could not compile `references` (bin "references") due to previous error
