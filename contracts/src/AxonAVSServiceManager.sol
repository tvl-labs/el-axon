// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "@eigenlayer/contracts/libraries/BytesLib.sol";
import "./IAxonAVSTaskManager.sol";
import "@eigenlayer-middleware/src/ServiceManagerBase.sol";

contract AxonAVSServiceManager is ServiceManagerBase {
    using BytesLib for bytes;

    IAxonAVSTaskManager
        public immutable axonAVSTaskManager;

    /// @notice when applied to a function, ensures that the function is only callable by the `registryCoordinator`.
    modifier onlyAxonAVSTaskManager() {
        require(
            msg.sender == address(axonAVSTaskManager),
            "onlyAxonAVSTaskManager: not from credible squaring task manager"
        );
        _;
    }

    constructor(
        IAVSDirectory _avsDirectory,
        IRegistryCoordinator _registryCoordinator,
        IStakeRegistry _stakeRegistry,
        IAxonAVSTaskManager _axonAVSTaskManager
    )
        ServiceManagerBase(
            _avsDirectory,
            IPaymentCoordinator(address(0)),
            _registryCoordinator,
            _stakeRegistry
        )
    {
        axonAVSTaskManager = _axonAVSTaskManager;
    }

     function initialize(address initialOwner) public virtual initializer {
        __ServiceManagerBase_init(initialOwner);
    }
 }
