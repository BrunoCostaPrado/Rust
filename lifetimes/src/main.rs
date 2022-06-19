use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!(
        "{}",
        longest_with_an_announcement("hello world", "hello rust", "4")
    );
}

/* ' indica uma lifetime

 &i32         a reference
 &'a i32      a reference with an explicit lifetime
 &'a mut i32  a mutable reference with an explicit lifetime

1. Each parameter that is a reference gets its own lifetime.

2. If there is exactly one input lifetime parameter, that lifetime is assigne to all output lifetime parameters.

3. If there are multiple input lifetime parameters, but one of them is &self or
&mut self the lifetime of seld is assigned to all output lifetime parameters.

&static - create an static lifetime for the duration of the program.
*/
