// SPDX-License-Identifier: MIT
pragma solidity >= 0.8.7;


contract Arithmetic {

    function divideByTwo(uint256 number) public pure returns (uint256) {
        return number / 2;
    }

    function bitwiseDivide(uint256 number) public pure returns (uint256) {
         /** YOUR CODE HERE */
         return number >> 2;
    }
}