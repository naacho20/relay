/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::client_extensions::CLIENT_EXTENSION_DIRECTIVE_NAME;
use crate::connections::ConnectionConstants;
use crate::handle_fields::HandleFieldConstants;
use crate::inline_data_fragment::INLINE_DATA_CONSTANTS;
use crate::match_::MATCH_CONSTANTS;
use crate::refetchable_fragment::CONSTANTS as REFETCHABLE_CONSTANTS;
use crate::INTERNAL_METADATA_DIRECTIVE;

use fnv::FnvHashSet;
use graphql_ir::{Argument, Directive, Value};
use interner::{Intern, StringKey};
use lazy_static::lazy_static;

// A wrapper type that allows comparing pointer equality of references. Two
// `PointerAddress` values are equal if they point to the same memory location.
//
// This type is _sound_, but misuse can easily lead to logical bugs if the memory
// of one PointerAddress could not have been freed and reused for a subsequent
// PointerAddress.
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct PointerAddress(usize);

impl PointerAddress {
    pub fn new<T>(ptr: &T) -> Self {
        let ptr_address: usize = unsafe { std::mem::transmute(ptr) };
        Self(ptr_address)
    }
}

/// This function will return a new Vec[...] of directives,
/// where one will be missing. The one with `remove_directive_name` name
pub fn remove_directive(
    directives: &[Directive],
    remove_directive_name: StringKey,
) -> Vec<Directive> {
    let mut next_directives = Vec::with_capacity(directives.len() - 1);
    for directive in directives {
        if directive.name.item != remove_directive_name {
            next_directives.push(directive.clone());
        }
    }
    next_directives
}

/// Function will create a new Vec[...] of directives
/// when one of them will be replaced with the `replacement`. If the name of
/// `replacement` is matched with the item in the list
pub fn replace_directive(directives: &[Directive], replacement: Directive) -> Vec<Directive> {
    directives
        .iter()
        .map(|directive| {
            if directive.name.item == replacement.name.item {
                return replacement.to_owned();
            }
            directive.to_owned()
        })
        .collect()
}

/// The function that will return a variable name for an argument
/// it it uses a variable (and it the argument is available)
pub fn extract_variable_name(argument: Option<&Argument>) -> Option<StringKey> {
    match argument {
        Some(arg) => match &arg.value.item {
            Value::Variable(var) => Some(var.name.item),
            _ => None,
        },
        None => None,
    }
}

pub struct CustomMetadataDirectives {
    connection_constants: ConnectionConstants,
    handle_field_constants: HandleFieldConstants,
}

impl Default for CustomMetadataDirectives {
    fn default() -> Self {
        Self {
            connection_constants: ConnectionConstants::default(),
            handle_field_constants: HandleFieldConstants::default(),
        }
    }
}

impl CustomMetadataDirectives {
    pub fn is_custom_metadata_directive(&self, name: StringKey) -> bool {
        name == *CLIENT_EXTENSION_DIRECTIVE_NAME
            || name == self.connection_constants.connection_metadata_directive_name
            || name == self.handle_field_constants.handle_field_directive_name
            || name == MATCH_CONSTANTS.custom_module_directive_name
            || name == REFETCHABLE_CONSTANTS.refetchable_metadata_name
            || name == REFETCHABLE_CONSTANTS.refetchable_operation_metadata_name
            || name == *INTERNAL_METADATA_DIRECTIVE
    }
}

lazy_static! {
    static ref RELAY_CUSTOM_INLINE_FRAGMENT_DIRECTIVES: FnvHashSet<StringKey> = vec![
        *CLIENT_EXTENSION_DIRECTIVE_NAME,
        MATCH_CONSTANTS.custom_module_directive_name,
        INLINE_DATA_CONSTANTS.internal_directive_name,
        "defer".intern(),
    ]
    .into_iter()
    .collect();
}

pub fn is_relay_custom_inline_fragment_directive(directive: &Directive) -> bool {
    RELAY_CUSTOM_INLINE_FRAGMENT_DIRECTIVES.contains(&directive.name.item)
}
