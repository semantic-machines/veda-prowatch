# \SitesApi

All URIs are relative to *http://localhost:8734*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sites**](SitesApi.md#sites) | **get** /pwapi/sites | sites
[**sites_site_channels**](SitesApi.md#sites_site_channels) | **get** /pwapi/sites/Generic/channels | sites/{site}/channels
[**sites_site_logdevs**](SitesApi.md#sites_site_logdevs) | **get** /pwapi/sites/Generic/logdevs | sites/{site}/logdevs
[**sites_site_panels**](SitesApi.md#sites_site_panels) | **get** /pwapi/sites/Generic/panels | sites/{site}/panels
[**sites_site_subpanels**](SitesApi.md#sites_site_subpanels) | **get** /pwapi/sites/Generic/subpanels | sites/{site}/subpanels
[**sites_site_summary**](SitesApi.md#sites_site_summary) | **get** /pwapi/sites/Generic/summary | sites/{site}/summary



## sites

> Vec<crate::models::Array> sites()
sites

Get sites

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


## sites_site_channels

> Vec<crate::models::Array> sites_site_channels()
sites/{site}/channels

Get all channels from a site

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


## sites_site_logdevs

> Vec<crate::models::Array> sites_site_logdevs()
sites/{site}/logdevs

Get all logical devices for a site

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


## sites_site_panels

> Vec<crate::models::Array> sites_site_panels()
sites/{site}/panels

Get all panels for a site

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


## sites_site_subpanels

> Vec<crate::models::Array> sites_site_subpanels()
sites/{site}/subpanels

Get all subpanels for a site

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


## sites_site_summary

> Vec<crate::models::Array> sites_site_summary()
sites/{site}/summary

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

