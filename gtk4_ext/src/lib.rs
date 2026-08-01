// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

#[macro_export]
macro_rules! yield_to_pending_events {
    ( ) => {
        while false {
            // TODO: fix or discontinue use of this macro
        }
    };
}

pub static UNEXPECTED: &str = "Unexpected error: please inform <pwil3058@bigpond.net.au>";

pub mod gtkx;
pub mod pwo;
