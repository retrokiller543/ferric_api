use crate::dto::LlmRequest;
use crate::error::ApiError;
use crate::{api_scope, ApiResult};
use actix_helper_utils::generate_endpoint;
use actix_web::{web, HttpResponse};
use bytes::Bytes;
use futures_util::stream::StreamExt;
use std::sync::LazyLock;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tosic_llm::gemini::{GeminiClient, GeminiModel};
use tosic_llm::LlmProvider;
use tracing::error;

api_scope! {
    pub(super) Ai = "/ai";

    paths: [completions];

    docs: {
        schemas: [LlmRequest];
    }
}

static GEMINI_PROVIDER: LazyLock<LlmProvider<GeminiClient>> = LazyLock::new(|| {
    LlmProvider::new(
        GeminiClient::new(GeminiModel::Gemini2Flash)
            .expect("Failed to construct Gemini2 provider object"),
    )
});

async fn completion_handler(req: LlmRequest) -> ApiResult<HttpResponse> {
    let response = GEMINI_PROVIDER
        .generate(req.contents.into(), req.stream)
        .await?;

    if req.stream {
        let (tx, rx) = mpsc::channel::<ApiResult<Bytes>>(1024);
        let completion_stream = ReceiverStream::new(rx);

        assert!(response.is_stream());

        tokio::spawn(async move {
            let mut stream = response.unwrap_stream();

            while let Some(chunk) = stream.next().await {
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

        Ok(HttpResponse::Ok()
            .content_type("text/event-stream")
            .streaming(completion_stream))
    } else {
        assert!(response.is_static());
        let response = response.unwrap_static();

        Ok(HttpResponse::Ok().json(response))
    }
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
        completion_handler(dto).await
    }
}
