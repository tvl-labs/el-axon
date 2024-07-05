#!/bin/bash

# Global variable to store the PID of the anvil process
ANVIL_PID=0

start() {
    echo "Starting the deployment process..."

    # Step 1: Dump state with anvil and run it in the background
    anvil --dump-state eigenlayer-deployed-anvil-state.json &
    ANVIL_PID=$!

    # Wait a bit to ensure anvil starts properly
    sleep 5

    # Step 2: Change directory to contracts and build
    cd contracts || exit
    forge build

    # Step 3: Change directory to eigenlayer-contracts
    cd lib/eigenlayer-middleware/lib/eigenlayer-contracts || exit

    # Step 4: Run the deployment script
    forge script script/deploy/devnet/M2_Deploy_From_Scratch.s.sol:Deployer_M2 --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --broadcast --sig "run(string memory configFile)" -- M2_deploy_from_scratch.anvil.config.json

    # Step 5: Wait for the script to finish
    # Adjust the sleep duration based on your script's execution time
    sleep 30

    # Step 6: Copy and rename the deployment data
    cp script/output/devnet/M2_from_scratch_deployment_data.json ../../../contracts/script/output/31337/eigenlayer_deployment_output.json

    # Step 7: Change directory back to contracts
    cd ../../../../ || exit

    # Step 8: Run the second deployment script
    forge script script/AxonAVSDeployer.s.sol --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --broadcast -v

    echo "Deployment process completed."
}

stop() {
    echo "Stopping anvil process on port 8545..."

    # Find the process using port 8545 and kill it
    ANVIL_PID=$(lsof -t -i:8545)
    if [ -z "$ANVIL_PID" ]; then
        echo "No anvil process found running on port 8545."
    else
        kill $ANVIL_PID
        echo "Anvil process $ANVIL_PID stopped."
    fi
}

case "$1" in
    start)
        start
        ;;
    stop)
        stop
        ;;
    *)
        echo "Usage: $0 {start|stop}"
        exit 1
        ;;
esac
