use serde::{Deserialize, Serialize};

macro_rules! setup_attributes {
    ( $($is:ident { $($attr:ident, )* },)*) => {
        $(
        #[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Debug, Clone)]
        #[serde(rename_all = "snake_case")]
        pub enum $is {
            $(
                $attr,
            )*
        }
        )*
    }
}

include!("./generated_attributes.rs");
