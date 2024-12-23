#![allow(dead_code)]
#![allow(unused_macros)]

use std::{
    io::{Stdout, Write, stdout},
    thread::sleep,
    time::Duration,
};

pub struct DelayingCounter {
    cnt: usize,
    delay_micro: u64,
    print_interval: usize,
    stdout: Stdout,
    char: char,
}

impl DelayingCounter {
    pub fn new(char: char, delay_nanos: u64, print_interval: usize) -> Self {
        let stdout = stdout();
        Self {
            cnt: 0,
            delay_micro: delay_nanos,
            print_interval,
            stdout,
            char,
        }
    }
    pub fn increment(&mut self) {
        self.cnt += 1;
        sleep(Duration::from_micros(self.delay_micro));
        if (self.cnt % self.print_interval) == 0 {
            print!("{}", self.char);
            self.stdout.flush().unwrap();
        }
    }
}

macro_rules! count_slow {
    ($x:expr) => {
        use crate::DelayingCounter;
        use std::sync::LazyLock;
        use std::sync::Mutex;

        static COUNTER: LazyLock<Mutex<DelayingCounter>> =
            LazyLock::new(|| Mutex::new(DelayingCounter::new($x, 1000, 100)));
        if let Ok(mut mg) = COUNTER.lock() {
            mg.increment();
        }
    };
}

macro_rules! count_fast {
    ($x:expr) => {
        use crate::DelayingCounter;
        use std::sync::LazyLock;
        use std::sync::Mutex;

        static COUNTER: LazyLock<Mutex<DelayingCounter>> =
            LazyLock::new(|| Mutex::new(DelayingCounter::new($x, 0, 100)));
        if let Ok(mut mg) = COUNTER.lock() {
            mg.increment();
        }
    };
}

pub mod d00;
pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d05;
pub mod d06;
pub mod d07;
pub mod d08;
pub mod d09;
pub mod d10;
pub mod d11;
pub mod d12;
pub mod d13;
pub mod d14;
pub mod d15;
pub mod d16;
pub mod d17;
pub mod d18;
pub mod d19;
pub mod d20;
pub mod d21;
pub mod d22;
