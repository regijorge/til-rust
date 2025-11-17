# Formatted print

Printting is handled by a series of macores defined in `std:fmt`

- format!: write formatted text to String
- print!: same as format! but the text is printed to the console (io::stdout).
- println!: same as print! but a newline is appended.
- eprint!: same as print! but the text is printed to the standard error (io::stderr).
- eprintln!: same as eprint! but a newline is appended.

In general, the `{}`will be automatically replaced by any argument. these will be stringified.
 
```
fn main() {
    // In general, the `{}`will be automatically replaced by any argument. these will be stringified.
    println!("{} days", 21);
    
    // Positional argments can be used specifying an integer inside {}
    println!("{0}. This is {0}, this is {1} | {1}-{0}", "First", "Second");
    
    // Also named argumenst
    println!("{subject} {verb} {object}",
             object="The lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
             
    // Add whitespaces. In this example 5
    println!("{number:>5}", number=1);
    
    // Pad numbers with extra zero
    println!("{number:0>5}", number=1);
    
    // Using $
    println!("{number:0>width$}", number=1, width=5);
}
```