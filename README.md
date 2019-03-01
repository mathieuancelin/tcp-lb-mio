# TCP LB

## Build

```sh
$ cargo build --release
```

## Usage

```sh
$ tcp-lb -h

Usage:
    tcp-lb [OPTIONS] [SERVER ...]

TCP load balancer

positional arguments:
  server                Servers to load balance

optional arguments:
  -h,--help             show this help message and exit
  -b,--bind BIND        Bind the load balancer to address:port (127.0.0.1:8000)
  -l,--log LOG          Log level [debug, info, warn, error] (info)

$ tcp-lb 13.33.147.237:80 13.33.147.118:80 13.33.147.124:80 -l error
$ curl -H 'Host: neverssl.com' http://127.0.0.1:8000
```

    

[package]
name = "tcp-lb"
version = "1.0.0"

[dependencies]
log = "0.3.1"
argparse = "0.2.1"
mio = "0.4.3"
env_logger = "0.3.1"
