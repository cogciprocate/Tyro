
extern crate tyro;
use tyro::Tyro;

fn main() {
    let mut tyro = Tyro::from_config();

    tyro.cycle();

}