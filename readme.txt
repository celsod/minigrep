This minigrep project is from chapter 12 of the Rust tutorial.  This was a difficult section and I learned a lot from this chapter.
This program shows that the main.rs file will gather the information necessary and call the functions.  The lib.rs file is where all of the logic resides
as well as the tests for the logic.  I have fairly extensive comments throughout the code to help remind me of why the code is written the way it is.
This program also uses an environment variable.  I could have made the environment variable an argument to the command line input by creating another
item inside the struct and then setting the logic inside the run function to have either the environment variable or the command line input to determine
 the case sensitivity.