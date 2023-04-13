mod proto {
    tonic::include_proto!("rpc_demo.nice");
}

use anyhow::Result;
use proto::route_guide_client::RouteGuideClient;
use proto::{Point, Rectangle};
use tonic::{transport::Channel, Request};
use tracing::{debug, info};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing_subscriber();
    info!("client");

    // rpc_say_hello().await?;
    rpc_route_guide().await?;

    Ok(())
}

fn init_tracing_subscriber() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tonic_demo_client=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[allow(dead_code)]
async fn rpc_say_hello() -> Result<()> {
    use proto::greeter_client::GreeterClient;
    use proto::HelloRequest;

    let endpoint = "http://127.0.0.1:6000";
    let mut client = GreeterClient::connect(endpoint).await?;
    debug!("connected to {}", endpoint);

    let request = Request::new(HelloRequest {
        name: "Amiao".to_string(),
    });

    let response = client.say_hello(request).await?;
    let hello_response = response.into_inner();
    debug!(?hello_response);

    Ok(())
}

async fn rpc_route_guide() -> Result<()> {
    let endpoint = "http://127.0.0.1:6000";
    let mut client = RouteGuideClient::connect(endpoint).await?;
    debug!("connected to {}", endpoint);

    // get_feature(&mut client).await?;
    // list_features(&mut client).await?;
    record_route(&mut client).await?;

    Ok(())
}

#[allow(dead_code)]
async fn get_feature(client: &mut RouteGuideClient<Channel>) -> Result<()> {
    let request = Request::new(Point {
        latitude: 408122808,
        longitude: -743999179,
    });

    match client.get_feature(request).await {
        Ok(response) => {
            let name = &response.get_ref().name;
            debug!(?name);
        }
        Err(status) => {
            debug!(?status);
        }
    }

    Ok(())
}

#[allow(dead_code)]
async fn list_features(client: &mut RouteGuideClient<Channel>) -> Result<()> {
    let request = Request::new(Rectangle {
        corner_one: Some(Point {
            latitude: 400000000,
            longitude: -750000000,
        }),
        corner_two: Some(Point {
            latitude: 420000000,
            longitude: -730000000,
        }),
    });

    let mut response_stream = client.list_features(request).await?.into_inner();
    while let Some(feature) = response_stream.message().await? {
        let name = &feature.name;
        debug!(?name);
    }

    Ok(())
}

#[allow(dead_code)]
async fn record_route(client: &mut RouteGuideClient<Channel>) -> Result<()> {
    use rand::rngs::ThreadRng;
    use rand::Rng;

    fn random_point(rng: &mut ThreadRng) -> Point {
        let latitude = (rng.gen_range(0..180) - 90) * 10_000_000;
        let longitude = (rng.gen_range(0..360) - 180) * 10_000_000;
        Point {
            latitude,
            longitude,
        }
    }

    let mut rng = rand::thread_rng();
    let point_count = rng.gen_range(2..100);

    let mut points = Vec::with_capacity(point_count);
    for _ in 0..point_count {
        points.push(random_point(&mut rng));
    }

    debug!("points length: {}", points.len());
    let request = Request::new(tokio_stream::iter(points));

    let response = client.record_route(request).await?;
    let route_summary = response.get_ref();
    debug!(?route_summary);

    Ok(())
}
