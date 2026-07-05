// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::rc::Rc;

use colour_math::hue_wheel::{ColouredShape, MakeColouredShape, Shape};
use colour_math::{HCV, LightLevel};
use colour_math_derive::Colour;

use crate::create_paint;
use crate::paint::{Paint, SerializablePaintData};
use crate::properties::{Property, PropertyType};
use crate::{LabelText, TooltipText};
use crate::{PaintEssence, SeriesId};

create_paint!(Watercolour, &[PropertyType::Transparency]);
