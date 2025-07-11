use futures_util::{StreamExt, stream::BoxStream};
use poem::{Route, Server, listener::TcpListener};
use poem_openapi::{Object, OpenApi, OpenApiService, payload::EventStream};
use tokio::time::Duration;

#[derive(Object)]
struct Event {
    value: i32,
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/events", method = "get")]
    async fn index(&self) -> EventStream<BoxStream<'static, Event>> {
        EventStream::new(
            async_stream::stream! {
                for i in 0.. {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    yield Event { value: i };
                }
            }
            .boxed(),
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(Route::new().nest("/api", api_service).nest("/", ui))
        .await
}
