mod identicon;
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
    println!("GRID ===> {:?}", identicon.grid);
    // identicon.filter_odd_squares();
    // println!("FILTER ===> {:?}", identicon.grid);
    // identicon.build_pixel_map();
    // println!("PIXEL MAP ===> {:?}", identicon.pixel_map);
    // identicon.draw_image();

    // println!("HELLO   ===>>>{:?}", (7 % 5) * 50)
    // hash_input("banana");
}
