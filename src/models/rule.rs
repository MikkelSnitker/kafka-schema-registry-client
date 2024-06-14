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

/// Rule : Rule
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    /// Rule name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Rule doc
    #[serde(rename = "doc", skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    /// Rule kind
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// Rule mode
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    /// Rule type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The tags to which this rule applies
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Optional params for the rule
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<std::collections::HashMap<String, String>>,
    /// Rule expression
    #[serde(rename = "expr", skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
    /// Rule action on success
    #[serde(rename = "onSuccess", skip_serializing_if = "Option::is_none")]
    pub on_success: Option<String>,
    /// Rule action on failure
    #[serde(rename = "onFailure", skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<String>,
    /// Whether the rule is disabled
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

impl Rule {
    /// Rule
    pub fn new() -> Rule {
        Rule {
            name: None,
            doc: None,
            kind: None,
            mode: None,
            r#type: None,
            tags: None,
            params: None,
            expr: None,
            on_success: None,
            on_failure: None,
            disabled: None,
        }
    }
}
/// Rule kind
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "TRANSFORM")]
    Transform,
    #[serde(rename = "CONDITION")]
    Condition,
}

impl Default for Kind {
    fn default() -> Kind {
        Self::Transform
    }
}
/// Rule mode
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "UPGRADE")]
    Upgrade,
    #[serde(rename = "DOWNGRADE")]
    Downgrade,
    #[serde(rename = "UPDOWN")]
    Updown,
    #[serde(rename = "WRITE")]
    Write,
    #[serde(rename = "READ")]
    Read,
    #[serde(rename = "WRITEREAD")]
    Writeread,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Upgrade
    }
}

