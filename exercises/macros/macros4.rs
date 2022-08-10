// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ( $val:expr ) => {
        println!("Look at this other macro: {}", $val);
    };
    ( $($val:expr),* ) => {
        let mut sum = 0;
        $(
            sum += $val;
        )*
        println!("Sum is {}", sum);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
    my_macro!(1, 2);

}
