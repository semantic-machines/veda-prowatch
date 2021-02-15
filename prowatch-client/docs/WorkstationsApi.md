# \WorkstationsApi

All URIs are relative to *http://localhost:8734*

Method | HTTP request | Description
------------- | ------------- | -------------
[**workstations**](WorkstationsApi.md#workstations) | **get** /pwapi/workstations | workstations
[**workstations1**](WorkstationsApi.md#workstations1) | **post** /pwapi/workstations | workstations
[**workstations2**](WorkstationsApi.md#workstations2) | **put** /pwapi/workstations | workstations
[**workstations_wrkst_id**](WorkstationsApi.md#workstations_wrkst_id) | **delete** /pwapi/workstations/0x001246333343363431362D333743422D3436 | workstations/{wrkstId}
[**workstations_wrkst_id_partition_partition_id**](WorkstationsApi.md#workstations_wrkst_id_partition_partition_id) | **post** /pwapi/workstations/0x001246333343363431362D333743422D3436/partition/0x001B9DC55667C3D94552A630DCD83A1200FF | workstations/{wrkstID}/partition/{partitionID}
[**workstations_wrkst_id_partition_partition_id_0**](WorkstationsApi.md#workstations_wrkst_id_partition_partition_id_0) | **delete** /pwapi/workstations/0x001246333343363431362D333743422D3436/partition/0x001B9DC55667C3D94552A630DCD83A1200FF | workstations/{wrkstId}/partition/{partitionId}
[**workstations_wrkst_id_partitions**](WorkstationsApi.md#workstations_wrkst_id_partitions) | **get** /pwapi/workstations/0x001246333343363431362D333743422D3436/partitions | workstations/{wrkstId}/partitions



## workstations

> Vec<crate::models::Array> workstations()
workstations

Get all workstations

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


## workstations1

> workstations1(body)
workstations

Add a new workstion to Pro-Watch

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | Add a new workstion to Pro-Watch | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workstations2

> workstations2(body)
workstations

Update a Pro-Watch workstation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | Update a Pro-Watch workstation | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workstations_wrkst_id

> workstations_wrkst_id()
workstations/{wrkstId}

Delete a Pro-Watch workstation

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


## workstations_wrkst_id_partition_partition_id

> workstations_wrkst_id_partition_partition_id()
workstations/{wrkstID}/partition/{partitionID}

Add a partition to a Pro-Watch workstation

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


## workstations_wrkst_id_partition_partition_id_0

> workstations_wrkst_id_partition_partition_id_0()
workstations/{wrkstId}/partition/{partitionId}

Delete a partition from a Pro-Watch workstation

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


## workstations_wrkst_id_partitions

> Vec<serde_json::Value> workstations_wrkst_id_partitions()
workstations/{wrkstId}/partitions

Get assigned partitions for a Pro-Watch workstation

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

