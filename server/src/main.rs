use pretty_env_logger;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    warp::serve(warp::fs::dir("../client/build"))
        .run(([192, 168, 1, 7], 3030))
        .await;
}