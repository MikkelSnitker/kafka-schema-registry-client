# \CompatibilityV1Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**test_compatibility_by_subject_name**](CompatibilityV1Api.md#test_compatibility_by_subject_name) | **POST** /compatibility/subjects/{subject}/versions/{version} | Test schema compatibility against a particular schema subject-version
[**test_compatibility_for_subject**](CompatibilityV1Api.md#test_compatibility_for_subject) | **POST** /compatibility/subjects/{subject}/versions | Test schema compatibility against all schemas under a subject



## test_compatibility_by_subject_name

> models::CompatibilityCheckResponse test_compatibility_by_subject_name(subject, version, register_schema_request, normalize, verbose)
Test schema compatibility against a particular schema subject-version

Test input schema against a particular version of a subject's schema for compatibility. The compatibility level applied for the check is the configured compatibility level for the subject (http:get:: /config/(string: subject)). If this subject's compatibility level was never changed, then the global compatibility level applies (http:get:: /config).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Subject of the schema version against which compatibility is to be tested | [required] |
**version** | **String** | Version of the subject's schema against which compatibility is to be tested. Valid values for versionId are between [1,2^31-1] or the string \"latest\".\"latest\" checks compatibility of the input schema with the last registered schema under the specified subject | [required] |
**register_schema_request** | [**RegisterSchemaRequest**](RegisterSchemaRequest.md) | Schema | [required] |
**normalize** | Option<**bool**> | Whether to normalize the given schema |  |
**verbose** | Option<**bool**> | Whether to return detailed error messages |  |

### Return type

[**models::CompatibilityCheckResponse**](CompatibilityCheckResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json, application/json, application/octet-stream
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_compatibility_for_subject

> models::CompatibilityCheckResponse test_compatibility_for_subject(subject, register_schema_request, normalize, verbose)
Test schema compatibility against all schemas under a subject

Test input schema against a subject's schemas for compatibility, based on the configured compatibility level of the subject. In other words, it will perform the same compatibility check as register for that subject. The compatibility level applied for the check is the configured compatibility level for the subject (http:get:: /config/(string: subject)). If this subject's compatibility level was never changed, then the global compatibility level applies (http:get:: /config).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Subject of the schema version against which compatibility is to be tested | [required] |
**register_schema_request** | [**RegisterSchemaRequest**](RegisterSchemaRequest.md) | Schema | [required] |
**normalize** | Option<**bool**> | Whether to normalize the given schema |  |
**verbose** | Option<**bool**> | Whether to return detailed error messages |  |

### Return type

[**models::CompatibilityCheckResponse**](CompatibilityCheckResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json, application/json, application/octet-stream
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

