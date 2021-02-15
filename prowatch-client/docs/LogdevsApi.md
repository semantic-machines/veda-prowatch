# \LogdevsApi

All URIs are relative to *http://localhost:8734*

Method | HTTP request | Description
------------- | ------------- | -------------
[**logdevs**](LogdevsApi.md#logdevs) | **get** /pwapi/logdevs/false | logdevs
[**logdevs_log_dev_id_card_no_curr_date_time**](LogdevsApi.md#logdevs_log_dev_id_card_no_curr_date_time) | **get** /pwapi/logdevs/0x006F97BD76868E3011D4A45600508BC86902/12345/{50}/false | logdevs/{logDevId}/{cardNo}/{currDateTime}
[**logdevs_log_dev_id_hardware**](LogdevsApi.md#logdevs_log_dev_id_hardware) | **get** /pwapi/logdevs/0x006F97BD76938E3011D4A45600508BC86902/hardware | logdevs/{logDevId}/hardware
[**logdevs_log_dev_id_lock**](LogdevsApi.md#logdevs_log_dev_id_lock) | **post** /pwapi/logdevs/0x006F97BD76868E3011D4A45600508BC86902/lock | logdevs/{logDevId}/lock
[**logdevs_log_dev_id_momentaryunlock**](LogdevsApi.md#logdevs_log_dev_id_momentaryunlock) | **post** /pwapi/logdevs/0x006F97BD76868E3011D4A45600508BC86902/momentaryunlock | logdevs/{logDevId}/momentaryunlock
[**logdevs_log_dev_id_reenable**](LogdevsApi.md#logdevs_log_dev_id_reenable) | **post** /pwapi/logdevs/0x006F97BD76868E3011D4A45600508BC86902/reenable | logdevs/{logDevId}/reenable
[**logdevs_log_dev_id_timeoverride_seconds**](LogdevsApi.md#logdevs_log_dev_id_timeoverride_seconds) | **post** /pwapi/logdevs/0x006F97BD76868E3011D4A45600508BC86902/timeoverride/30 | logdevs/{logDevId}/timeoverride/{seconds}
[**logdevs_log_dev_id_unlock**](LogdevsApi.md#logdevs_log_dev_id_unlock) | **post** /pwapi/logdevs/0x006F97BD76868E3011D4A45600508BC86902/unlock | logdevs/{logDevId}/unlock



## logdevs

> Vec<crate::models::Array> logdevs()
logdevs

Get all logical devices

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


## logdevs_log_dev_id_card_no_curr_date_time

> logdevs_log_dev_id_card_no_curr_date_time(var_50)
logdevs/{logDevId}/{cardNo}/{currDateTime}

Check if card has access to Logical Device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**var_50** | **f32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logdevs_log_dev_id_hardware

> Vec<crate::models::Array> logdevs_log_dev_id_hardware()
logdevs/{logDevId}/hardware

Get the hardware for a logical device

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


## logdevs_log_dev_id_lock

> logdevs_log_dev_id_lock(body)
logdevs/{logDevId}/lock

Lock a door

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | Lock a door | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logdevs_log_dev_id_momentaryunlock

> logdevs_log_dev_id_momentaryunlock()
logdevs/{logDevId}/momentaryunlock

Momentary unlock a door

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logdevs_log_dev_id_reenable

> logdevs_log_dev_id_reenable()
logdevs/{logDevId}/reenable

Re-enable a logical device

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logdevs_log_dev_id_timeoverride_seconds

> logdevs_log_dev_id_timeoverride_seconds()
logdevs/{logDevId}/timeoverride/{seconds}

Time override a door

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logdevs_log_dev_id_unlock

> logdevs_log_dev_id_unlock()
logdevs/{logDevId}/unlock

Unlock a logical device/door

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

