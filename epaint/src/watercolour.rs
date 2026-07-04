// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::rc::Rc;

use colour_math::{HCV, LightLevel};
use colour_math_derive::Colour;

use crate::paint::{PaintEssentialsIfce, PropertiedPaint, PropertiedPaintPlus, SerializablePaintData};
use crate::properties::PropertyType;
use crate::series::*;
use crate::realize_propertied_paint_plus;
use crate::TooltipText;

realize_propertied_paint_plus!(Watercolour, &[PropertyType::Transparency]);
