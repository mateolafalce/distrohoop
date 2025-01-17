use utils::play_animation;

mod distros;
mod utils;


fn main() {

    let vec = distros::get_distros();

    play_animation(&vec);

    print!("Executed successfully!")
}
