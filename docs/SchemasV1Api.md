# \SchemasV1Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_schema**](SchemasV1Api.md#get_schema) | **GET** /schemas/ids/{id} | Get schema string by ID
[**get_schema_only**](SchemasV1Api.md#get_schema_only) | **GET** /schemas/ids/{id}/schema | Get schema by ID
[**get_schema_types**](SchemasV1Api.md#get_schema_types) | **GET** /schemas/types | List supported schema types
[**get_schemas**](SchemasV1Api.md#get_schemas) | **GET** /schemas | List schemas
[**get_subjects**](SchemasV1Api.md#get_subjects) | **GET** /schemas/ids/{id}/subjects | List subjects associated to schema ID
[**get_versions**](SchemasV1Api.md#get_versions) | **GET** /schemas/ids/{id}/versions | List subject-versions associated to schema ID



## get_schema

> models::SchemaString get_schema(id, subject, format, fetch_max_id)
Get schema string by ID

Retrieves the schema string identified by the input ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Globally unique identifier of the schema | [required] |
**subject** | Option<**String**> | Name of the subject |  |
**format** | Option<**String**> | Desired output format, dependent on schema type |  |[default to ]
**fetch_max_id** | Option<**bool**> | Whether to fetch the maximum schema identifier that exists |  |[default to false]

### Return type

[**models::SchemaString**](SchemaString.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schema_only

> String get_schema_only(id, subject, format)
Get schema by ID

Retrieves the schema identified by the input ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Globally unique identifier of the schema | [required] |
**subject** | Option<**String**> | Name of the subject |  |
**format** | Option<**String**> | Desired output format, dependent on schema type |  |[default to ]

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schema_types

> Vec<String> get_schema_types()
List supported schema types

Retrieve the schema types supported by this registry.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schemas

> Vec<models::Schema> get_schemas(subject_prefix, deleted, latest_only, offset, limit)
List schemas

Get the schemas matching the specified parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_prefix** | Option<**String**> | Filters results by the respective subject prefix |  |[default to ]
**deleted** | Option<**bool**> | Whether to return soft deleted schemas |  |[default to false]
**latest_only** | Option<**bool**> | Whether to return latest schema versions only for each matching subject |  |[default to false]
**offset** | Option<**i32**> | Pagination offset for results |  |[default to 0]
**limit** | Option<**i32**> | Pagination size for results. Ignored if negative |  |[default to -1]

### Return type

[**Vec<models::Schema>**](Schema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subjects

> Vec<String> get_subjects(id, subject, deleted)
List subjects associated to schema ID

Retrieves all the subjects associated with a particular schema ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Globally unique identifier of the schema | [required] |
**subject** | Option<**String**> | Filters results by the respective subject |  |
**deleted** | Option<**bool**> | Whether to include subjects where the schema was deleted |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_versions

> Vec<models::SubjectVersion> get_versions(id, subject, deleted)
List subject-versions associated to schema ID

Get all the subject-version pairs associated with the input ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Globally unique identifier of the schema | [required] |
**subject** | Option<**String**> | Filters results by the respective subject |  |
**deleted** | Option<**bool**> | Whether to include subject versions where the schema was deleted |  |

### Return type

[**Vec<models::SubjectVersion>**](SubjectVersion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

