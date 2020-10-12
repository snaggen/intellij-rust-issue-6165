use std::fmt::Debug;

use core::fmt;
use serde::export::Formatter;
use std::collections::HashMap;

macro_rules! setup_attribute_valueclasses {
    ( $($valueclass:expr ,)*) => {
        impl Debug for AttributeMap {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "AttributeMap {{ ")?;
                for (k,_) in &self.map {
                        $(
                        write!(f, "{} {} ", $valueclass, k)?;
                        )*
                    write!(f,", ")?;
                }
                write!(f," }}")
            }
        }
    }
}

include!("./generated_core.rs");

pub struct AttributeMap {
    map: HashMap<i32, i32>,
}

impl AttributeMap {
    pub fn new() -> Self {
        Self {
           map: Default::default()
        }
    }
}

