# Rust API client for openapi

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1
- Package version: v1
- Generator version: 7.6.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*CompatibilityV1Api* | [**test_compatibility_by_subject_name**](docs/CompatibilityV1Api.md#test_compatibility_by_subject_name) | **POST** /compatibility/subjects/{subject}/versions/{version} | Test schema compatibility against a particular schema subject-version
*CompatibilityV1Api* | [**test_compatibility_for_subject**](docs/CompatibilityV1Api.md#test_compatibility_for_subject) | **POST** /compatibility/subjects/{subject}/versions | Test schema compatibility against all schemas under a subject
*ConfigV1Api* | [**delete_subject_config**](docs/ConfigV1Api.md#delete_subject_config) | **DELETE** /config/{subject} | Delete subject compatibility level
*ConfigV1Api* | [**delete_top_level_config**](docs/ConfigV1Api.md#delete_top_level_config) | **DELETE** /config | Delete global compatibility level
*ConfigV1Api* | [**get_subject_level_config**](docs/ConfigV1Api.md#get_subject_level_config) | **GET** /config/{subject} | Get subject compatibility level
*ConfigV1Api* | [**get_top_level_config**](docs/ConfigV1Api.md#get_top_level_config) | **GET** /config | Get global compatibility level
*ConfigV1Api* | [**update_subject_level_config**](docs/ConfigV1Api.md#update_subject_level_config) | **PUT** /config/{subject} | Update subject compatibility level
*ConfigV1Api* | [**update_top_level_config**](docs/ConfigV1Api.md#update_top_level_config) | **PUT** /config | Update global compatibility level
*ContextsV1Api* | [**list_contexts**](docs/ContextsV1Api.md#list_contexts) | **GET** /contexts | List contexts
*DefaultApi* | [**get**](docs/DefaultApi.md#get) | **GET** / | Schema Registry Root Resource
*DefaultApi* | [**get_latest_with_metadata**](docs/DefaultApi.md#get_latest_with_metadata) | **GET** /subjects/{subject}/metadata | Retrieve the latest version with the given metadata.
*DefaultApi* | [**post**](docs/DefaultApi.md#post) | **POST** / | 
*ModesV1Api* | [**delete_subject_mode**](docs/ModesV1Api.md#delete_subject_mode) | **DELETE** /mode/{subject} | Delete subject mode
*ModesV1Api* | [**get_mode**](docs/ModesV1Api.md#get_mode) | **GET** /mode/{subject} | Get subject mode
*ModesV1Api* | [**get_top_level_mode**](docs/ModesV1Api.md#get_top_level_mode) | **GET** /mode | Get global mode
*ModesV1Api* | [**update_mode**](docs/ModesV1Api.md#update_mode) | **PUT** /mode/{subject} | Update subject mode
*ModesV1Api* | [**update_top_level_mode**](docs/ModesV1Api.md#update_top_level_mode) | **PUT** /mode | Update global mode
*SchemasV1Api* | [**get_schema**](docs/SchemasV1Api.md#get_schema) | **GET** /schemas/ids/{id} | Get schema string by ID
*SchemasV1Api* | [**get_schema_only**](docs/SchemasV1Api.md#get_schema_only) | **GET** /schemas/ids/{id}/schema | Get schema by ID
*SchemasV1Api* | [**get_schema_types**](docs/SchemasV1Api.md#get_schema_types) | **GET** /schemas/types | List supported schema types
*SchemasV1Api* | [**get_schemas**](docs/SchemasV1Api.md#get_schemas) | **GET** /schemas | List schemas
*SchemasV1Api* | [**get_subjects**](docs/SchemasV1Api.md#get_subjects) | **GET** /schemas/ids/{id}/subjects | List subjects associated to schema ID
*SchemasV1Api* | [**get_versions**](docs/SchemasV1Api.md#get_versions) | **GET** /schemas/ids/{id}/versions | List subject-versions associated to schema ID
*ServerMetadataV1Api* | [**get_cluster_id**](docs/ServerMetadataV1Api.md#get_cluster_id) | **GET** /v1/metadata/id | Get the server metadata
*ServerMetadataV1Api* | [**get_schema_registry_version**](docs/ServerMetadataV1Api.md#get_schema_registry_version) | **GET** /v1/metadata/version | Get Schema Registry server version
*SubjectsV1Api* | [**delete_schema_version**](docs/SubjectsV1Api.md#delete_schema_version) | **DELETE** /subjects/{subject}/versions/{version} | Delete schema version
*SubjectsV1Api* | [**delete_subject**](docs/SubjectsV1Api.md#delete_subject) | **DELETE** /subjects/{subject} | Delete subject
*SubjectsV1Api* | [**get_referenced_by**](docs/SubjectsV1Api.md#get_referenced_by) | **GET** /subjects/{subject}/versions/{version}/referencedby | List schemas referencing a schema
*SubjectsV1Api* | [**get_schema_by_version**](docs/SubjectsV1Api.md#get_schema_by_version) | **GET** /subjects/{subject}/versions/{version} | Get schema by version
*SubjectsV1Api* | [**get_schema_only2**](docs/SubjectsV1Api.md#get_schema_only2) | **GET** /subjects/{subject}/versions/{version}/schema | Get schema string by version
*SubjectsV1Api* | [**list**](docs/SubjectsV1Api.md#list) | **GET** /subjects | List subjects
*SubjectsV1Api* | [**list_versions**](docs/SubjectsV1Api.md#list_versions) | **GET** /subjects/{subject}/versions | List versions under subject
*SubjectsV1Api* | [**look_up_schema_under_subject**](docs/SubjectsV1Api.md#look_up_schema_under_subject) | **POST** /subjects/{subject} | Lookup schema under subject
*SubjectsV1Api* | [**register**](docs/SubjectsV1Api.md#register) | **POST** /subjects/{subject}/versions | Register schema under a subject


## Documentation For Models

 - [CompatibilityCheckResponse](docs/CompatibilityCheckResponse.md)
 - [Config](docs/Config.md)
 - [ConfigUpdateRequest](docs/ConfigUpdateRequest.md)
 - [ErrorMessage](docs/ErrorMessage.md)
 - [Metadata](docs/Metadata.md)
 - [Mode](docs/Mode.md)
 - [ModeUpdateRequest](docs/ModeUpdateRequest.md)
 - [RegisterSchemaRequest](docs/RegisterSchemaRequest.md)
 - [RegisterSchemaResponse](docs/RegisterSchemaResponse.md)
 - [Rule](docs/Rule.md)
 - [RuleSet](docs/RuleSet.md)
 - [Schema](docs/Schema.md)
 - [SchemaReference](docs/SchemaReference.md)
 - [SchemaRegistryServerVersion](docs/SchemaRegistryServerVersion.md)
 - [SchemaString](docs/SchemaString.md)
 - [ServerClusterId](docs/ServerClusterId.md)
 - [SubjectVersion](docs/SubjectVersion.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



