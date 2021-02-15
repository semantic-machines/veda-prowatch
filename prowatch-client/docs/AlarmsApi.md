# \AlarmsApi

All URIs are relative to *http://localhost:8734*

Method | HTTP request | Description
------------- | ------------- | -------------
[**alarms**](AlarmsApi.md#alarms) | **get** /pwapi/alarms | alarms
[**alarms_event_id_state_acknowledge**](AlarmsApi.md#alarms_event_id_state_acknowledge) | **put** /pwapi/alarms/0x007107763771C78E4D398CC79E15A79A8C20/state/acknowledge | alarms/{eventId}/state/acknowledge
[**alarms_event_id_state_clear**](AlarmsApi.md#alarms_event_id_state_clear) | **put** /pwapi/alarms/0x007107763771C78E4D398CC79E15A79A8C20/state/clear | alarms/{eventId}/state/clear
[**alarms_event_id_state_unacknowledge**](AlarmsApi.md#alarms_event_id_state_unacknowledge) | **put** /pwapi/alarms/0x007107763771C78E4D398CC79E15A79A8C20/state/unacknowledge | alarms/{eventId}/state/unacknowledge
[**alarms_event_id_state_wait**](AlarmsApi.md#alarms_event_id_state_wait) | **put** /pwapi/alarms/0x007110140EF5E61D4189BBD4B08627CB5336/state/wait | alarms/{eventId}/state/wait
[**alarms_state**](AlarmsApi.md#alarms_state) | **get** /pwapi/alarms/state | alarms/state
[**alarms_state_change**](AlarmsApi.md#alarms_state_change) | **put** /pwapi/alarms/state/change | alarms/state/change



## alarms

> Vec<crate::models::Array> alarms(X_PW_WRKST)
alarms

Gets all alarms in system that have not been cleared

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**X_PW_WRKST** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Array>**](array.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarms_event_id_state_acknowledge

> alarms_event_id_state_acknowledge(X_PW_WRKST)
alarms/{eventId}/state/acknowledge

Acknowledge an alarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**X_PW_WRKST** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarms_event_id_state_clear

> alarms_event_id_state_clear(X_PW_WRKST)
alarms/{eventId}/state/clear

Clear an alarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**X_PW_WRKST** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarms_event_id_state_unacknowledge

> alarms_event_id_state_unacknowledge(X_PW_WRKST)
alarms/{eventId}/state/unacknowledge

Put an alarm in unacknowledged state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**X_PW_WRKST** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarms_event_id_state_wait

> alarms_event_id_state_wait(X_PW_WRKST, X_PW_WRKST2)
alarms/{eventId}/state/wait

Put an alarm in wait state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**X_PW_WRKST** | **String** |  | [required] |
**X_PW_WRKST2** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarms_state

> Vec<crate::models::Array> alarms_state(X_PW_WRKST)
alarms/state

Gets current dispositions for all alarms in the system

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**X_PW_WRKST** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Array>**](array.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarms_state_change

> alarms_state_change(X_PW_WRKST)
alarms/state/change

Changes the current dispositions for alarms in the system

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**X_PW_WRKST** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

