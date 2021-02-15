# \SubpanelsApi

All URIs are relative to *http://localhost:8734*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hwclass_hw_class_id_logdevs**](SubpanelsApi.md#hwclass_hw_class_id_logdevs) | **get** /pwapi/hwclass/0x006E729E6F428D9311D4A45600508BC86902/logdevs/false | hwclass/{hwClassId}/logdevs
[**subpanels**](SubpanelsApi.md#subpanels) | **get** /pwapi/subpanels | subpanels



## hwclass_hw_class_id_logdevs

> Vec<crate::models::Array> hwclass_hw_class_id_logdevs()
hwclass/{hwClassId}/logdevs

Get all logical devices in a hardware class

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


## subpanels

> Vec<crate::models::Array> subpanels()
subpanels

Get subpanels

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

