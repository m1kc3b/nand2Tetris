use std::env;

fn main() {
    // The program gets the name of the input source file, say Prog (.vm is mandatory), from the command-line argument.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Aucun fichier fourni !");
        std::process::exit(1);
    }

    // Read the file from data directory or from the path provided
    // Create output file with name of the file

    // Loop over each line of the file
    // parse each line
    // translate each parsed line
    // write command into the output file (with comments)
    
}
