mod identicon;
fn main() {
    let mut identicon = identicon::Identicon {
        hex: vec![],
        color: [0, 0, 0],
        grid: vec![],
        pixel_map: vec![],
    };
    identicon.hash_input("banana");
    identicon.pick_color();
    identicon.build_grid();
    identicon.remove_odd_squares();
    identicon.build_pixel_map();
    identicon.paint_pixels();
}
