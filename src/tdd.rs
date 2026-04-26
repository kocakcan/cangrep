/// Test Driven Development
///
/// 1. Write a test that fails and run it to make sure it fails for the reason you expect.
/// 2. Write or modify just enough code to make the new test pass.
/// 3. Refactor the code you just added or changed and make sure the tests continue to pass.
/// 4. Repeat from step 1.
///
/// Though it's just one of many ways to write software, TDD can help drive code design. Writing the
/// test before you write code that makes the test pass helps to maintain high test coverage
/// throughout the process.
///
/// Writing Code to Pass the Test
///
/// Currently, our test is failing because we always return an empty vector. To fix that and
/// implement search, our program needs to follow these steps:
///
///     1. Iterate through each line of the contents.
///     2. Check whether the line contains our query string.
///     3. If it does, add it to the list of values we're returning.
///     4. If it doesn't, do nothing.
///     5. Return the list of results that match.
///
/// Iterating Through Lines with the lines Method
///
/// Rust has a helpful method to handle line-by-line iteration of strings, conveniently named lines.
///
/// Searching Each Line for the Query
///
/// Next, we'll check whether the current line contains our query string. Fortunately, strings have
/// a helpful method named contains that does this for us!
///
/// Storing Matching Lines
///
/// To finish this function, we need a way to store the matching lines that we want to return. For
/// that, we can make a mutable vector before the for loop and call the push method to store a line
/// in the vector. After the for loop, we return the vector.
///
/// Using the search Function in the run Function
///
/// Now that the search function is working and tested, we need to call search from our run
/// function. We need to pass the config.query value and the contents that run reads from the file
/// to the search function.
