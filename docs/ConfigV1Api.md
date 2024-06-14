# \ConfigV1Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_subject_config**](ConfigV1Api.md#delete_subject_config) | **DELETE** /config/{subject} | Delete subject compatibility level
[**delete_top_level_config**](ConfigV1Api.md#delete_top_level_config) | **DELETE** /config | Delete global compatibility level
[**get_subject_level_config**](ConfigV1Api.md#get_subject_level_config) | **GET** /config/{subject} | Get subject compatibility level
[**get_top_level_config**](ConfigV1Api.md#get_top_level_config) | **GET** /config | Get global compatibility level
[**update_subject_level_config**](ConfigV1Api.md#update_subject_level_config) | **PUT** /config/{subject} | Update subject compatibility level
[**update_top_level_config**](ConfigV1Api.md#update_top_level_config) | **PUT** /config | Update global compatibility level



## delete_subject_config

> String delete_subject_config(subject)
Delete subject compatibility level

Deletes the specified subject-level compatibility level config and reverts to the global default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Name of the subject | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_top_level_config

> String delete_top_level_config()
Delete global compatibility level

Deletes the global compatibility level config and reverts to the default.

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


## get_subject_level_config

> models::Config get_subject_level_config(subject, default_to_global)
Get subject compatibility level

Retrieves compatibility level for a subject.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Name of the subject | [required] |
**default_to_global** | Option<**bool**> | Whether to return the global compatibility level  if subject compatibility level not found |  |

### Return type

[**models::Config**](Config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_top_level_config

> models::Config get_top_level_config()
Get global compatibility level

Retrieves the global compatibility level.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Config**](Config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subject_level_config

> models::ConfigUpdateRequest update_subject_level_config(subject, config_update_request)
Update subject compatibility level

Update compatibility level for the specified subject. On success, echoes the original request back to the client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Name of the subject | [required] |
**config_update_request** | [**ConfigUpdateRequest**](ConfigUpdateRequest.md) | Config Update Request | [required] |

### Return type

[**models::ConfigUpdateRequest**](ConfigUpdateRequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json, application/json, application/octet-stream
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_top_level_config

> models::ConfigUpdateRequest update_top_level_config(config_update_request)
Update global compatibility level

Updates the global compatibility level. On success, echoes the original request back to the client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_update_request** | [**ConfigUpdateRequest**](ConfigUpdateRequest.md) | Config Update Request | [required] |

### Return type

[**models::ConfigUpdateRequest**](ConfigUpdateRequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json, application/json, application/octet-stream
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

