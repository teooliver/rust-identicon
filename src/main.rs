// use hex_literal::hex;
mod identicon;
// use crate::identicon;
fn main() {
    let mut identicon = identicon::Identicon {
        hex: vec![],
        color: vec![],
        grid: vec![],
        pixel_map: vec![],
    };
    identicon.hash_input("banana");
    identicon.pick_color();
    identicon.build_grid();
    println!("{:?}", identicon);

    // println!("{:?}", identicon)
    // hash_input("banana");
}
