// http://c.learncodethehardway.org/book/ex10.html

fn main() {
    let states = ["California", "Oregon", "Washington", "Texas"];

    // Using the non macro version of println since we do not need variable interpolation
    std::io::stdio::println("Printing some states!");

    // We will use a for loop to loop through the states array and print it.
    // for loops first declare some variable that will represent the current item
    // you have access to as you loop. You will refer to this variable in the body
    // of the for loop. After the in we use the "range" function to get a range from
    // 0 to the length of states.
    for i in range(0, states.len()) {
        println!("state {}", states[i]);
    }

    std::io::stdio::println("Printing some more states!");
    // Perhaps a more idiomatic way to do this is to use an iterator which is simply
    // a data stucuture that permits iterating. We can get an array's iterator by calling
    // '.iter()' on it.
    for state in states.iter() {
        println!("state {}", state);
    }

    // This function gives us a vector of command line arguments
    // Remember all we know (and need to know for now) are that vectors are
    // just like arrays but are growable.
    let args = std::os::args();

    for arg in args.slice(1, args.len()).iter() {
        println!("arg {}", arg);
    }

    // Remember to run the binary with some command line arguments!
}

// Questions and Exercises:
// 1.) See if you can assign an element from the states array to the argv vector
//     before printing both. Try the inverse.
// 2.) Why do I take a slice of args (skipping the 0th element)? Try removing that
//     and rerunning.
// 3.) Look up the 'break' keyword and see if you can figure out how to get the same
//     functionality as the last for loop without first creating the slice.