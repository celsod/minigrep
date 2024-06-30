use std::env;
use std::process;
use minigrep::Config; //bring lib.rs into scope

fn main() {
    let args: Vec<String> = env::args().collect();
    //Need to specify the type of collection we want to occur.  That is why the collect function
    //creates a vector of strings
    /*The first line under main() reads: let args be a vector of type string which is the env::args() iterator and then collect
    the values of the iterator into the args vector. */
    //dbg!(args); commented out because no longer needed
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1); //Let's call this a graceful exit
    });
    /*Calling the impl Config and the associated function new */
    /*unwrap_or_else creates a custom non-panic error handling.  It will unwrap the Ok value or it will display
    the error code that we created. */
    

    println!("Searching for {}", config.query); //query and file_path are related and their purpose is to configure
    //how the program will work.
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) { //calling the public function from lib.rs
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    /*We don't need to unwrap anything, so we instead use the if let Err(e) to display the error that the run
    function may return. */

}