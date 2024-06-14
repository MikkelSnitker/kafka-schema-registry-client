# \SubjectsV1Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_schema_version**](SubjectsV1Api.md#delete_schema_version) | **DELETE** /subjects/{subject}/versions/{version} | Delete schema version
[**delete_subject**](SubjectsV1Api.md#delete_subject) | **DELETE** /subjects/{subject} | Delete subject
[**get_referenced_by**](SubjectsV1Api.md#get_referenced_by) | **GET** /subjects/{subject}/versions/{version}/referencedby | List schemas referencing a schema
[**get_schema_by_version**](SubjectsV1Api.md#get_schema_by_version) | **GET** /subjects/{subject}/versions/{version} | Get schema by version
[**get_schema_only2**](SubjectsV1Api.md#get_schema_only2) | **GET** /subjects/{subject}/versions/{version}/schema | Get schema string by version
[**list**](SubjectsV1Api.md#list) | **GET** /subjects | List subjects
[**list_versions**](SubjectsV1Api.md#list_versions) | **GET** /subjects/{subject}/versions | List versions under subject
[**look_up_schema_under_subject**](SubjectsV1Api.md#look_up_schema_under_subject) | **POST** /subjects/{subject} | Lookup schema under subject
[**register**](SubjectsV1Api.md#register) | **POST** /subjects/{subject}/versions | Register schema under a subject



## delete_schema_version

> i32 delete_schema_version(subject, version, permanent)
Delete schema version

Deletes a specific version of the schema registered under this subject. This only deletes the version and the schema ID remains intact making it still possible to decode data using the schema ID. This API is recommended to be used only in development environments or under extreme circumstances where-in, its required to delete a previously registered schema for compatibility purposes or re-register previously registered schema.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Name of the subject | [required] |
**version** | **String** | Version of the schema to be returned. Valid values for versionId are between [1,2^31-1] or the string \"latest\". \"latest\" returns the last registered schema under the specified subject. Note that there may be a new latest schema that gets registered right after this request is served. | [required] |
**permanent** | Option<**bool**> | Whether to perform a permanent delete |  |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subject

> Vec<i32> delete_subject(subject, permanent)
Delete subject

Deletes the specified subject and its associated compatibility level if registered. It is recommended to use this API only when a topic needs to be recycled or in development environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Name of the subject | [required] |
**permanent** | Option<**bool**> | Whether to perform a permanent delete |  |

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_referenced_by

> Vec<i32> get_referenced_by(subject, version)
List schemas referencing a schema

Retrieves the IDs of schemas that reference the specified schema.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Name of the subject | [required] |
**version** | **String** | Version of the schema to be returned. Valid values for versionId are between [1,2^31-1] or the string \"latest\". \"latest\" returns the last registered schema under the specified subject. Note that there may be a new latest schema that gets registered right after this request is served. | [required] |

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schema_by_version

> models::Schema get_schema_by_version(subject, version, deleted)
Get schema by version

Retrieves a specific version of the schema registered under this subject.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Name of the subject | [required] |
**version** | **String** | Version of the schema to be returned. Valid values for versionId are between [1,2^31-1] or the string \"latest\". \"latest\" returns the last registered schema under the specified subject. Note that there may be a new latest schema that gets registered right after this request is served. | [required] |
**deleted** | Option<**bool**> | Whether to include deleted schema |  |

### Return type

[**models::Schema**](Schema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schema_only2

> String get_schema_only2(subject, version, deleted)
Get schema string by version

Retrieves the schema for the specified version of this subject. Only the unescaped schema string is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Name of the subject | [required] |
**version** | **String** | Version of the schema to be returned. Valid values for versionId are between [1,2^31-1] or the string \"latest\". \"latest\" returns the last registered schema under the specified subject. Note that there may be a new latest schema that gets registered right after this request is served. | [required] |
**deleted** | Option<**bool**> | Whether to include deleted schema |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list

> Vec<String> list(subject_prefix, deleted, deleted_only)
List subjects

Retrieves a list of registered subjects matching specified parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_prefix** | Option<**String**> | Subject name prefix |  |[default to :*:]
**deleted** | Option<**bool**> | Whether to look up deleted subjects |  |
**deleted_only** | Option<**bool**> | Whether to return deleted subjects only |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_versions

> Vec<i32> list_versions(subject, deleted, deleted_only)
List versions under subject

Retrieves a list of versions registered under the specified subject.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Name of the subject | [required] |
**deleted** | Option<**bool**> | Whether to include deleted schemas |  |
**deleted_only** | Option<**bool**> | Whether to return deleted schemas only |  |

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## look_up_schema_under_subject

> models::Schema look_up_schema_under_subject(subject, register_schema_request, normalize, deleted)
Lookup schema under subject

Check if a schema has already been registered under the specified subject. If so, this returns the schema string along with its globally unique identifier, its version under this subject and the subject name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Subject under which the schema will be registered | [required] |
**register_schema_request** | [**RegisterSchemaRequest**](RegisterSchemaRequest.md) | Schema | [required] |
**normalize** | Option<**bool**> | Whether to normalize the given schema |  |
**deleted** | Option<**bool**> | Whether to lookup deleted schemas |  |

### Return type

[**models::Schema**](Schema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json, application/json, application/octet-stream
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register

> models::RegisterSchemaResponse register(subject, register_schema_request, normalize)
Register schema under a subject

Register a new schema under the specified subject. If successfully registered, this returns the unique identifier of this schema in the registry. The returned identifier should be used to retrieve this schema from the schemas resource and is different from the schema's version which is associated with the subject. If the same schema is registered under a different subject, the same identifier will be returned. However, the version of the schema may be different under different subjects. A schema should be compatible with the previously registered schema or schemas (if there are any) as per the configured compatibility level. The configured compatibility level can be obtained by issuing a GET http:get:: /config/(string: subject). If that returns null, then GET http:get:: /config When there are multiple instances of Schema Registry running in the same cluster, the schema registration request will be forwarded to one of the instances designated as the primary. If the primary is not available, the client will get an error code indicating that the forwarding has failed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Name of the subject | [required] |
**register_schema_request** | [**RegisterSchemaRequest**](RegisterSchemaRequest.md) | Schema | [required] |
**normalize** | Option<**bool**> | Whether to normalize the given schema |  |

### Return type

[**models::RegisterSchemaResponse**](RegisterSchemaResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json, application/json, application/octet-stream
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

