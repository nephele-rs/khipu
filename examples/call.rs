syntax = "proto3";

// This is bad: this file is not in a directory called "echo."
// Normally, protobuf files should reside in a directory matching the package name.
// It's fine for this particular test case though.
package echo;

// The Echo service. This service returns back the same data that it is given.
service Echo {
  // Echoes back the data sent, unmodified.
  rpc Echo (EchoRequest) returns (EchoResponse);
}

// The request for an `Echo.Echo` call.
message EchoRequest {
  // The data to be echoed back.
  bytes data = 1;
  
}

// The response for an `Echo.Echo` call.
message EchoResponse {
  // The echoed back data from `EchoRequest.data`.
  bytes data = 1;
}
