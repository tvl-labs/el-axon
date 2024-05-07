// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import "../src/AxonAVSServiceManager.sol" as aasm;
import {AxonAVSTaskManager} from "../src/AxonAVSTaskManager.sol";
// import {IRegistryCoordinator} from "@eigenlayer-middleware/src/interfaces/IRegistryCoordinator.sol";
import {BLSMockAVSDeployer} from "@eigenlayer-middleware/test/utils/BLSMockAVSDeployer.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {BitmapUtils} from "@eigenlayer-middleware/src/libraries/BitmapUtils.sol";
import "forge-std/console.sol";

contract AxonAVSTaskManagerTest is BLSMockAVSDeployer {
    // EVENTS
    event NewTaskCreated(uint32 indexed taskIndex, AxonAVSTaskManager.Task task);

    event TaskResponded(
        AxonAVSTaskManager.TaskResponse taskResponse,
        AxonAVSTaskManager.TaskResponseMetadata taskResponseMetadata
    );

    event TaskCompleted(uint32 indexed taskIndex);

    aasm.AxonAVSServiceManager sm;
    aasm.AxonAVSServiceManager smImplementation;
    AxonAVSTaskManager tm;
    AxonAVSTaskManager tmImplementation;

    uint32 public constant TASK_RESPONSE_WINDOW_BLOCK = 30;
    address aggregator =
        address(uint160(uint256(keccak256(abi.encodePacked("aggregator")))));
    address generator =
        address(uint160(uint256(keccak256(abi.encodePacked("generator")))));

    function setUp() public {
        _setUpBLSMockAVSDeployer();

        tmImplementation = new AxonAVSTaskManager(
            aasm.IRegistryCoordinator(address(registryCoordinator)),
            TASK_RESPONSE_WINDOW_BLOCK
        );

        // Third, upgrade the proxy contracts to use the correct implementation contracts and initialize them.
        tm = AxonAVSTaskManager(
            address(
                new TransparentUpgradeableProxy(
                    address(tmImplementation),
                    address(proxyAdmin),
                    abi.encodeWithSelector(
                        tm.initialize.selector,
                        pauserRegistry,
                        registryCoordinatorOwner,
                        aggregator,
                        generator
                    )
                )
            )
        );
    }

    function testCreateNewTask_InvalidSender() public {
        bytes memory quorumNumbers = new bytes(0);
        bytes memory proof = bytes("0x1234");
        vm.expectRevert("Task generator must be the caller");
        tm.createNewTask(proof, 100, quorumNumbers);
    }

    function testCreateNewTask() public {
        bytes memory quorumNumbers = new bytes(0);
        cheats.prank(generator, generator);
        bytes memory proof = bytes("0x1234");
        uint32 blockNumber = uint32(block.number);
        AxonAVSTaskManager.Task memory task;
        task.proof = proof;
        task.taskCreatedBlock = blockNumber;
        task.quorumNumbers = quorumNumbers;
        task.quorumThresholdPercentage = 100;

        vm.expectEmit(address(tm));
        emit NewTaskCreated(0, task);
        tm.createNewTask(proof, 100, quorumNumbers);
        assertEq(tm.latestTaskNum(), 1);
        assertEq(tm.getProof(blockNumber), proof);
    }

    function testRespondToTask_InvalidSender() public {
        AxonAVSTaskManager.Task memory task;
        AxonAVSTaskManager.TaskResponse memory taskResponse;
        AxonAVSTaskManager.NonSignerStakesAndSignature memory nonSignerStakesAndSignature;

        vm.expectRevert("Aggregator must be the caller");
        tm.respondToTask(task, taskResponse, nonSignerStakesAndSignature);
    }

    function testRespondToTask_SuppliedTaskMismatch() public {
        AxonAVSTaskManager.Task memory task;
        AxonAVSTaskManager.TaskResponse memory taskResponse;
        AxonAVSTaskManager.NonSignerStakesAndSignature memory nonSignerStakesAndSignature;

        bytes memory quorumNumbers = new bytes(0);
        bytes memory proof = bytes("0x1234");

        //creating the task
        testCreateNewTask();

        // Let's change the task, which we used while creating the task
        task.proof = proof;
        task.taskCreatedBlock = uint32(block.number);
        task.quorumNumbers = quorumNumbers;
        // here we changed the quorumThresholdPercentage from 100 to 90
        task.quorumThresholdPercentage = 90;

        vm.expectRevert("supplied task does not match the one recorded in the contract");
        cheats.prank(aggregator, aggregator);
        tm.respondToTask(task, taskResponse, nonSignerStakesAndSignature);
    }
    
}
