// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           27
// Async Callback (empty):               1
// Total number of exported functions:  29

#![no_std]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    liquidity_pool
    (
        getPoolAsset
        getReserves
        getRewardsReserves
        getLendToken
        borrowToken
        getPoolParams
        getTotalBorrow
        getLiquidationThreshold
        getBorrowIndex
        getSupplyIndex
        borrowIndexLastUpdateRound
        getAccountToken
        getAccountPositions
        updateCollateralWithInterest
        updateBorrowsWithDebt
        addCollateral
        borrow
        remove_collateral
        repay
        sendTokens
        getCapitalUtilisation
        getTotalSuppliedCapital
        getDebtInterest
        getDepositRate
        getBorrowRate
        setPriceAggregatorAddress
        getAggregatorAddress
    )
}

dharitri_sc_wasm_adapter::empty_callback! {}
