/*
 * Confluent Schema Registry
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ModeUpdateRequest : Mode update request
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModeUpdateRequest {
    /// Schema Registry operating mode
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
}

impl ModeUpdateRequest {
    /// Mode update request
    pub fn new() -> ModeUpdateRequest {
        ModeUpdateRequest {
            mode: None,
        }
    }
}
/// Schema Registry operating mode
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "READWRITE")]
    Readwrite,
    #[serde(rename = "READONLY")]
    Readonly,
    #[serde(rename = "READONLY_OVERRIDE")]
    ReadonlyOverride,
    #[serde(rename = "IMPORT")]
    Import,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Readwrite
    }
}

