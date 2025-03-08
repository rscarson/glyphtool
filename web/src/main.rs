use axum::{response::Response, routing::post, Json, Router};
use base64::prelude::*;
use libglyphtool::{
    error::EtroisResult,
    lexer::{self, phonambulator::AlwaysAutoSource},
    postprocessor::OutputImage,
    renderer::{bitmap::ToBitmap, GlyphBlockRenderer},
};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() -> EtroisResult<()> {
    let app = Router::new()
        .route("/render", post(render))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let port = get_port();
    println!("Listening on port {port}",);
    let listener = TcpListener::bind(&format!("0.0.0.0:{port}")).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

const DEFAULT_PORT: u16 = 3000;
fn get_port() -> u16 {
    // Search args for --port=
    std::env::args()
        .find_map(|arg| {
            let port = arg.strip_prefix("--port=")?;
            port.parse().ok()
        })
        .unwrap_or(DEFAULT_PORT)
}

async fn render(Json(body): Json<RenderRequest>) -> Json<RenderResponse> {
    println!("Received request to render {} bytes...", body.text.len());

    println!("Translating... ");
    let block = match lexer::parse(&body.text, None, AlwaysAutoSource, false) {
        Ok(block) => block,
        Err(e) => {
            return Json(RenderResponse::Error {
                message: e.to_string(),
            });
        }
    };

    println!("Rendering... ");
    let renderer = GlyphBlockRenderer::new(&block, body.margin as usize);
    let bitmap = renderer.to_bitmap();
    let mut image = OutputImage::new_grayscale(&bitmap);

    println!("Antialiasing... ");
    image.antialias();

    println!("Scaling... ");
    image.scale(body.scale as usize);

    println!("Filtering... ");
    match body.filter.as_deref() {
        Some("sketch") => image.filter_sketch(1.0),
        Some("space") => image.filter_space(1.0),
        Some("granite") => image.filter_granite(1.0),
        _ => {}
    }

    println!("Encoding... ");
    let Some(bytes) = image.into_webp(50.0) else {
        return Json(RenderResponse::Error {
            message: "Failed to convert image to PNG".to_string(),
        });
    };

    println!("Done!");
    let translated = block.to_string();
    let bytes = BASE64_STANDARD.encode(bytes);
    Json(RenderResponse::Success {
        translated,
        bytes,
        format: "webp".to_string(),
    })
}

#[derive(serde::Deserialize)]
pub struct RenderRequest {
    pub text: String,

    #[serde(default = "default_scale")]
    pub scale: u8,

    #[serde(default = "default_margin")]
    pub margin: u8,

    pub filter: Option<String>,
}
fn default_scale() -> u8 {
    5
}
fn default_margin() -> u8 {
    10
}

#[derive(serde::Serialize)]
pub enum RenderResponse {
    Success {
        translated: String,
        bytes: String,
        format: String,
    },
    Error {
        message: String,
    },
}
impl From<RenderResponse> for Response<String> {
    fn from(response: RenderResponse) -> Response<String> {
        let status = match response {
            RenderResponse::Success { .. } => 200,
            RenderResponse::Error { .. } => 500,
        };

        let json = serde_json::to_string(&response).unwrap_or_default();
        let maybe_response = Response::builder()
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .status(status)
            .body(json);

        maybe_response.unwrap_or_else(|_| {
            Response::builder()
                .status(500)
                .body("Internal Server Error".into())
                .unwrap()
        })
    }
}
