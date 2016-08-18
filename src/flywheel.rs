#![allow(unused_imports, dead_code)]

use std::sync::mpsc::{self, Sender, Receiver};
use bismit::{Cortex, LayerMapSchemeList, AreaSchemeList};
use ::{Control, Result};

pub struct Flywheel {
    cortex: Cortex,
    control_rx: Receiver<Control>,
    result_tx: Sender<Result>,
}

impl Flywheel {
    pub fn new(lm_schemes: LayerMapSchemeList, a_schemes: AreaSchemeList,
                control_rx: Receiver<Control>, result_tx: Sender<Result>) -> Flywheel {
        let cortex = Cortex::new(lm_schemes, a_schemes, None);

        result_tx.send(Result::Ready).unwrap();

        Flywheel {
            cortex: cortex,
            control_rx: control_rx,
            result_tx: result_tx,
        }
    }

    pub fn spin(&mut self) {
        loop {
            match self.control_rx.recv().unwrap() {
                Control::Exit => break,
                _ => (),
            }
        }
    }
}