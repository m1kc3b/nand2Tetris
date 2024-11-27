use std::env;

fn main() {
    // The program gets the name of the input source file, say Prog.vm, from the command-line argument.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Aucun fichier fourni !");
        std::process::exit(1);
    }
    
}
