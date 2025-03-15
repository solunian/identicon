mod identicon;

use clap::Parser;
use std::hash::Hasher;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CLI {
    /// seed for hash
    seed: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    scale: Option<u32>,
}

fn main() {
    let cli = CLI::parse();

    let mut std_hash = std::hash::DefaultHasher::new();
    std_hash.write(cli.seed.as_bytes());
    let hash_val = std_hash.finish();

    println!("resultant hash from seed `{}`: {}", cli.seed, hash_val);

    let output_fname = if let Some(output) = cli.output {
        &format!("identicons/{}", output)
    } else {
        "identicons/icon.png"
    };

    identicon::gen_github_style(output_fname, hash_val, cli.scale.unwrap_or(1));
    // identicon::gen_pixel_icon("identicons/pixel_icon.png", hash_val);

    println!("created identicons at `{}`", output_fname);
}

// fn main() {
//     let seed = "thelosttree";

//     let mut std_hash = std::hash::DefaultHasher::new();
//     std_hash.write(seed.as_bytes());
//     let hash_val = std_hash.finish();

//     println!("resultant hash: {}", hash_val);

//     identicon::gen_github_style("identicons/github_style.png", hash_val);
//     identicon::gen_pixel_icon("identicons/pixel_icon.png", hash_val);
// }
