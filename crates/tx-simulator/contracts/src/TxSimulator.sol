// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import {IUniswapV3QuoterV2Lib} from "./interfaces/IUniswapV3QuoterV2.sol";
import {IUniswapV3QuoterV2} from "./interfaces/IUniswapV3QuoterV2.sol";
import {ICurvePool} from "./interfaces/ICurvePool.sol";
import {UniswapV2Library} from "./libraries/UniswapV2Library.sol";
import {SafeMath} from "./libraries/SafeMath.sol";

// Credits to https://github.com/solidquant/whack-a-mole
contract TxSimulator {
    using SafeMath for uint256;

    struct SwapParams {
        uint8 protocol; // 0 (UniswapV2), 1 (UniswapV3), 2 (Curve Finance)
        address handler; // UniswapV2: Factory, UniswapV3: Quoter, Curve: Pool
        address tokenIn;
        address tokenOut;
        uint24 fee; // only used in Uniswap V3
        uint256 amount; // amount in (1 USDC = 1,000,000 / 1 MATIC = 1 * 10 ** 18)
    }

    constructor() {}

    function simulateSwapIn(SwapParams[] calldata paramsArray) public returns (uint256) {
        uint256 amountOut;
        uint256 paramsArrayLength = paramsArray.length;

        for (uint256 i; i < paramsArrayLength;) {
            SwapParams memory params = paramsArray[i];

            if (amountOut == 0) {
                amountOut = params.amount;
            } else {
                params.amount = amountOut;
            }

            if (params.protocol == 0) {
                amountOut = simulateUniswapV2SwapIn(params);
            } else if (params.protocol == 1) {
                amountOut = simulateUniswapV3SwapIn(params);
            }

            unchecked {
                ++i;
            }
        }

        return amountOut;
    }

    function simulateUniswapV2SwapIn(SwapParams memory params) public view returns (uint256 amountOut) {
        (uint256 reserveIn, uint256 reserveOut) =
            UniswapV2Library.getReserves(params.handler, params.tokenIn, params.tokenOut);
        amountOut = UniswapV2Library.getAmountOut(params.amount, reserveIn, reserveOut);
    }

    function simulateUniswapV3SwapIn(SwapParams memory params) public returns (uint256 amountOut) {
        IUniswapV3QuoterV2 quoter = IUniswapV3QuoterV2(params.handler);
        IUniswapV3QuoterV2Lib.QuoteExactInputSingleParams memory quoterParams;
        quoterParams.tokenIn = params.tokenIn;
        quoterParams.tokenOut = params.tokenOut;
        quoterParams.amountIn = params.amount;
        quoterParams.fee = params.fee;
        quoterParams.sqrtPriceLimitX96 = 0;
        (amountOut,,,) = quoter.quoteExactInputSingle(quoterParams);
    }

    function simulateCurveSwapIn(SwapParams memory params) public returns (uint256 amountOut) {
        ICurvePool pool = ICurvePool(params.handler);

        int128 i;
        int128 j;

        int128 coinIdx;

        while (i == j) {
            require(coinIdx > 0, "Coin index is out of bounds");
            address coin = pool.coins(uint256(int256(coinIdx)));

            if (coin == params.tokenIn) {
                i = coinIdx;
            } else if (coin == params.tokenOut) {
                j = coinIdx;
            }

            if (i != j) {
                break;
            }

            unchecked {
                ++coinIdx;
            }
        }

        amountOut = pool.get_dy(i, j, params.amount);
    }
}
