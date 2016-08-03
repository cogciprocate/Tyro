extern crate bismit;

use bismit::{Cortex, LayerMapSchemeList, AreaSchemeList};

pub mod config;

#[repr(C)]
pub struct Tyro {
	cortex: Cortex,
	reward: f32,
}

impl Tyro {
	pub fn new(plmaps: LayerMapSchemeList, pamaps: AreaSchemeList) -> Tyro {
		let cortex = Cortex::new(plmaps, pamaps);

		Tyro {
			cortex: cortex,
			reward: 0.0,
		}
	}

	pub fn from_config() -> Tyro {
		let cortex = Cortex::new(config::define_plmaps(), config::define_pamaps());

		Tyro {
			cortex: cortex,
			reward: 0.0,
		}
	}

	pub fn cortex(&mut self) -> &mut Cortex {
		&mut self.cortex
	}

	pub fn cycle(&mut self) {
		self.cortex.cycle();
	}

	pub fn add_100(&self, a: i32) -> i32 {
		a + 100
	}

	pub fn add_reward(&mut self, reward: f32) -> f32 {
		self.reward += reward;
		self.reward
	}

	pub fn reward(&self) -> f32 {
		self.reward
	}
}


#[no_mangle]
pub extern "C" fn hello(a: i32) {
	println!("Hello with: {}", a);
}

#[no_mangle]
pub extern "C" fn new_tyro() -> Box<Tyro> {
	Box::new(Tyro::from_config())
}

#[no_mangle]
pub extern "C" fn add_100(tyro: &Tyro, a: i32) -> i32 {
	tyro.add_100(a)
}

#[no_mangle]
pub extern "C" fn add_reward(tyro: &mut Tyro, reward: f32) -> f32 {
	tyro.add_reward(reward)
}

#[no_mangle]
pub extern "C" fn get_reward(tyro: &Tyro) -> f32 {
	tyro.reward()
}

#[no_mangle]
pub extern "C" fn drop_tyro(_: Box<Tyro>) {

}