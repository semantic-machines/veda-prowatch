# \BadgeFieldsApi

All URIs are relative to *http://localhost:8734*

Method | HTTP request | Description
------------- | ------------- | -------------
[**badgefields**](BadgeFieldsApi.md#badgefields) | **get** /pwapi/badgefields | badgefields
[**badgefields_column_dropdowns**](BadgeFieldsApi.md#badgefields_column_dropdowns) | **get** /pwapi/badgefields/STATE/dropdowns | badgefields/{column}/dropdowns



## badgefields

> Vec<crate::models::Array> badgefields()
badgefields

Get a list of all badge fields from BADGE and BADGE_V tables.

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


## badgefields_column_dropdowns

> Vec<crate::models::Array> badgefields_column_dropdowns()
badgefields/{column}/dropdowns

A list of all drop down values for a drop down BADGE_V field.

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

