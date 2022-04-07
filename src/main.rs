mod identicon;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the image
    #[clap(short, long)]
    identicon_name: String,
    resolution: String,
}

fn main() {
    let args = Args::parse();
    let identicon_name = args.identicon_name;

    let mut identicon = identicon::Identicon::new();
    identicon.hash_input(&identicon_name);
    identicon.pick_color();
    identicon.build_grid();
    identicon.remove_odd_squares();
    identicon.build_pixel_map();
    identicon.paint_pixels(&identicon_name);
}
