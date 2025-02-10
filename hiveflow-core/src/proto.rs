// src/proto.rs
pub use hiveflow::*;

// Include the generated protobuf code
pub mod hiveflow {
    tonic::include_proto!("hiveflow");
}
