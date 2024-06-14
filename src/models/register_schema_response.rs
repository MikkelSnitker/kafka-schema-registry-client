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

/// RegisterSchemaResponse : Schema register response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterSchemaResponse {
    /// Globally unique identifier of the schema
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Version number
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// Schema type
    #[serde(rename = "schemaType", skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<String>,
    /// References to other schemas
    #[serde(rename = "references", skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<models::SchemaReference>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::Metadata>>,
    #[serde(rename = "ruleSet", skip_serializing_if = "Option::is_none")]
    pub rule_set: Option<Box<models::RuleSet>>,
    /// Schema definition string
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

impl RegisterSchemaResponse {
    /// Schema register response
    pub fn new() -> RegisterSchemaResponse {
        RegisterSchemaResponse {
            id: None,
            version: None,
            schema_type: None,
            references: None,
            metadata: None,
            rule_set: None,
            schema: None,
        }
    }
}
