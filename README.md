# iris

[![Build Status](https://travis-ci.org/chris-wood/iris.svg?branch=master)](https://travis-ci.org/chris-wood/iris)

Prototype iris forwarder written in Rust to test CCNx interoperability

## Setup 

Receiver:
  ./receiver 9697 ~/Projects/iris/data/hello_data

Link/Route:
  mk link server tcp 127.0.0.1:9697
  mk route server /hello

Send interest:
  ./pusher 127.0.0.1 9696 ~/Projects/iris/data/hello_int
  cat hello_int | nc 127.0.0.1 9696
