use std::fs;
use std::io;
use std::path::Path;

use crate::tokenizer::tokenize;
use crate::compilation_engine::CompilationEngine;

pub fn analyze(source: &str) -> Result<(), io::Error> {
    let path = Path::new(source);

    if path.is_file() {
        // C'est un fichier `.jack`, on le traite directement
        if let Some(ext) = path.extension() {
            if ext == "jack" {
                process_file(path)?;
            }
        }
    } else if path.is_dir() {
        // C'est un dossier, on traite chaque fichier `.jack`
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();

            if let Some(ext) = file_path.extension() {
                if ext == "jack" {
                    process_file(&file_path)?;
                }
            }
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Invalid source path"));
    }

    Ok(())
}

fn process_file(file_path: &Path) -> Result<(), io::Error> {
    // 1. Tokenizer : handle tokens
    let tokens = tokenize(file_path.to_str().unwrap())?;

    // 2. CompilationEngine : handle xml file
    let _engine = CompilationEngine::new(tokens, file_path.to_str().unwrap())?;

    Ok(())
}
