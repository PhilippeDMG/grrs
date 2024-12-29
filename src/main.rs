// invocation $ grrs foobar test.txt
use anyhow::{Context, Result};
use clap::Parser;

/// Cherche un motif dans un fichier et affiche les lignes qui le contiennent.
#[derive(Parser)]
struct Cli {
    pattern: String,          // ce que l'on recherche
    path: std::path::PathBuf, // le fichier dans lequel on recherche
}

// fn main() {
// Façon plus basique de récupérer les arguments
// let pattern = std::env::args().nth(1).expect("no pattern given");
// let path = std::env::args().nth(2).expect("no path given");
//
// let args = Cli {
//     pattern,
//     path: std::path::PathBuf::from(path),
// };
// println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
// let args = Cli::parse();
//shortcut:
// let content = std::fs::read_to_string("test.txt").unwrap();
//     let result = std::fs::read_to_string("test.txt");
//     let content = match result {
//         Ok(content) => content,
//         Err(error) => {
//             panic!("Error: {}", error); // panic! est une macro qui arrête le programme
//         }
//     };
//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line);
//         }
//     }
// }

// Just like calling .unwrap() is a shortcut for the match with panic! in the error arm, we have another shortcut for the match that returns in the error arm: ?.

fn main() -> Result<()> {
    let args = Cli::parse();

    // le context permet de donner plus d'information sur l'erreur
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    // tu peux utiliser ? si tu as un valeur de retour Result
    // rust va faire le match qui on a fait plus haut et renvoyer l'erreur si il y en a une

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
