# \UsersApi

All URIs are relative to *http://localhost:8734*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users**](UsersApi.md#users) | **get** /pwapi/users | users
[**users1**](UsersApi.md#users1) | **post** /pwapi/users | users
[**users2**](UsersApi.md#users2) | **put** /pwapi/users | users
[**users_user_id**](UsersApi.md#users_user_id) | **delete** /pwapi/users/0x003B65462FFF4FDD4115A582BBF7E672EBF7 | users/{userId}
[**users_user_id_partition_partition_id**](UsersApi.md#users_user_id_partition_partition_id) | **post** /pwapi/users/0x003B230666581381417B9C7B1AE419A8B6C6/partition/0x001BE981B939B63E412E8D9138B3A0F0C12F | users/{userId}/partition/{partitionId}
[**users_user_id_partition_partition_id6**](UsersApi.md#users_user_id_partition_partition_id6) | **delete** /pwapi/users/0x003B230666581381417B9C7B1AE419A8B6C6/partition/0x001BE981B939B63E412E8D9138B3A0F0C12F | users/{userId}/partition/{partitionId}
[**users_user_id_partitions**](UsersApi.md#users_user_id_partitions) | **get** /pwapi/users/0x003B230666581381417B9C7B1AE419A8B6C6/partitions | users/{userId}/partitions
[**users_user_id_workstation_workstation_id**](UsersApi.md#users_user_id_workstation_workstation_id) | **post** /pwapi/users/0x003B230666581381417B9C7B1AE419A8B6C6/workstation/0x0012756D0E6C73B2474CA4EDA650FB7B88AC | users/{userId}/workstation/{workstationId}
[**users_user_id_workstation_workstation_id9**](UsersApi.md#users_user_id_workstation_workstation_id9) | **delete** /pwapi/users/0x003B230666581381417B9C7B1AE419A8B6C6/workstation/0x0012756D0E6C73B2474CA4EDA650FB7B88AC | users/{userId}/workstation/{workstationId}
[**users_user_id_workstations**](UsersApi.md#users_user_id_workstations) | **get** /pwapi/users/0x003B230666581381417B9C7B1AE419A8B6C6/workstations | users/{userId}/workstations



## users

> Vec<crate::models::Array> users()
users

Get all Pro-Watch users

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


## users1

> users1(body)
users

Add a Pro-Watch user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | Add a Pro-Watch user | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users2

> users2(body)
users

Update a Pro-Watch user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | Update a Pro-Watch user | [required] |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id

> users_user_id()
users/{userId}

Delete a Pro-Watch user

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


## users_user_id_partition_partition_id

> users_user_id_partition_partition_id()
users/{userId}/partition/{partitionId}

Add a partition to a Pro-Watch user

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


## users_user_id_partition_partition_id6

> users_user_id_partition_partition_id6()
users/{userId}/partition/{partitionId}

Delete a partition for a Pro-Watch user

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


## users_user_id_partitions

> Vec<serde_json::Value> users_user_id_partitions()
users/{userId}/partitions

Get a list of partitions a Pro-Watch user is in

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


## users_user_id_workstation_workstation_id

> users_user_id_workstation_workstation_id()
users/{userId}/workstation/{workstationId}

Add a new workstation for a Pro-Watch user

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


## users_user_id_workstation_workstation_id9

> users_user_id_workstation_workstation_id9()
users/{userId}/workstation/{workstationId}

Delete a workstion from a Pro-Watch user

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


## users_user_id_workstations

> Vec<serde_json::Value> users_user_id_workstations()
users/{userId}/workstations

Get all workstations for a Pro-Watch user

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

