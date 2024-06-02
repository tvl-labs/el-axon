use el_utils::client::ClientBuilder;
use ethers_core::types::H160;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let builder = ClientBuilder {
        eth_http_url:                 "http://localhost:8545".to_string(),
        eth_ws_url:                   "http://localhost:8545".to_string(),
        bls_apk_registry_addr:        Default::default(),
        operator_registry_addr:       Default::default(),
        registry_coordinator_addr:    H160::from_str("0xc3e53F4d16Ae77Db1c982e75a937B9f60FE63690")
            .unwrap(),
        avs_name:                     "".to_string(),
        from_metrics_ip_port_address: "".to_string(),
        sender:                       H160::from_str("0x860B6912C2d0337ef05bbC89b0C2CB6CbAEAB4A5")
            .unwrap(),
        private_key:
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
    };

    builder.build().await.run().await;
}
