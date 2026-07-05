// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::rc::Rc;

use colour_math::{HCV, LightLevel};
use colour_math_derive::Colour;

use crate::create_paint;
use crate::{PaintEssence, SeriesId};
use crate::paint::{Paint, SerializablePaintData};
use crate::properties:: {Property, PropertyType};
use crate::{LabelText, TooltipText};

create_paint!(Watercolour, &[PropertyType::Transparency]);
