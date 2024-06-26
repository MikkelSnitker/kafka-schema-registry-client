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

/// Schema : Schema
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    /// Name of the subject
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Version number
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// Globally unique identifier of the schema
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Schema type
    #[serde(rename = "schemaType", skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<String>,
    /// References to other schemas
    #[serde(rename = "references", skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<models::SchemaReference>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::Metadata>>,
    #[serde(rename = "ruleset", skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<Box<models::RuleSet>>,
    /// Schema definition string
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "ruleSet", skip_serializing_if = "Option::is_none")]
    pub rule_set: Option<Box<models::RuleSet>>,
}

impl Schema {
    /// Schema
    pub fn new() -> Schema {
        Schema {
            subject: None,
            version: None,
            id: None,
            schema_type: None,
            references: None,
            metadata: None,
            ruleset: None,
            schema: None,
            rule_set: None,
        }
    }
}

