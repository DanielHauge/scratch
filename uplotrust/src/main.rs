use futures::{SinkExt, StreamExt};
use rand::{Rng, SeedableRng, rngs::SmallRng};
use serde::Serialize;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;
use tokio::{sync::mpsc, time::sleep};
use warp::Filter;
use warp::ws::{Message, WebSocket};

#[derive(Serialize)]
struct SensorData {
    value: f64,
    timestamp: u128,
}

type Tx = mpsc::UnboundedSender<Message>;
type Clients = Arc<Mutex<Vec<Tx>>>;

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(Mutex::new(Vec::new()));

    for i in 0..5 {
        let clients = clients.clone();
        tokio::spawn(async move {
            let mut rng = SmallRng::from_os_rng();
            loop {
                let value = match i % 3 {
                    0 => {
                        ((SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as f64
                            / 1000.0)
                            .sin())
                            * 10.0
                    }
                    1 => rng.gen_range(-5.0..5.0),
                    _ => rng.gen_range(0.0..1.0) + rng.gen_range(-0.5..0.5),
                };

                let msg = SensorData {
                    value,
                    timestamp: SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                };
                let json = serde_json::to_string(&msg).unwrap();

                let mut lock = clients.lock().await;
                lock.retain(|tx| tx.send(Message::text(json.clone())).is_ok());

                sleep(tokio::time::Duration::from_millis(10)).await;
            }
        });
    }

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .map(|ws: warp::ws::Ws, clients| {
            ws.on_upgrade(move |socket| handle_socket(socket, clients))
        });

    let html_route = warp::path::end().map(|| warp::reply::html(include_str!("static/index.html")));

    println!("Visit http://localhost:3030");
    warp::serve(ws_route.or(html_route))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn with_clients(
    clients: Clients,
) -> impl Filter<Extract = (Clients,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

async fn handle_socket(ws: WebSocket, clients: Clients) {
    let (mut tx_ws, mut rx_ws) = ws.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    clients.lock().await.push(tx);

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if tx_ws.send(msg).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(_)) = rx_ws.next().await {}
}
