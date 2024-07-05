use std::error::Error;
use std::fs;
use std::env;


pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
/*The struct is the memory instance and the impl is the usage of the memory instance.  The function inside the 
impl is the function related to the memory instance.  Think kind of like a class and the functions associated
with the class.  Idiomatic Rust has an impl for each struct, I believe. */

/* impl Config {
    pub fn build(mut args: impl Iterator<Item = String>, ) -> Result<Config, &'static str> {
            if args.len() < 3 { //the input needs at least three elements to be successful
                return Err("Not enough arguments. Please try again.");
            }
            /*Updating the signature of the build function to be mut args: impl Iterator<Item = String>, the env:args function
            in main returns a type that implements the Iterator trait and returns String values. This means the args can be 
            any type that implemnts the Iterator trait and returns String items. This function is taking ownership of args
            and mutating it by iterating over it. I am re-writing the impl Config below.*/
            let query = args[1].clone(); //the program's name takes args[0] location; need to start at index 1.
            let file_path = args[2].clone();
            let ignore_case = env::var("IGNORE_CASE").is_ok();
            /* We have created a new variable ignore_case and set its value to the env::var function.  The function
            returns a Result that will be the successful OK variant that contains the value of the environment variable,
            if it is set to any value.  Or return an error if it is not set.  We do not care what the value is, as long
            as the value is set or unset.  The is_ok method returns false if unset and true if set.  During the impl of the
            Config and the build function, we pass the IGNORE_CASE input into the struct which is then read by the 
            run function to determine which search function to call. */
    
            Ok(Config {query, file_path, ignore_case}) //no semi-colon because this is the output
            /* Notice here that all three items from the structure must be returned for the Ok to be satisfied. */
        }
} */

/*Here is the re-written impl Config below */

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>, ) -> Result<Config, &'static str> {
        args.next(); //calling this to skip over the first element of the env::args which is the name of the program

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    /*the fs::read_to_string takes the file path, opens the file, and returns a std::io::Result<String> of the
    files contents.  The Box<dyn Error> implements the Error trait bu we don't specify the particular type the 
    return value will be. The ? propogates the error from the function for Main to handle. */
    
    //println!("With text:\n{}", contents); //temporary println! statement

    
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    /* the code above puts the results of the search, if the ignore_case item is true, into the results after calling
    the search_case_insensitive function.  If the ignore_case item is false, then it uses the regular search function.
    The results are then iterated through using the for loop and prints out the lines.*/
    Ok(()) //return the unit () type
}

/*Re-writing the search function to use iterator adaptor methods */
/* pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> { //case sensitive
    
    let mut results = Vec::new(); //create a new vector to store the results of the search

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results //no semi-colon because this is the output.
} */
/*the search function has a lifetime parameter 'a which specifies that the contents are the items that need to 
remain in memory until the search function is closed. Without the lifetime parameter, the complier will throw an 
error. Rust requires that you connect arguments to return values. This way the search function knows that the 
contents input is what will drive the Vec output. */

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

/*The search function above takes advantage of the iterator adaptor methods. Note how there is no need for a
mutable vector to be created before anything is done. The collect() method creates the vector and puts all of
the results from the lines() method and filter() closure into the contents vector. */

/*Re-writing the function below to use the same iterator adaptors */
/* pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results //no semi-colon because this is the output.
} */
/* the case insensitive search function has the same signature as the search function above it but the guts are
slightly different. In this function, the query is modified to be all lowercase as well as the lines in the contents
of the search item. The rest of the function is almost the same as the case sensitive search function. The query.to_lowercase()
creates a string that is referenced by the contains method.  This is because the old way was using a string slice and this
way is using a full string.  The to_lowercase creates new data instead of referencing data.  */

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.to_lowercase().contains(query.to_lowercase())).collect()
}
/*The function above uses iterator adaptors to make the code concise.  The contents will iterate through the lines
and ensure the input is all lower case and filter for the query and then collect the matching lines into a vector */

#[cfg(test)]
mod tests {
    use super::*;

    #[test] //this test searches for the string duct
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust: 
safe, fast, productive.
Pick three.
Duct tape"; //the first \ tells rust to not put a newline character at the beginning of the contents

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}