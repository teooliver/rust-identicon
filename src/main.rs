mod identicon;
fn main() {
    let mut identicon = identicon::Identicon::new();
    identicon.hash_input("banana");
    identicon.pick_color();
    identicon.build_grid();
    identicon.remove_odd_squares();
    identicon.build_pixel_map();
    identicon.paint_pixels();
}
