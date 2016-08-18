#![allow(unused_variables)]

#[macro_use] extern crate enum_primitive;
extern crate libc;
extern crate bismit;

// use libc::c_void;
use std::fmt::Debug;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};
use enum_primitive::FromPrimitive;
use bismit::{LayerMapSchemeList, AreaSchemeList, TypeId};
use bismit::flywheel::{Flywheel, Command, Request, Response};

pub mod config;


#[allow(dead_code)]
pub struct Tyro {
    reward: f32,
    th_flywheel: JoinHandle<()>,
    command_tx: Sender<Command>,
    request_tx: Sender<Request>,
    response_rx: Receiver<Response>,
}

impl Tyro {
    pub fn new(lm_schemes: LayerMapSchemeList, a_schemes: AreaSchemeList) -> Tyro {
        // let (command_tx, control_rx) = mpsc::channel();
        // let (result_tx, response_rx) = mpsc::channel();

        // let th_flywheel = thread::Builder::new().name("tyro::flywheel".to_string()).spawn(move || {
        //     let mut flywheel = flywheel::Flywheel::new(lm_schemes, a_schemes, control_rx, result_tx);
        //     flywheel.spin();
        // }).expect("Error creating 'flywheel' thread");

        let (command_tx, command_rx) = mpsc::channel();
        let (request_tx, request_rx) = mpsc::channel();
        let (response_tx, response_rx) = mpsc::channel();

        let th_flywheel = thread::Builder::new().name("flywheel".to_string()).spawn(move || {
            let mut flywheel = Flywheel::from_blueprint(command_rx, lm_schemes,
                a_schemes, None);
            flywheel.add_req_res_pair(request_rx, response_tx);
            flywheel.spin();
        }).expect("Error creating 'flywheel' thread");

        // Wait for the flywheel to initialize bismit:
        request_tx.send(Request::CurrentIter).unwrap();
        match response_rx.recv().unwrap() {
            Response::CurrentIter(_) => (),
            _ => panic!("Tyro::new(): Error initializing flywheel."),
        }

        Tyro {
            reward: 0.0,
            th_flywheel: th_flywheel,
            command_tx: command_tx,
            request_tx: request_tx,
            response_rx: response_rx,
        }
    }

    #[inline]
    pub fn default() -> Tyro {
        Tyro::new(config::define_lm_schemes(), config::define_a_schemes())
    }

    pub fn cycle(&mut self) {
        self.command_tx.send(Command::Iterate(0)).unwrap()
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

    // pub fn motor(&self) ->
}

impl Default for Tyro {
    fn default() -> Tyro { Tyro::default() }
}

impl Drop for Tyro {
    fn drop(&mut self) {
        self.command_tx.send(Command::Exit).unwrap();

        let th_flywheel = std::mem::replace(&mut self.th_flywheel,
            thread::Builder::new().spawn(|| ()).unwrap());

        if let Err(e) = th_flywheel.join() { println!("th_flywheel.join(): Error: '{:?}'", e); }
        // if let Err(e) = self.th_vis.join() { println!("th_vin.join(): Error: '{:?}'", e); }
    }
}


// ##########################################
// ############# MISC STUFF #################
// ##########################################

fn print_something<T: Debug>(ptr: *const T, len: usize) {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    println!("Array Value: {:?}", slice);
}


// ##########################################
// ############## FFI STUFF #################
// ##########################################

#[no_mangle]
pub extern "C" fn print_array(ptr: *const libc::c_void, dims: [i32; 2], type_id: i32) {
    let len = (dims[0] * dims[1]) as usize;

    let ptr_typed = match TypeId::from_i32(type_id).unwrap() {
        TypeId::Float32 => print_something(ptr as *const f32, len),
        TypeId::Float64 => print_something(ptr as *const f64, len),
        TypeId::Int32 => print_something(ptr as *const i32, len),
        TypeId::Int64 => print_something(ptr as *const i64, len),
    };
}

#[no_mangle]
pub extern "C" fn new_tyro() -> Box<Tyro> {
    Box::new(Tyro::default())
}

#[no_mangle]
pub extern "C" fn send_input(ptr: *const libc::c_void, dims: [i32; 2], type_id: i32) {

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


// ##########################################
// ############ BULLSHIT STUFF ##############
// ##########################################

#[no_mangle]
pub extern "C" fn print_int(a: i32) {
    println!("Integer Value: {}", a);
}

#[no_mangle]
pub extern "C" fn print_array_f64(p: *const f64, len: i32) {
    // println!("Array Value: {:?}", *array);
    let slice = unsafe { std::slice::from_raw_parts(p, len as usize) };
    println!("Array Value: {:?}", slice);
}

#[no_mangle]
pub extern "C" fn add_100(tyro: &Tyro, a: i32) -> i32 {
    tyro.add_100(a)
}
