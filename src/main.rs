mod identicon;
fn main() {
    let mut identicon = identicon::Identicon {
        hex: vec![],
        color: vec![],
        grid: vec![],
        pixel_map: vec![],
    };
    // identicon.hash_input("banana");
    // identicon.pick_color();
    // identicon.build_grid();
    // println!("{:?}", identicon.grid);
    // identicon.filter_odd_squares();
    // println!("{:?}", identicon.grid);

    // identicon.draw_image();

    println!("HELLO   ===>>>{:?}", (7 % 5) * 50)
    // hash_input("banana");
}
