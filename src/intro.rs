/// Reading the Argument Values
///
/// To enable cangrep to read the values of command line arguments we pass to it, we'll need the
/// std::env::args function provided in Rust's standard library. This function returns an iterator
/// of the command line arguments passed to cangrep. Iterators produce a series of values, and we
/// can call the collect method on an iterator to turn it into a collection, such as a vector, that
/// contains all the elements the iterator produces.
///
/// The code in Listing 12-1 allows your cangrep program to read any command line arguments passed
/// to it, and then collect the values into a vector.
///
///     use std::env;
///
///     fn main() {
///         let args: Vec<String> = env::args().collect();
///         dbg!(args);
///     }
///     Listing 12-1: Collecting the command line arguments into a vector and printing them
/// First we bring the std::env module into scope with a use statement so we can use its args
/// function. Notice that the std::env::args function is nested in two levels of modules. In cases
/// where the desired function is nested in more than one module, we've chosen to bring the parent
/// module into scope rather than the function. By doing so, we can easily use other functions from
/// std::env. It's also less ambiguous than adding use std::env::args and then calling the function
/// with just args, because args might easily be mistaken for a function that's defined in the
/// current module.
///
/// The args Function and Invalid Unicode
///
/// Note that std::env::args will panic if any argument contains invalid Unicode. If your program
/// needs to accept arguments containing invalid Unicode, use std::env::args_os instead. That
/// function returns an iterator that produces OsString values instead of String values. We've
/// chosen to use std::env::args here for simplicity because OsString values differ per platform
/// and are more complex to work with than String values.
///
/// On the first line of main, we call env::args, and we immediately use collect to turn the iterator 
/// into a vector containing all the values produced by the iterator. We can use the collect function 
/// to create many kinds of collections, so we explicitly annotate the type of args to specify that we 
/// want a vector of strings. Although you very rarely need to annotate types in Rust, collect is one 
/// function you do often need to annotate because Rust isn't able to infer the kind of collection you 
/// want.
///
/// Finally, we print the vector using the debug macro.
///
/// Notice that the first value in the vector is "target/debug/cangrep", which is the name of our
/// binary. This matches the behaviour of the arguments list in C, letting programs use the name by 
/// which they were invoked in their execution. It's often convenient to have access to the program
/// name in case you want to print it in messages or change the behaviour of the program based on
/// what command line alias was used to invoke the program.
///
/// Saving the Argument Values in Variables
///
/// The program is currently able to access the values specified as command line arguments. Now we
/// need to save the values of the two arguments in variables so we can use the values throughout
/// the rest of the program. We do that in Listing 12-2.
///
///     use std::env;
///
///     fn main() {
///         let args: Vec<String> = env::args().collect();
///
///         let query = &args[1];
///         let file_path = &args[2];
///
///         println!("Searching for {query}");
///         println!("In file {file_path}");
///     }
///     Listing 12-2: Creating variables to hold the query argument and file path argument
/// As we saw when we printed the vector, the program's name takes up the first value in the vector
/// at args[0], so we're starting arguments at index 1. The first argument cangrep takes is the
/// string we're searching for, so we put a reference to the first argument in the variable query.
/// The second argument will be the file path, so we put a reference to the second argument in the
/// variable file_path.
///
/// We temporarily print the values of these variables to prove that the code is working as
/// intended.
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {query}");
    println!("In file {file_path}");
}
