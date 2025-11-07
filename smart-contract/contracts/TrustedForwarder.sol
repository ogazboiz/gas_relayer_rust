// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {ERC2771Forwarder} from "@openzeppelin/contracts/metatx/ERC2771Forwarder.sol";

/**
 * @title TrustedForwarder
 * @notice EIP-2771 compliant meta-transaction forwarder using OpenZeppelin
 */
contract TrustedForwarder is ERC2771Forwarder {
    constructor(string memory name) ERC2771Forwarder(name) {}
}