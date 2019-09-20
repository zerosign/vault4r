use serde::de::{self, Visitor};
use std::fmt;

#[derive(Deserialize)]
pub struct ListLeaseData {
    #[serde(flatten)]
    keys: Vec<String>,
}

//!
//! { "data" ; { "keys" : [...]} } => should be vec![...]
//!


// impl<'de> Visitor<'de> for ListLeaseDataVisitor {
//     type Value = Vec<String>;

//     fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         fmt.write_str("json should have field {\"data\" : {\"keys\" : [...]}}");
//     }

//     fn visit_
// }
