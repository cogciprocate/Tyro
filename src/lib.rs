#![allow(unused_variables)]

#[macro_use] extern crate enum_primitive;
extern crate libc;
extern crate num;
extern crate bismit;

// use libc::c_void;
use std::fmt::Debug;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, SyncSender, Receiver};
use num::ToPrimitive;
use enum_primitive::FromPrimitive;
use bismit::{LayerMapSchemeList, AreaSchemeList, TypeId};
use bismit::flywheel::{Flywheel, Command, Request, Response, SensoryFrame, MotorFrame};

pub mod config;


#[allow(dead_code)]
pub struct Tyro {
    reward: f32,
    th_flywheel: JoinHandle<()>,
    command_tx: Sender<Command>,
    request_tx: Sender<Request>,
    response_rx: Receiver<Response>,
    sensory_tx: SyncSender<SensoryFrame>,
    motor_rx: Receiver<MotorFrame>,
}

impl Tyro {
    pub fn new(lm_schemes: LayerMapSchemeList, a_schemes: AreaSchemeList) -> Tyro {
        let (command_tx, command_rx) = mpsc::channel();
        let (request_tx, request_rx) = mpsc::channel();
        let (response_tx, response_rx) = mpsc::channel();
        let (sensory_tx, sensory_rx) = mpsc::sync_channel(1);
        let (motor_tx, motor_rx) = mpsc::sync_channel(1);

        let th_flywheel = thread::Builder::new().name("flywheel".to_string()).spawn(move || {
            let mut flywheel = Flywheel::from_blueprint(command_rx, lm_schemes,
                a_schemes, None);
            flywheel.add_req_res_pair(request_rx, response_tx);
            // flywheel.add_sen_mot_pair(sensory_rx, motor_tx);
            flywheel.add_sensory_rx(sensory_rx, "v0b".to_owned());
            flywheel.add_motor_tx(motor_tx);
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
            sensory_tx: sensory_tx,
            motor_rx: motor_rx,
        }
    }

    #[inline]
    pub fn default() -> Tyro {
        Tyro::new(config::define_lm_schemes(), config::define_a_schemes())
    }

    pub fn cycle(&self) {
        self.command_tx.send(Command::Iterate(1)).unwrap()
    }

    pub fn add_reward(&mut self, reward: f32) -> f32 {
        self.reward += reward;
        self.reward
    }

    pub fn reward(&self) -> f32 {
        self.reward
    }

    pub fn push_vec_frame(&self, ptr: *const libc::c_void, type_id: i32, dims: &[i64; 2]) {
        let len = (dims[0] * dims[1]) as usize;

        let f32_array16 = match TypeId::from_i32(type_id).expect("print_array(): Invalid type_id.") {
            TypeId::Float32 => to_f32_arr(ptr as *const f32, len),
            TypeId::Float64 => to_f32_arr(ptr as *const f64, len),
            TypeId::Int32 => to_f32_arr(ptr as *const i32, len),
            TypeId::Int64 => to_f32_arr(ptr as *const i64, len),
        };

        self.sensory_tx.send(SensoryFrame::F32Array16(f32_array16)).unwrap();
    }

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

fn to_f32_arr<T: ToPrimitive>(ptr: *const T, len: usize) -> [f32; 16] {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let mut f32_array16 = [0.0f32; 16];

    for (i, val) in slice.iter().enumerate() {
        f32_array16[i] = val.to_f32().unwrap_or(0.0);
    }

    f32_array16
}

fn print_something<T: Debug>(ptr: *const T, len: usize) {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    println!("Array Value: {:?}", slice);
}


// ##########################################
// ############## FFI STUFF #################
// ##########################################

#[no_mangle]
pub extern "C" fn push_vec_frame(tyro: &Tyro, ptr: *const libc::c_void, type_id: i32,
            dims: &[i64; 2]) {
    tyro.push_vec_frame(ptr, type_id, dims);
}

#[no_mangle]
pub extern "C" fn cycle(tyro: &Tyro) {
    tyro.cycle();
}

#[no_mangle]
pub extern "C" fn print_array(ptr: *const libc::c_void, type_id: i32, dims: &[i64; 2]) {
    // println!("print_array(): dims: {:?}", dims);
    let len = (dims[0] * dims[1]) as usize;

    let ptr_typed = match TypeId::from_i32(type_id).expect("print_array(): Invalid type_id.") {
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
    println!("Adding reward: {}", reward);
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
    a + 100
}
