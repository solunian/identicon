mod identicon;

use std::hash::Hasher;

fn main() {
    let seed = "thelosttree";

    let mut std_hash = std::hash::DefaultHasher::new();
    std_hash.write(seed.as_bytes());
    let hash_val = std_hash.finish();

    println!("resultant hash: {}", hash_val);

    identicon::gen_github_style("identicons/github_style.png", hash_val);
    identicon::gen_pixel_icon("identicons/pixel_icon.png", hash_val);
}
