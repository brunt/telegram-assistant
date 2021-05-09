use prost_build;

fn main() {
    prost_build::compile_protos(&["proto/metro-schedule.proto", "proto/spending-tracker.proto"], &["proto/"]).unwrap();
}