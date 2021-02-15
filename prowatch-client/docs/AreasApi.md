# \AreasApi

All URIs are relative to *http://localhost:8734*

Method | HTTP request | Description
------------- | ------------- | -------------
[**areas**](AreasApi.md#areas) | **get** /pwapi/areas | areas
[**areas_area_id_card_no_curr_date_time**](AreasApi.md#areas_area_id_card_no_curr_date_time) | **get** /pwapi/areas/0x006318BB0353EF0D403DAA71C591A0AAC592/12345/{12} | areas/{areaId}/{cardNo}/{currDateTime}
[**areas_area_id_occupants**](AreasApi.md#areas_area_id_occupants) | **get** /pwapi/areas/0x006318BB0353EF0D403DAA71C591A0AAC592/occupants | areas/{areaId}/occupants



## areas

> Vec<crate::models::Array> areas()
areas

Gets a list of all Areas.

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


## areas_area_id_card_no_curr_date_time

> areas_area_id_card_no_curr_date_time(var_12)
areas/{areaId}/{cardNo}/{currDateTime}

Confirmation that a card can enter the area at the specified DateTime.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**var_12** | **f32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## areas_area_id_occupants

> Vec<crate::models::Array> areas_area_id_occupants()
areas/{areaId}/occupants

Get a list of occupants in an Area

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

