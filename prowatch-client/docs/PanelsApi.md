# \PanelsApi

All URIs are relative to *http://localhost:8734*

Method | HTTP request | Description
------------- | ------------- | -------------
[**panels**](PanelsApi.md#panels) | **get** /pwapi/panels | panels
[**panels_panel_id_logdevs**](PanelsApi.md#panels_panel_id_logdevs) | **get** /pwapi/panels/Generic::050100/logdevs/true | panels/{panelId}/logdevs
[**panels_panel_id_subpanels**](PanelsApi.md#panels_panel_id_subpanels) | **get** /pwapi/panels/Generic::050100/subpanels | panels/{panelId}/subpanels
[**panels_panel_id_timezones**](PanelsApi.md#panels_panel_id_timezones) | **get** /pwapi/panels/Generic::050100/timezones | panels/{panelId}/timezones



## panels

> Vec<crate::models::Array> panels()
panels

Gets all panels

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


## panels_panel_id_logdevs

> Vec<crate::models::Array> panels_panel_id_logdevs()
panels/{panelId}/logdevs

Get all logical devices for a panel

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


## panels_panel_id_subpanels

> Vec<crate::models::Array> panels_panel_id_subpanels()
panels/{panelId}/subpanels

Gets all sub-panels for a panel

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


## panels_panel_id_timezones

> Vec<crate::models::Array> panels_panel_id_timezones()
panels/{panelId}/timezones

Get all time zones for a panel

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

