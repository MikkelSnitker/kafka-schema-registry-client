# Schema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subject** | Option<**String**> | Name of the subject | [optional]
**version** | Option<**i32**> | Version number | [optional]
**id** | Option<**i32**> | Globally unique identifier of the schema | [optional]
**schema_type** | Option<**String**> | Schema type | [optional]
**references** | Option<[**Vec<models::SchemaReference>**](SchemaReference.md)> | References to other schemas | [optional]
**metadata** | Option<[**models::Metadata**](Metadata.md)> |  | [optional]
**ruleset** | Option<[**models::RuleSet**](RuleSet.md)> |  | [optional]
**schema** | Option<**String**> | Schema definition string | [optional]
**rule_set** | Option<[**models::RuleSet**](RuleSet.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


