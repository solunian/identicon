let fs = require("fs");

let pallete = JSON.parse(fs.readFileSync("palette.json"));

let latte = pallete.macchiato;


for(let color of Object.values(latte.colors)){
    let rgb= color.rgb;
    console.log(`(${rgb.r}, ${rgb.g}, ${rgb.b}),`);
}