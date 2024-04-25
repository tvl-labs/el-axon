pub mod avs_dictionary;
pub mod axon_avs_deployer;
pub mod axon_avs_service_manager;
pub mod axon_avs_task_manager;
pub mod bls_apk_registry;
pub mod delegation_manager;
pub mod erc20_mock;
pub mod i_slasher;
pub mod i_strategy;
pub mod operator_state_retriever;
pub mod registry_coordinator;
pub mod service_manager_base;
pub mod stake_registry;
pub mod strategy_manager;

#[test]
fn test() {
    use ethers::contract::Abigen;

    Abigen::new("ERC20Mock", "../../avs-contract/ERC20Mock.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("./src/contract/erc20_mock.rs")
        .unwrap();
}
