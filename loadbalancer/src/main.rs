use hyper::{Body, Client, Request, Response, Server, Uri};
use hyper::service::{make_service_fn, service_fn};
use log::{info, warn, error};
use rand::prelude::SliceRandom;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use tokio::time::{interval, Duration};
use std::convert::Infallible;

extern crate dotenv;

/// Struct that stores the informations about the servers as the address and the health
#[derive(Debug, Clone)]
struct ServerInfo {
    address: Uri,
    healthy: bool,
}

/// Struct that stores the status of the loadbalancer
struct LoadBalancerState {
    servers: Vec<ServerInfo>,
    request_count: Arc<Mutex<u32>>,
}

/// Function to check the health of a server
async fn health(server: &ServerInfo) -> bool {
    /// Create a new HTTP client
    let client = Client::new();

    /// make an asynchronous GET request to the server address
    match client.get(server.address.clone()).await {

    /// return true indicating that the server is healthy
        Ok(resp) => resp.status().is_success(),
    
    /// return false indicating that the server is NOT healthy
        Err(_) => false,
    }
}

/// Asynchronous function to perform health checks on all servers at regular intervals
async fn health_check(state: Arc<RwLock<LoadBalancerState>>) {

    /// Loop twith 5sec interval to check the servers health
    let mut interval_timer = interval(Duration::from_secs(5));
    loop {
        interval_timer.tick().await;
        let mut state = state.write().await;
        for server in &mut state.servers {
            /// Perform a health check on the current server asynchronously
            let healthy = health(server).await;
            server.healthy = healthy;

            /// Log results of the health check
            if healthy {
                info!("The server {:?} is up and running |STATUS: OK|", server.address);
            } else {
                error!("The server {:?} is NOT Up /!\\  |STATUS: FAILED|", server.address);
            }
        }
    }
}

/// Asynchronous function to handle incoming HTTP requests
async fn handle_request(_req: Request<Body>, state: Arc<RwLock<LoadBalancerState>>) -> Result<Response<Body>, hyper::Error> {
    let state = state.read().await;

/// Limit the number of requests to 100
    let mut request_count = state.request_count.lock().await;
    if *request_count > 100 {

/// If number of requests > to 100, requests gets rejected
        warn!("Request limit exceeded, rejecting further requests");
        return Ok(Response::new(Body::from("Too many requests, wait a moment")));
    }
    *request_count += 1;

/// Select a healthy server randomly and redirect the request
    let healthy_servers: Vec<&ServerInfo> = state.servers.iter().filter(|s| s.healthy).collect();
    if let Some(server) = healthy_servers.choose(&mut rand::thread_rng()) {
        info!("Redirecting requests at {:?}", server.address);
        let new_uri = format!("{}", server.address);
        Ok(Response::builder()
            .status(302)
            .header("Location", new_uri)
            .body(Body::empty())
            .unwrap())
    } else {
        warn!("No healthy servers available");
        Ok(Response::new(Body::from("No healthy servers available")))
    }
}

#[tokio::main]
async fn main() {
    println!(r#"
     _/_/_/                          _/              _/                                  _/  _/                  _/                                                    
    _/    _/  _/    _/    _/_/_/  _/_/_/_/  _/      _/          _/_/      _/_/_/    _/_/_/  _/_/_/      _/_/_/  _/    _/_/_/  _/_/_/      _/_/_/    _/_/    _/  _/_/   
   _/_/_/    _/    _/  _/_/        _/              _/        _/    _/  _/    _/  _/    _/  _/    _/  _/    _/  _/  _/    _/  _/    _/  _/        _/_/_/_/  _/_/        
  _/    _/  _/    _/      _/_/    _/              _/        _/    _/  _/    _/  _/    _/  _/    _/  _/    _/  _/  _/    _/  _/    _/  _/        _/        _/           
 _/    _/    _/_/_/  _/_/_/        _/_/  _/      _/_/_/_/    _/_/      _/_/_/    _/_/_/  _/_/_/      _/_/_/  _/    _/_/_/  _/    _/    _/_/_/    _/_/_/  _/                                                                                                                                                                                                                                                                                                            
                                                                                                                                   | Made by Vishal DESAI - 4SI4 |                  
    "#);

    dotenv::dotenv().ok();
    env_logger::init();

    /// Initialize the state of the load balancer with server information
    let load_balancer_state = Arc::new(RwLock::new(LoadBalancerState {
        servers: vec![
            ServerInfo { address: "http://127.0.0.1:1717".parse().unwrap(), healthy: true },
            ServerInfo { address: "http://127.0.0.1:1718".parse().unwrap(), healthy: true },
        ],
        request_count: Arc::new(Mutex::new(0)),
    }));

/// Ensure the clone is done before moving it into the async block
    let state_clone_for_health_check = load_balancer_state.clone();
    tokio::spawn(async move {
        health_check(state_clone_for_health_check).await;
    });

/// Create a service function to handle incoming connections
    let make_svc = make_service_fn(move |_conn| {
        /// Clone again here for use in the request handling closure
        let state_clone_for_request = load_balancer_state.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| handle_request(req, state_clone_for_request.clone())))
        }
    });

/// Specify the address on which the load balancer will listen
    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    info!("Load balancer up and running > http://{} |STATUS: OK|", addr);

/// Start the server and handle any errors
    if let Err(e) = server.await {
        error!("Error on server : {} |STATUS: FAILED|", e);
    }
}
