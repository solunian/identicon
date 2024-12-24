use std::hash::Hasher;
use image;

mod hash;

fn main() {
    println!("Hello, world!");

    let seed = "thelosttree";

    let hash = hash::hashfn(seed);

    let mut std_hash = std::hash::DefaultHasher::new();
    std_hash.write(seed.as_bytes());
    let hash_val = std_hash.finish();

    println!("resultant hash: {}", hash_val);

    pixel_icon(hash_val);
}


// macchiato ?
static PALLETTE: [[u8;3]; 16] = [
    [244, 219, 214],
    [240, 198, 198],
    [245, 189, 230],
    [198, 160, 246],
    [237, 135, 150],
    [238, 153, 160],
    [245, 169, 127],
    [238, 212, 159],
    [166, 218, 149],
    [139, 213, 202],
    [145, 215, 227],
    [125, 196, 228],
    [138, 173, 244],
    [183, 189, 248],
    [202, 211, 245],
    [184, 192, 224],
];




fn pixel_icon(hash_val: u64) {
    // we can use a sliding window of 3? 4? bits to index into a color pallette 

    // use bits to determine if pixel is "on" / "off"
    // "on" -> index into pallette
    // "off" -> default color value of something?

    let bitmask = 0b1111;

    let mut palette_idx: [u64; 256] = [0; 256];
    
    for i in 0..128 {
        //todo: when i > 252 its just 0000 but should we care?
        let bits = (hash_val as u128 >> i) & bitmask;
        palette_idx[i] = bits as u64;
        palette_idx[256 - 1 - i] = bits as u64;
    }
    
    let imgw = 8;
    let imgh = 8;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgw, imgh);


    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        // println!(" {x}, {y}" );
        *pixel = image::Rgb(PALLETTE[palette_idx[(x + y * imgw) as usize] as usize]);
    }

    imgbuf.save("identicon.png").unwrap();
}
