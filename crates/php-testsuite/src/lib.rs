#![cfg_attr(windows, feature(abi_vectorcall))]

// We really shouldn't have to re-import all those things
// into the main module...
use std::collections::HashMap;

use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;

mod types;

use types::*;

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
