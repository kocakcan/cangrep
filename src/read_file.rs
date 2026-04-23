/// Reading a File
///
/// First we bring in a relevant part of the standard library with a use statement: we need std::fs
/// to handle files.
///
/// In main, the new statement fs::read_to_string takes the file_path, opens that file, and returns
/// a value of type std::io::Result<String> that contains the file's contents.
///
/// After that, we again add a temporary println! statement that prints the value of contents after
/// the file is read, so we can check that the program is working so far.
///
/// But the code has a few flaws. At the moment, the main function has multiple responsibilities:
/// generally, functions are clearer and easier to maintain if each function is responsible for
/// only one idea. The other problem is that we're not handling errors as well as we could. The
/// program is still small, so these flaws aren't a big problem, but as the program grows, it will
/// be harder to fix them cleanly. It's a good practice to begin refactoring early on when
/// developing a program because it's much easier to refactor smaller amounts of code.
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    println!("In file {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}
