use crate::dto::LlmRequest;
use crate::error::ApiError;
use crate::{api_scope, ApiResult};
use actix_helper_utils::generate_endpoint;
use actix_web::{web, HttpResponse};
use bytes::Bytes;
use futures_util::stream::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tosic_llm::traits::LlmClient;
use tosic_llm::{GeminiClient, GeminiModel};
use tracing::error;

api_scope! {
    pub(super) Ai = "/ai";

    paths: [completions];

    docs: {
        schemas: [LlmRequest];
    }
}

#[tracing::instrument]
async fn stream_handler(req: LlmRequest) -> ApiResult<HttpResponse> {
    let (tx, rx) = mpsc::channel::<ApiResult<Bytes>>(1024);
    let stream = ReceiverStream::new(rx);

    tokio::spawn(async move {
        let client = GeminiClient::new(GeminiModel::Gemini2Flash);

        let mut ai_stream = client.stream_chat_completion(req.contents.into()).await?;

        while let Some(chunk) = ai_stream.next().await {
            match chunk {
                Ok(bytes) => {
                    if tx.send(Ok(bytes.into())).await.is_err() {
                        error!("failed to send data");
                        break;
                    }
                }
                Err(error) => {
                    error!(error = ?error, "Failed to read data");
                    break;
                }
            }
        }

        Ok::<_, ApiError>(())
    });

    // Return a streaming response
    Ok(HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(stream))
}

generate_endpoint! {
    #[tracing::instrument]
    fn completions;
    method: post;
    path: "/completions";
    return_type: HttpResponse;
    error: ApiError;
    docs: {
        tag: "llm",
        context_path: "/ai",

    }
    params: {
        web::Json(dto): web::Json<LlmRequest>
    }
    {
        if dto.stream {
            stream_handler(dto).await
        } else {
            let client = GeminiClient::new(GeminiModel::Gemini2Flash);

            let ai_stream = client.generate_content_iter(dto.contents.0).await?;

            Ok(HttpResponse::Ok().json(ai_stream))
        }
    }
}
