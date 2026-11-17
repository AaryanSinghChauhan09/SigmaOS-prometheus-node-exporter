//! Integration tests for SigmaOS-prometheus-node-exporter.

use sigmaos_prometheus_node_exporter_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
