//! Tests asserting that types properly roudtrip between rust and php

use std::collections::HashMap;

use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;

#[php_function(name = "ExtPhpRs\\TestSuite\\Types\\passthrough_zval")]
pub(crate) fn passthrough_zval(value: &Zval) -> Zval {
    value.shallow_clone()
}

#[php_function(name = "ExtPhpRs\\TestSuite\\Types\\passthrough_string")]
pub(crate) fn passthrough_string(str: String) -> String {
    str
}

#[php_function(name = "ExtPhpRs\\TestSuite\\Types\\passthrough_str")]
pub(crate) fn passthrough_str(str: &str) -> &str {
    str
}

#[php_function(name = "ExtPhpRs\\TestSuite\\Types\\passthrough_vec_i32")]
pub(crate) fn passthrough_vec_i32(value: Vec<i32>) -> Vec<i32> {
    value
}

#[php_function(name = "ExtPhpRs\\TestSuite\\Types\\passthrough_hashmap_string_i32")]
pub(crate) fn passthrough_hashmap_string_i32(value: HashMap<String, i32>) -> HashMap<String, i32> {
    value
}
