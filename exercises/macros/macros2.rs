// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

fn main() {
    macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

    my_macro!();
}

