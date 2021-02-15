# \ClearcodesApi

All URIs are relative to *http://localhost:8734*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clearcodes**](ClearcodesApi.md#clearcodes) | **get** /pwapi/clearcodes | clearcodes
[**clearcodes_clear_code_logdevs**](ClearcodesApi.md#clearcodes_clear_code_logdevs) | **get** /pwapi/clearcodes/0x004730D686E40D8B4E928C760440EF75745B/logdevs | clearcodes/{clearCode}/logdevs



## clearcodes

> Vec<crate::models::Array> clearcodes()
clearcodes

Get all clearance codes

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


## clearcodes_clear_code_logdevs

> Vec<crate::models::Array> clearcodes_clear_code_logdevs()
clearcodes/{clearCode}/logdevs

Get a list of logical devices in a clearance code

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

