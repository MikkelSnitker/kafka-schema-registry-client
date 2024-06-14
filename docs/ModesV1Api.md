# \ModesV1Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_subject_mode**](ModesV1Api.md#delete_subject_mode) | **DELETE** /mode/{subject} | Delete subject mode
[**get_mode**](ModesV1Api.md#get_mode) | **GET** /mode/{subject} | Get subject mode
[**get_top_level_mode**](ModesV1Api.md#get_top_level_mode) | **GET** /mode | Get global mode
[**update_mode**](ModesV1Api.md#update_mode) | **PUT** /mode/{subject} | Update subject mode
[**update_top_level_mode**](ModesV1Api.md#update_top_level_mode) | **PUT** /mode | Update global mode



## delete_subject_mode

> models::Mode delete_subject_mode(subject)
Delete subject mode

Deletes the specified subject-level mode and reverts to the global default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Name of the subject | [required] |

### Return type

[**models::Mode**](Mode.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mode

> models::Mode get_mode(subject, default_to_global)
Get subject mode

Retrieves the subject mode.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Name of the subject | [required] |
**default_to_global** | Option<**bool**> | Whether to return the global mode if subject mode not found |  |

### Return type

[**models::Mode**](Mode.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_top_level_mode

> models::Mode get_top_level_mode()
Get global mode

Retrieves global mode.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Mode**](Mode.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_mode

> models::ModeUpdateRequest update_mode(subject, mode_update_request, force)
Update subject mode

Update mode for the specified subject. On success, echoes the original request back to the client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | Name of the subject | [required] |
**mode_update_request** | [**ModeUpdateRequest**](ModeUpdateRequest.md) | Update Request | [required] |
**force** | Option<**bool**> | Whether to force update if setting mode to IMPORT and schemas currently exist |  |

### Return type

[**models::ModeUpdateRequest**](ModeUpdateRequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json, application/json, application/octet-stream
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_top_level_mode

> models::ModeUpdateRequest update_top_level_mode(mode_update_request, force)
Update global mode

Update global mode. On success, echoes the original request back to the client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mode_update_request** | [**ModeUpdateRequest**](ModeUpdateRequest.md) | Update Request | [required] |
**force** | Option<**bool**> | Whether to force update if setting mode to IMPORT and schemas currently exist |  |

### Return type

[**models::ModeUpdateRequest**](ModeUpdateRequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json, application/json, application/octet-stream
- **Accept**: application/vnd.schemaregistry.v1+json, application/vnd.schemaregistry+json; qs=0.9, application/json; qs=0.5

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

