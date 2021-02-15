# \ConfigurationApi

All URIs are relative to *http://localhost:8734*

Method | HTTP request | Description
------------- | ------------- | -------------
[**badgestatuses**](ConfigurationApi.md#badgestatuses) | **get** /pwapi/badgestatuses | badgestatuses
[**badgetypes**](ConfigurationApi.md#badgetypes) | **get** /pwapi/badgetypes | badgetypes
[**blobtypes**](ConfigurationApi.md#blobtypes) | **get** /pwapi/blobtypes | blobtypes
[**certifications**](ConfigurationApi.md#certifications) | **get** /pwapi/certifications | certifications
[**partitions**](ConfigurationApi.md#partitions) | **get** /pwapi/partitions | partitions
[**version**](ConfigurationApi.md#version) | **get** /pwapi/version | version



## badgestatuses

> Vec<crate::models::Array> badgestatuses()
badgestatuses

Get all badge statuses

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


## badgetypes

> Vec<crate::models::Array> badgetypes()
badgetypes

Get all badge types

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


## blobtypes

> Vec<crate::models::Array> blobtypes()
blobtypes

Get all blob types

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


## certifications

> Vec<crate::models::Array> certifications()
certifications

Get all certifications

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


## partitions

> Vec<crate::models::Array> partitions()
partitions

Get partitions

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


## version

> Vec<serde_json::Value> version()
version

Get the Pro-Watch server and Pro-Watch API version

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

