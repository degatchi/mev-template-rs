pragma solidity ^0.8.13;

abstract contract Ownable {
    address public owner;

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    /**
     * @dev Initializes the contract setting the deployer as the initial owner.
     */
    constructor() {
        owner = msg.sender;
    }

    /**
     * @dev Throws if called by any account other than the owner.
     */
    modifier onlyOwner() {
        require(owner == msg.sender, "Ownable: caller is not the owner");
        _;
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Can only be called by the current owner.
     */
    function transferOwnership(address newOwner) public virtual onlyOwner {
        emit OwnershipTransferred(owner, newOwner);    
        owner = newOwner;
    }
}

// --------------------------------------------------------------------------------------
//
// Trigger 031/08/2022 | SPDX-License-Identifier: MIT
// Designed by, DeGatchi (https://github.com/DeGatchi).
//
// --------------------------------------------------------------------------------------

interface IERC20 {
    function balanceOf(address account) external view returns (uint);
    function allowance(address owner, address spender) external view returns (uint);
    function approve(address spender, uint amount) external returns (bool);
    function transfer(address recipient, uint amount) external returns (bool);
    function name() external returns (string memory);
    function symbol() external returns (string memory);
}

interface IFactory {
    function getPair(address tokenA, address tokenB) external view returns (address pair);
    function allPairs(uint256) external view returns (address pair);
    function allPairsLength() external view returns (uint256);
}

interface IPair {
    function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
    function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external;
    function token0() external view returns (address);
    function token1() external view returns (address);
}

interface IUniV2 {
    function swapExactTokensForTokens(uint amountIn, uint amountOutMin, address[] calldata path, address to, uint deadline) external returns (uint[] memory amounts);
    function getAmountIn(uint256 amountOut, uint256 reserveIn, uint256 reserveOut) external view returns (uint256 amountIn);
    function getAmountOut(uint256 amountIn, uint256 reserveIn, uint256 reserveOut) external pure returns (uint256 amountOut);
    function getAmountsOut(uint amountIn, address[] memory path) external view returns (uint[] memory amounts);
}

error Fail();
contract Trigger is Ownable {



    /// >>>>>>>>>>>>>>>>>>>>>> Getters <<<<<<<<<<<<<<<<<<<<<<<< ///
    
    struct Reserves {
        uint112 reserves0;
        uint112 reserves1;
    }

    struct Pair {
        address pair;
        address t0;
        address t1;
        uint128 r0;
        uint128 r1;
    }

    /// @notice Returns the reserves of each pair given.
    function getData(IPair[] calldata p) external view returns(Reserves[] memory r){
        r = new Reserves[](p.length);
        for (uint i; i < p.length;) {  
            (r[i].reserves0, r[i].reserves1, ) = p[i].getReserves();
            unchecked { ++i; }
        }
    }

    /// @notice Returns the pair length of each factory given.
    function getFactoryLen(IFactory[] calldata factory) external view returns(uint[] memory l) {
        l = new uint[](factory.length);
        for (uint i; i < factory.length; ) {
            l[i] = factory[i].allPairsLength();
            unchecked { ++i; }
        }
    }

    /// @notice Returns pairs `from` to `to` for a factory.
    function getFactoryPairs(IFactory factory, uint from, uint to) external view returns(Pair[] memory pairs) {
        uint len = to - from;
        uint counter;
        pairs = new Pair[](len);

        for (uint i = from; i < to;) {
            address pair = factory.allPairs(i);
            (uint128 r0, uint128 r1, ) = IPair(pair).getReserves();
            
            pairs[counter] = Pair({
                pair: pair,
                t0: IPair(pair).token0(),
                t1: IPair(pair).token1(),
                r0: r0,
                r1: r1
            });

            unchecked { 
                ++counter;
                ++i; 
            }
        }
    }

    /// @notice Returns the ERC20 token bal of the contract.
    function balanceOf(IERC20 token) external view returns(uint256) {
        return token.balanceOf(address(this));
    }

    /// @notice Returns ETH bal of the contract.
    function contractBal() public view returns(uint256) {
        return address(this).balance;
    }   

    /// >>>>>>>>>>>>>>>>>>>>>> Withdraw <<<<<<<<<<<<<<<<<<<<<<<< ///

    /// @notice Transfers ERC20 token out of contract.
    function withdrawToken(IERC20 token, address to, uint256 amount, bool max) external onlyOwner returns(bool) {
        if (max) amount = token.balanceOf(address(this));
        token.transfer(to, amount);
        return true;
    }

    /// @notice Transfers ETH out of contract.
    function withdrawEth(address payable to) external onlyOwner returns(bool) {
        require(address(this).balance > 0, "contract has an empty ETH balance");
        to.transfer(address(this).balance);
        return true;
    }

    receive() external payable {}
}