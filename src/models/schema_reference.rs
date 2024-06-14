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

/// SchemaReference : Schema reference
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaReference {
    /// Reference name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Name of the referenced subject
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Version number of the referenced subject
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

impl SchemaReference {
    /// Schema reference
    pub fn new() -> SchemaReference {
        SchemaReference {
            name: None,
            subject: None,
            version: None,
        }
    }
}

