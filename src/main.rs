use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};

/// Cherche un motif dans un fichier et affiche les lignes qui le contiennent.
#[derive(Parser)]
struct Cli {
    pattern: String,          // ce que l'on recherche
    path: std::path::PathBuf, // le fichier dans lequel on recherche
}

// Just like calling .unwrap() is a shortcut for the match with panic! in the error arm, we have another shortcut for the match that returns in the error arm: ?.

fn main() -> Result<()> {
    // TODO: explorer d'autres alternatives à env_logger
    env_logger::init();
    info!("starting up");
    let args = Cli::parse();

    // le context permet de donner plus d'information sur l'erreur
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    // tu peux utiliser ? si tu as un valeur de retour Result
    // rust va faire le match qui on a fait plus haut et renvoyer l'erreur si il y en a une

    let mut found = false;
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
            found = true;
        }
    }

    if !found {
        warn!("pattern `{}` not found", args.pattern);
    } else {
        info!("done!");
    }
    Ok(())
}
