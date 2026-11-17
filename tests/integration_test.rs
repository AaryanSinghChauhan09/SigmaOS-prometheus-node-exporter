// Integration test for SigmaOS-prometheus-node-exporter
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use prometheus_node_exporter as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
