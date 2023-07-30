use std::io;

fn get_input() -> io::Result<String> {
    let mut buffer = String::new(); // CREATE A STRING TO CAPTURE INFO
    io::stdin().read_line(&mut buffer)?; // IO::STDIN() SENDS COMMANDS TO CONSOLE, CAPTURE IT IN BUFFER VARIABLE
    Ok(buffer.trim().to_owned())  // TRIM() WHITE SPACE, CONVERT BACK TO OWNED STRING()
}

fn main() {
    let mut all_input = vec![]; // EMPTY VEC TO CAPTURE THE USER INPUT
    let mut times_input = 0; // HOW MANY TIMES USER IS ALLOWED TO INPUT
    while times_input < 2 {
        match get_input() {       // MATCH GET_INPUT(),  WITH RESULT OK() OR ERR()
            Ok(words) => {
                all_input.push(words);
                times_input += 1;
            }
            Err(e) => println!("{:?}", e)
        }
    }
    for input in all_input {
        // FOR LOOP THE VEC AND OUTPUT THE RESULTS
        println!("Original: {:?}, Capitalized: {:?}", input, input.to_uppercase());
    }
}
