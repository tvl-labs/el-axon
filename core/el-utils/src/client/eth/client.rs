use ethers::providers::{Http, Provider};

#[derive(Debug)]
pub struct EthClient {
    client:             Provider<Http>,
    client_and_version: String,
}

impl EthClient {
    pub fn new(client: Provider<Http>, client_type: &str) -> Self {
        let client_and_version = format!("{}-{}", client_type, "0.1.0");
        return EthClient {
            client,
            client_and_version,
        };
    }
}
