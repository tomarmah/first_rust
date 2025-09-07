use rand::rand_core::le;

pub fn try_out() {
    println!("Hello, world, I am from formatted_print!");
    println!("Number: {}", 42);

    //Positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    //Named arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    //Special formatting
    //by specifying formatting characters after a colon within the curly braces
    println!("Base 10:               {}", 69420);
    println!("Base 2 (binary):       {:b}", 69420);
    println!("Base 8 (octal):        {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);

    //Alignment and padding
    println!("Number: {:>8}", 42); // Right-align in a field of width 8
    println!("Number: {:<8}", 42); // Left-align in a field of width 8
    println!("Number: {:^8}", 42); // Center-align in a field of width 8
    println!("Number: {:0>8}", 42); // Pad with zeros on the left to width 8
    println!("Number: {:*^8}", 42); // Pad with '*' on both sides to width 8

    //Floating-point precision
    println!("Pi: {:.2}", 3.141592653589793); // Two decimal places
    println!("Pi: {:.5}", 3.141592653589793); // Five decimal places

    //Debug formatting
    println!("Debug: {:?}", (3, "hello", 4.5));

    //Pretty-printing with Debug
    println!("Pretty Debug: {:#?}", (3, "hello", 4.5));

    //Using `format!` to create a formatted string
    let formatted_string = format!("Hello, {}!", "world");
    println!("{}", formatted_string);

    //An unprintable struct (doesn't implement Display
    struct Unprintable(i32);
    // println!("This struct is {}", Unprintable(3)); // This line would cause a compile-time error

    //A printable struct (implements Display)
    #[derive(Debug)]
    struct Printable(i32);

    println!("This struct is {:?}", Printable(3));

    #[derive(Debug)]
    struct Person<'a, 'b> {
        name: &'a str,
        color: &'b str,
        age: u8,
        height: u8,
    }

    let name = "Peter";
    let age = 27;
    let color = "blue";
    let height = 180;
    let peter = Person {
        age,
        color,
        name,
        height,
    };
    println!("{:#?}", peter);
}
