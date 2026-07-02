// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::rc::Rc;

use colour_math::{HCV, LightLevel};
use colour_math_derive::Colour;

use crate::paint::{PaintEssentialsIfce, PaintSpec, PropertiedPaint};
use crate::properties::PropertyType;
use crate::series::*;
use crate::{
    declare_propertied_paint_struct, impl_eq_for_paint, impl_from_paint_spec, impl_ord_for_paint,
    impl_paint_essential_ifce, implement_propertied_paint,
};

declare_propertied_paint_struct!(WaterColour);
impl_paint_essential_ifce!(WaterColour);
implement_propertied_paint!(WaterColour, &[PropertyType::Transparency]);
impl_eq_for_paint!(WaterColour);
impl_ord_for_paint!(WaterColour);
impl_from_paint_spec!(WaterColour);
