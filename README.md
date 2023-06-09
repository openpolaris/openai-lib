# Rust API client for openai-lib

APIs for sampling from and fine-tuning language models


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.2.0
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openai-lib` and add the following to `Cargo.toml` under `[dependencies]`:

```
openai-lib = { path = "./openai-lib" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.openai.com/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*OpenAiApi* | [**cancel_fine_tune**](docs/OpenAiApi.md#cancel_fine_tune) | **POST** /fine-tunes/{fine_tune_id}/cancel | Immediately cancel a fine-tune job. 
*OpenAiApi* | [**create_answer**](docs/OpenAiApi.md#create_answer) | **POST** /answers | Answers the specified question using the provided documents and examples.  The endpoint first [searches](/docs/api-reference/searches) over provided documents or files to find relevant context. The relevant context is combined with the provided examples and question to create the prompt for [completion](/docs/api-reference/completions). 
*OpenAiApi* | [**create_chat_completion**](docs/OpenAiApi.md#create_chat_completion) | **POST** /chat/completions | Creates a completion for the chat message
*OpenAiApi* | [**create_classification**](docs/OpenAiApi.md#create_classification) | **POST** /classifications | Classifies the specified `query` using provided examples.  The endpoint first [searches](/docs/api-reference/searches) over the labeled examples to select the ones most relevant for the particular query. Then, the relevant examples are combined with the query to construct a prompt to produce the final label via the [completions](/docs/api-reference/completions) endpoint.  Labeled examples can be provided via an uploaded `file`, or explicitly listed in the request using the `examples` parameter for quick tests and small scale use cases. 
*OpenAiApi* | [**create_completion**](docs/OpenAiApi.md#create_completion) | **POST** /completions | Creates a completion for the provided prompt and parameters
*OpenAiApi* | [**create_edit**](docs/OpenAiApi.md#create_edit) | **POST** /edits | Creates a new edit for the provided input, instruction, and parameters.
*OpenAiApi* | [**create_embedding**](docs/OpenAiApi.md#create_embedding) | **POST** /embeddings | Creates an embedding vector representing the input text.
*OpenAiApi* | [**create_file**](docs/OpenAiApi.md#create_file) | **POST** /files | Upload a file that contains document(s) to be used across various endpoints/features. Currently, the size of all the files uploaded by one organization can be up to 1 GB. Please contact us if you need to increase the storage limit. 
*OpenAiApi* | [**create_fine_tune**](docs/OpenAiApi.md#create_fine_tune) | **POST** /fine-tunes | Creates a job that fine-tunes a specified model from a given dataset.  Response includes details of the enqueued job including job status and the name of the fine-tuned models once complete.  [Learn more about Fine-tuning](/docs/guides/fine-tuning) 
*OpenAiApi* | [**create_image**](docs/OpenAiApi.md#create_image) | **POST** /images/generations | Creates an image given a prompt.
*OpenAiApi* | [**create_image_edit**](docs/OpenAiApi.md#create_image_edit) | **POST** /images/edits | Creates an edited or extended image given an original image and a prompt.
*OpenAiApi* | [**create_image_variation**](docs/OpenAiApi.md#create_image_variation) | **POST** /images/variations | Creates a variation of a given image.
*OpenAiApi* | [**create_moderation**](docs/OpenAiApi.md#create_moderation) | **POST** /moderations | Classifies if text violates OpenAI's Content Policy
*OpenAiApi* | [**create_search**](docs/OpenAiApi.md#create_search) | **POST** /engines/{engine_id}/search | The search endpoint computes similarity scores between provided query and documents. Documents can be passed directly to the API if there are no more than 200 of them.  To go beyond the 200 document limit, documents can be processed offline and then used for efficient retrieval at query time. When `file` is set, the search endpoint searches over all the documents in the given file and returns up to the `max_rerank` number of documents. These documents will be returned along with their search scores.  The similarity score is a positive score that usually ranges from 0 to 300 (but can sometimes go higher), where a score above 200 usually means the document is semantically similar to the query. 
*OpenAiApi* | [**create_transcription**](docs/OpenAiApi.md#create_transcription) | **POST** /audio/transcriptions | Transcribes audio into the input language.
*OpenAiApi* | [**create_translation**](docs/OpenAiApi.md#create_translation) | **POST** /audio/translations | Translates audio into into English.
*OpenAiApi* | [**delete_file**](docs/OpenAiApi.md#delete_file) | **DELETE** /files/{file_id} | Delete a file.
*OpenAiApi* | [**delete_model**](docs/OpenAiApi.md#delete_model) | **DELETE** /models/{model} | Delete a fine-tuned model. You must have the Owner role in your organization.
*OpenAiApi* | [**download_file**](docs/OpenAiApi.md#download_file) | **GET** /files/{file_id}/content | Returns the contents of the specified file
*OpenAiApi* | [**list_engines**](docs/OpenAiApi.md#list_engines) | **GET** /engines | Lists the currently available (non-finetuned) models, and provides basic information about each one such as the owner and availability.
*OpenAiApi* | [**list_files**](docs/OpenAiApi.md#list_files) | **GET** /files | Returns a list of files that belong to the user's organization.
*OpenAiApi* | [**list_fine_tune_events**](docs/OpenAiApi.md#list_fine_tune_events) | **GET** /fine-tunes/{fine_tune_id}/events | Get fine-grained status updates for a fine-tune job. 
*OpenAiApi* | [**list_fine_tunes**](docs/OpenAiApi.md#list_fine_tunes) | **GET** /fine-tunes | List your organization's fine-tuning jobs 
*OpenAiApi* | [**list_models**](docs/OpenAiApi.md#list_models) | **GET** /models | Lists the currently available models, and provides basic information about each one such as the owner and availability.
*OpenAiApi* | [**retrieve_engine**](docs/OpenAiApi.md#retrieve_engine) | **GET** /engines/{engine_id} | Retrieves a model instance, providing basic information about it such as the owner and availability.
*OpenAiApi* | [**retrieve_file**](docs/OpenAiApi.md#retrieve_file) | **GET** /files/{file_id} | Returns information about a specific file.
*OpenAiApi* | [**retrieve_fine_tune**](docs/OpenAiApi.md#retrieve_fine_tune) | **GET** /fine-tunes/{fine_tune_id} | Gets info about the fine-tune job.  [Learn more about Fine-tuning](/docs/guides/fine-tuning) 
*OpenAiApi* | [**retrieve_model**](docs/OpenAiApi.md#retrieve_model) | **GET** /models/{model} | Retrieves a model instance, providing basic information about the model such as the owner and permissioning.


## Documentation For Models

 - [ChatCompletionRequestMessage](docs/ChatCompletionRequestMessage.md)
 - [ChatCompletionResponseMessage](docs/ChatCompletionResponseMessage.md)
 - [CreateAnswerRequest](docs/CreateAnswerRequest.md)
 - [CreateAnswerRequestStop](docs/CreateAnswerRequestStop.md)
 - [CreateAnswerResponse](docs/CreateAnswerResponse.md)
 - [CreateAnswerResponseSelectedDocumentsInner](docs/CreateAnswerResponseSelectedDocumentsInner.md)
 - [CreateChatCompletionRequest](docs/CreateChatCompletionRequest.md)
 - [CreateChatCompletionRequestStop](docs/CreateChatCompletionRequestStop.md)
 - [CreateChatCompletionResponse](docs/CreateChatCompletionResponse.md)
 - [CreateChatCompletionResponseChoicesInner](docs/CreateChatCompletionResponseChoicesInner.md)
 - [CreateClassificationRequest](docs/CreateClassificationRequest.md)
 - [CreateClassificationResponse](docs/CreateClassificationResponse.md)
 - [CreateClassificationResponseSelectedExamplesInner](docs/CreateClassificationResponseSelectedExamplesInner.md)
 - [CreateCompletionRequest](docs/CreateCompletionRequest.md)
 - [CreateCompletionRequestPrompt](docs/CreateCompletionRequestPrompt.md)
 - [CreateCompletionRequestStop](docs/CreateCompletionRequestStop.md)
 - [CreateCompletionResponse](docs/CreateCompletionResponse.md)
 - [CreateCompletionResponseChoicesInner](docs/CreateCompletionResponseChoicesInner.md)
 - [CreateCompletionResponseChoicesInnerLogprobs](docs/CreateCompletionResponseChoicesInnerLogprobs.md)
 - [CreateCompletionResponseUsage](docs/CreateCompletionResponseUsage.md)
 - [CreateEditRequest](docs/CreateEditRequest.md)
 - [CreateEditResponse](docs/CreateEditResponse.md)
 - [CreateEmbeddingRequest](docs/CreateEmbeddingRequest.md)
 - [CreateEmbeddingRequestInput](docs/CreateEmbeddingRequestInput.md)
 - [CreateEmbeddingResponse](docs/CreateEmbeddingResponse.md)
 - [CreateEmbeddingResponseDataInner](docs/CreateEmbeddingResponseDataInner.md)
 - [CreateEmbeddingResponseUsage](docs/CreateEmbeddingResponseUsage.md)
 - [CreateFineTuneRequest](docs/CreateFineTuneRequest.md)
 - [CreateImageRequest](docs/CreateImageRequest.md)
 - [CreateModerationRequest](docs/CreateModerationRequest.md)
 - [CreateModerationRequestInput](docs/CreateModerationRequestInput.md)
 - [CreateModerationResponse](docs/CreateModerationResponse.md)
 - [CreateModerationResponseResultsInner](docs/CreateModerationResponseResultsInner.md)
 - [CreateModerationResponseResultsInnerCategories](docs/CreateModerationResponseResultsInnerCategories.md)
 - [CreateModerationResponseResultsInnerCategoryScores](docs/CreateModerationResponseResultsInnerCategoryScores.md)
 - [CreateSearchRequest](docs/CreateSearchRequest.md)
 - [CreateSearchResponse](docs/CreateSearchResponse.md)
 - [CreateSearchResponseDataInner](docs/CreateSearchResponseDataInner.md)
 - [CreateTranscriptionResponse](docs/CreateTranscriptionResponse.md)
 - [CreateTranslationResponse](docs/CreateTranslationResponse.md)
 - [DeleteFileResponse](docs/DeleteFileResponse.md)
 - [DeleteModelResponse](docs/DeleteModelResponse.md)
 - [Engine](docs/Engine.md)
 - [FineTune](docs/FineTune.md)
 - [FineTuneEvent](docs/FineTuneEvent.md)
 - [ImagesResponse](docs/ImagesResponse.md)
 - [ImagesResponseDataInner](docs/ImagesResponseDataInner.md)
 - [ListEnginesResponse](docs/ListEnginesResponse.md)
 - [ListFilesResponse](docs/ListFilesResponse.md)
 - [ListFineTuneEventsResponse](docs/ListFineTuneEventsResponse.md)
 - [ListFineTunesResponse](docs/ListFineTunesResponse.md)
 - [ListModelsResponse](docs/ListModelsResponse.md)
 - [Model](docs/Model.md)
 - [OpenAiFile](docs/OpenAiFile.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



