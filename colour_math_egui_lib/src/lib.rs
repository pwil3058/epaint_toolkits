// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

pub mod colour;
pub mod components;
pub mod egui_drawer;
pub mod widgets;
pub mod drawer;
// Converation traits for where From/Into forbidden.
// Use for traits to make direction of conversion clear.

pub trait FromColourMath<T>: Sized {
    fn from_cm(arg: T) -> Self;
}

pub trait ColourMathInto<T>: Sized {
    fn cm_into(self) -> T;
}

impl<S, T: FromColourMath<S>> ColourMathInto<T> for S {
    fn cm_into(self) -> T {
        T::from_cm(self)
    }
}

pub trait ColourMathFrom<T>: Sized {
    fn cm_from(arg: T) -> Self;
}

pub trait IntoColourMath<T>: Sized {
    fn into_cm(self) -> T;
}

impl<S, T: ColourMathFrom<S>> IntoColourMath<T> for S {
    fn into_cm(self) -> T {
        T::cm_from(self)
    }
}
