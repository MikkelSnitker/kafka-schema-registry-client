# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get**](DefaultApi.md#get) | **GET** / | Schema Registry Root Resource
[**get_latest_with_metadata**](DefaultApi.md#get_latest_with_metadata) | **GET** /subjects/{subject}/metadata | Retrieve the latest version with the given metadata.
[**post**](DefaultApi.md#post) | **POST** / | 



## get

> String get()
Schema Registry Root Resource

The Root resource is a no-op.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_with_metadata

> models::Schema get_latest_with_metadata(subject, key, value, deleted)
Retrieve the latest version with the given metadata.

Retrieve the latest version with the given metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Subject under which the schema will be registered | [required] |
**key** | Option<[**Vec<String>**](String.md)> | The metadata key |  |
**value** | Option<[**Vec<String>**](String.md)> | The metadata value |  |
**deleted** | Option<**bool**> | Whether to lookup deleted schemas |  |

### Return type

[**models::Schema**](Schema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post

> std::collections::HashMap<String, String> post(request_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | Option<[**std::collections::HashMap<String, String>**](String.md)> |  |  |

### Return type

**std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json, application/json, application/octet-stream
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

