use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

/// Handles incoming HTTP requests and returns a response
async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    /// Handle your request logic here
    let response = Response::new(Body::from("Server 2 up and running"));
    Ok(response)
}

/// The main function that sets up and runs the HTTP server
#[tokio::main]
async fn main() {
    println!(
        r#"
                                                                                                                                
    _/_/_/                          _/                _/_/_/                                                            _/_/    
   _/    _/  _/    _/    _/_/_/  _/_/_/_/  _/      _/          _/_/    _/  _/_/  _/      _/    _/_/    _/  _/_/      _/    _/   
  _/_/_/    _/    _/  _/_/        _/                _/_/    _/_/_/_/  _/_/      _/      _/  _/_/_/_/  _/_/              _/      
 _/    _/  _/    _/      _/_/    _/                    _/  _/        _/          _/  _/    _/        _/              _/         
_/    _/    _/_/_/  _/_/_/        _/_/  _/      _/_/_/      _/_/_/  _/            _/        _/_/_/  _/            _/_/_/_/   
                                                                                            | Made by Vishal DESAI - 4SI4 |
"#                                                                                                  
    );

    /// Set the IP and port on wich the server will listen
    let addr = ([127, 0, 0, 1], 1718).into();

    /// Create a make_service function to handle incoming connections
    let make_svc = make_service_fn(|_conn| {
        /// This closure is called once for each connection, and returns a service to handle requests
        async { Ok::<_, hyper::Error>(service_fn(handle_request)) }
    });

    /// Create a server that uses the make_service function to handle incoming connections
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on http://{}", addr);

    /// Start the server and await for it to finish
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
