use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use url::Url;

/// A simple Spin HTTP component.
#[http_component]
fn handle_comp_web(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let full_url = req.header("spin-full-url").unwrap_or("");
    let path = Url::parse(full_url)?.path();

    let response = match path {
        "/app" => Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body("app")
            .build(),

        _ => Response::builder()
            .status(404)
            .header("content-type", "text/plain")
            .body("Not found")
            .build(),
    };

    Ok(response)
}
