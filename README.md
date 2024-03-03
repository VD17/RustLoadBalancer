# Rust Loadbalancer

  ## Whats is a loadbalancer ? 
  A loadbalancer is used to efficiently distribute incoming network traffic or workload across multible servers.
  It prevents a server to get overwhelmed with too much traffic, degradation and potential server crash.
  It can aslo help omptimize ressource usage and minimize response time avoiding failures.

## Author
  Myself

## Summary

- [Installation](#Installation)
     - [prerequisites](#Prerequisites)
       
- [Configuration](#Configuration)
  
- [Usage](#Usage)

- [Perform Attack](#Attack)



## Installation

### Prerequisites

- Rust -> https://www.rust-lang.org/tools/install
- Cargo -> https://doc.rust-lang.org/cargo/getting-started/installation.html

- Clone this repo -> https://github.com/VD17/RustLoadBalance as below: 

```bash
git clone https://github.com/VD17/RustLoadBalancer
cd RustLoadBalancer
```
## Configuration

If you want to use this loadbalancer according to your needs you can make some configurations as below: 

- Servers:
   - Path to file:  loadbalancer/src/bin/server1.rs or loadbalancer/src/bin/server2.rs
     
      You can edit IP and port at lines 27 on both server codes
       ```bash
       let addr = ([127, 0, 0, 1], 1717).into();
       ```
     

- Loadbalancer:
    - Path to file: loadbalancer/src/main.rs
      
       You can edit IP and port at lines 120 on the main.rs code
        ```bash
        let addr = ([127, 0, 0, 1], 8080).into();
        ```

## Usage

To run this projet you have to start the servers: 

```bash
cargo run --bin server1
cargo run --bin server2
```

Now you can perfom the folowing command to start the loadbalancer :

```bash
cargo run
```


## Attack

To perform an attack to see how the loadbalancer deals with the HTTP requests that it recives and sends those to the servers do the following: 

