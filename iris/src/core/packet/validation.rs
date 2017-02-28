use std::string::String as String;
use std::str;

use common::name as name;
use common::name::Name as Name;
use core::datastructure::identifier;
use core::packet::message;

pub struct Validation {
    type: message::ValidationType,
    // XXX: need to store the validation-dependent data
}
