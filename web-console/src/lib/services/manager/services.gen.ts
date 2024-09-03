// This file is auto-generated by @hey-api/openapi-ts

import { client, type Options } from '@hey-api/client-fetch'
import type {
  GetConfigError,
  GetConfigResponse,
  GetConfigAuthenticationError,
  GetConfigAuthenticationResponse,
  ListApiKeysError,
  ListApiKeysResponse,
  PostApiKeyData,
  PostApiKeyError,
  PostApiKeyResponse,
  GetApiKeyData,
  GetApiKeyError,
  GetApiKeyResponse,
  DeleteApiKeyData,
  DeleteApiKeyError,
  DeleteApiKeyResponse,
  GetConfigDemosError,
  GetConfigDemosResponse,
  ListPipelinesData,
  ListPipelinesError,
  ListPipelinesResponse,
  PostPipelineData,
  PostPipelineError,
  PostPipelineResponse,
  GetPipelineData,
  GetPipelineError,
  GetPipelineResponse,
  PutPipelineData,
  PutPipelineError,
  PutPipelineResponse,
  DeletePipelineData,
  DeletePipelineError,
  DeletePipelineResponse,
  PatchPipelineData,
  PatchPipelineError,
  PatchPipelineResponse,
  GetPipelineCircuitProfileData,
  GetPipelineCircuitProfileError,
  GetPipelineCircuitProfileResponse,
  HttpOutputData,
  HttpOutputError,
  HttpOutputResponse,
  GetPipelineHeapProfileData,
  GetPipelineHeapProfileError,
  GetPipelineHeapProfileResponse,
  HttpInputData,
  HttpInputError,
  HttpInputResponse,
  InputEndpointActionData,
  InputEndpointActionError,
  InputEndpointActionResponse,
  PipelineAdhocSqlData,
  PipelineAdhocSqlError,
  PipelineAdhocSqlResponse,
  GetPipelineStatsData,
  GetPipelineStatsError,
  GetPipelineStatsResponse,
  PostPipelineActionData,
  PostPipelineActionError,
  PostPipelineActionResponse
} from './types.gen'

/**
 * Retrieve general configuration.
 */
export const getConfig = (options?: Options) => {
  return (options?.client ?? client).get<GetConfigResponse, GetConfigError>({
    ...options,
    url: '/config'
  })
}

/**
 * Retrieve authentication provider configuration.
 */
export const getConfigAuthentication = (options?: Options) => {
  return (options?.client ?? client).get<
    GetConfigAuthenticationResponse,
    GetConfigAuthenticationError
  >({
    ...options,
    url: '/config/authentication'
  })
}

/**
 * Retrieve the list of API keys.
 */
export const listApiKeys = (options?: Options) => {
  return (options?.client ?? client).get<ListApiKeysResponse, ListApiKeysError>({
    ...options,
    url: '/v0/api_keys'
  })
}

/**
 * Create a new API key.
 */
export const postApiKey = (options: Options<PostApiKeyData>) => {
  return (options?.client ?? client).post<PostApiKeyResponse, PostApiKeyError>({
    ...options,
    url: '/v0/api_keys'
  })
}

/**
 * Retrieve an API key.
 */
export const getApiKey = (options: Options<GetApiKeyData>) => {
  return (options?.client ?? client).get<GetApiKeyResponse, GetApiKeyError>({
    ...options,
    url: '/v0/api_keys/{api_key_name}'
  })
}

/**
 * Delete an API key.
 */
export const deleteApiKey = (options: Options<DeleteApiKeyData>) => {
  return (options?.client ?? client).delete<DeleteApiKeyResponse, DeleteApiKeyError>({
    ...options,
    url: '/v0/api_keys/{api_key_name}'
  })
}

/**
 * Retrieve the list of demos.
 */
export const getConfigDemos = (options?: Options) => {
  return (options?.client ?? client).get<GetConfigDemosResponse, GetConfigDemosError>({
    ...options,
    url: '/v0/config/demos'
  })
}

/**
 * Retrieve the list of pipelines.
 * Inclusion of program code is configured with by the `code` boolean query parameter.
 */
export const listPipelines = (options?: Options<ListPipelinesData>) => {
  return (options?.client ?? client).get<ListPipelinesResponse, ListPipelinesError>({
    ...options,
    url: '/v0/pipelines'
  })
}

/**
 * Create a new pipeline.
 */
export const postPipeline = (options: Options<PostPipelineData>) => {
  return (options?.client ?? client).post<PostPipelineResponse, PostPipelineError>({
    ...options,
    url: '/v0/pipelines'
  })
}

/**
 * Retrieve a pipeline.
 */
export const getPipeline = (options: Options<GetPipelineData>) => {
  return (options?.client ?? client).get<GetPipelineResponse, GetPipelineError>({
    ...options,
    url: '/v0/pipelines/{pipeline_name}'
  })
}

/**
 * Fully update a pipeline if it already exists, otherwise create a new pipeline.
 */
export const putPipeline = (options: Options<PutPipelineData>) => {
  return (options?.client ?? client).put<PutPipelineResponse, PutPipelineError>({
    ...options,
    url: '/v0/pipelines/{pipeline_name}'
  })
}

/**
 * Delete a pipeline.
 */
export const deletePipeline = (options: Options<DeletePipelineData>) => {
  return (options?.client ?? client).delete<DeletePipelineResponse, DeletePipelineError>({
    ...options,
    url: '/v0/pipelines/{pipeline_name}'
  })
}

/**
 * Partially update a pipeline.
 */
export const patchPipeline = (options: Options<PatchPipelineData>) => {
  return (options?.client ?? client).patch<PatchPipelineResponse, PatchPipelineError>({
    ...options,
    url: '/v0/pipelines/{pipeline_name}'
  })
}

/**
 * Retrieve the circuit performance profile of a running or paused pipeline.
 */
export const getPipelineCircuitProfile = (options: Options<GetPipelineCircuitProfileData>) => {
  return (options?.client ?? client).get<
    GetPipelineCircuitProfileResponse,
    GetPipelineCircuitProfileError
  >({
    ...options,
    url: '/v0/pipelines/{pipeline_name}/circuit_profile'
  })
}

/**
 * Subscribe to a stream of updates from a SQL view or table.
 * The pipeline responds with a continuous stream of changes to the specified
 * table or view, encoded using the format specified in the `?format=`
 * parameter. Updates are split into `Chunk`s.
 *
 * The pipeline continues sending updates until the client closes the
 * connection or the pipeline is shut down.
 *
 * This API is a POST instead of a GET, because when performing neighborhood
 * queries (query='neighborhood'), the call expects a request body which
 * contains, among other things, a full row to execute a neighborhood search
 * around. A row can be quite large and is not appropriate as a query
 * parameter.
 */
export const httpOutput = (options: Options<HttpOutputData>) => {
  return (options?.client ?? client).post<HttpOutputResponse, HttpOutputError>({
    ...options,
    url: '/v0/pipelines/{pipeline_name}/egress/{table_name}'
  })
}

/**
 * Retrieve the heap profile of a running or paused pipeline.
 */
export const getPipelineHeapProfile = (options: Options<GetPipelineHeapProfileData>) => {
  return (options?.client ?? client).get<
    GetPipelineHeapProfileResponse,
    GetPipelineHeapProfileError
  >({
    ...options,
    url: '/v0/pipelines/{pipeline_name}/heap_profile'
  })
}

/**
 * Push data to a SQL table.
 * The client sends data encoded using the format specified in the `?format=`
 * parameter as a body of the request.  The contents of the data must match
 * the SQL table schema specified in `table_name`
 *
 * The pipeline ingests data as it arrives without waiting for the end of
 * the request.  Successful HTTP response indicates that all data has been
 * ingested successfully.
 */
export const httpInput = (options: Options<HttpInputData>) => {
  return (options?.client ?? client).post<HttpInputResponse, HttpInputError>({
    ...options,
    url: '/v0/pipelines/{pipeline_name}/ingress/{table_name}'
  })
}

/**
 * Change the desired state of an input endpoint.
 * The following values of the `action` argument are accepted by this endpoint:
 *
 * - 'start': Start processing data.
 * - 'pause': Pause the pipeline.
 */
export const inputEndpointAction = (options: Options<InputEndpointActionData>) => {
  return (options?.client ?? client).post<InputEndpointActionResponse, InputEndpointActionError>({
    ...options,
    url: '/v0/pipelines/{pipeline_name}/input_endpoints/{endpoint_name}/{action}'
  })
}

/**
 * Execute an ad-hoc query in a running or paused pipeline.
 */
export const pipelineAdhocSql = (options: Options<PipelineAdhocSqlData>) => {
  return (options?.client ?? client).get<PipelineAdhocSqlResponse, PipelineAdhocSqlError>({
    ...options,
    url: '/v0/pipelines/{pipeline_name}/query'
  })
}

/**
 * Retrieve pipeline statistics (e.g., metrics, performance counters).
 */
export const getPipelineStats = (options: Options<GetPipelineStatsData>) => {
  return (options?.client ?? client).get<GetPipelineStatsResponse, GetPipelineStatsError>({
    ...options,
    url: '/v0/pipelines/{pipeline_name}/stats'
  })
}

/**
 * Start, pause or shutdown a pipeline.
 * The endpoint returns immediately after performing initial request validation
 * (e.g., upon start checking the program is compiled) and initiating the relevant
 * procedure (e.g., informing the runner or the already running pipeline).
 * The state changes completely asynchronously. On error, the pipeline
 * transitions to the `Failed` state. The user can monitor the current status
 * of the pipeline by polling the `GET /pipelines` and
 * `GET /pipelines/{pipeline_name}` endpoint.
 *
 * The following values of the `action` argument are accepted:
 * - `start`: Start the pipeline
 * - `pause`: Pause the pipeline
 * - `shutdown`: Terminate the pipeline
 */
export const postPipelineAction = (options: Options<PostPipelineActionData>) => {
  return (options?.client ?? client).post<PostPipelineActionResponse, PostPipelineActionError>({
    ...options,
    url: '/v0/pipelines/{pipeline_name}/{action}'
  })
}
