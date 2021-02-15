# \TimezonesApi

All URIs are relative to *http://localhost:8734*

Method | HTTP request | Description
------------- | ------------- | -------------
[**timezones**](TimezonesApi.md#timezones) | **get** /pwapi/timezones | timezones
[**timezones1**](TimezonesApi.md#timezones1) | **post** /pwapi/timezones | timezones
[**timezones_timezone_id**](TimezonesApi.md#timezones_timezone_id) | **put** /pwapi/timezones/{timezoneID} | timezones/{timezoneID}



## timezones

> Vec<crate::models::Array> timezones()
timezones

Get all time zones

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Array>**](array.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timezones1

> timezones1(body)
timezones

Add a timezone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | Add a timezone | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timezones_timezone_id

> timezones_timezone_id(timezone_id, body)
timezones/{timezoneID}

Edit a timezone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timezone_id** | **String** |  | [required] |
**body** | **String** | Edit a timezone | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

