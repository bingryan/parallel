#![allow(non_camel_case_types)]

use super::{BalanceOf, Config};
use codec::{Decode, Encode, MaxEncodedLen};
use cumulus_primitives_core::ParaId;
use scale_info::TypeInfo;
use sp_runtime::traits::StaticLookup;
use sp_runtime::{MultiSignature, RuntimeDebug};
use sp_std::{boxed::Box, marker::PhantomData};

// 0 == create
// 1 == contribute
// 2 == withdraw
// 3 == refund

// https://github.com/paritytech/polkadot/blob/4fdec8348ed275088f9d31dce8d212addea8ef5b/runtime/kusama/src/lib.rs#L1559
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum WestendCall<T: Config> {
    #[codec(index = 64)]
    Crowdloan(CrowdloanCall<T>),
    #[codec(index = 4)]
    Balances(BalancesCall<T>),
    #[codec(index = 16)]
    Utility(Box<UtilityCall<Self>>),
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum KusamaCall<T: Config> {
    #[codec(index = 73)]
    Crowdloan(CrowdloanCall<T>),
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum PolkadotCall<T: Config> {
    #[codec(index = 73)]
    Crowdloan(CrowdloanCall<T>),
}

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum CrowdloanCall<T: Config> {
    #[codec(index = 1)]
    Contribute(CrowdloanContributeCall<T>),
    #[codec(index = 2)]
    Withdraw(CrowdloanWithdrawCall<T>),
    #[codec(index = 3)]
    Refund(CrowdloanRefundCall<T>),
}

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct CrowdloanContributeCall<T: Config> {
    /// - `crowdloan`: The crowdloan who you are contributing to
    #[codec(compact)]
    pub index: ParaId,
    /// - `value`: The amount of tokens you want to contribute to a parachain.
    #[codec(compact)]
    pub value: BalanceOf<T>,
    // `signature`: The signature if the fund has a verifier
    pub signature: Option<MultiSignature>,
}

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct CrowdloanWithdrawCall<T: Config> {
    /// - `who`: The account whose contribution should be withdrawn.
    pub who: T::AccountId,
    /// - `index`: The parachain to whose crowdloan the contribution was made.
    #[codec(compact)]
    pub index: ParaId,
}

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct CrowdloanRefundCall<T: Config> {
    /// - `index`: The parachain to whose crowdloan the contribution was made.
    #[codec(compact)]
    pub index: ParaId,
    pub _ghost: PhantomData<T>,
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum ProxyCall<T: Config> {
    #[codec(index = 0)]
    proxy(ProxyproxyCall<T>),
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct ProxyproxyCall<T: Config> {
    pub real: T::AccountId,
    pub force_proxy_type: Option<ProxyType>,
    pub call: Box<<T as frame_system::Config>::Call>,
}

/// The type used to represent the kinds of proxying allowed.
#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Encode,
    Decode,
    RuntimeDebug,
    MaxEncodedLen,
    TypeInfo,
)]
pub enum ProxyType {
    Any,
    NonTransfer,
    Governance,
    Staking,
    IdentityJudgement,
    CancelProxy,
    Auction,
}
impl Default for ProxyType {
    fn default() -> Self {
        Self::Any
    }
}

/// Relaychain balances.transfer_keep_alive call arguments
#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct BalancesTransferKeepAliveCall<T: Config> {
    /// dest account
    pub dest: <T::Lookup as StaticLookup>::Source,
    /// transfer amount
    #[codec(compact)]
    pub value: BalanceOf<T>,
}

/// Relaychain balances.transfer_all call arguments
#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct BalancesTransferAllCall<T: Config> {
    /// dest account
    pub dest: <T::Lookup as StaticLookup>::Source,
    pub keep_alive: bool,
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum BalancesCall<T: Config> {
    #[codec(index = 3)]
    TransferKeepAlive(BalancesTransferKeepAliveCall<T>),
    #[codec(index = 4)]
    TransferAll(BalancesTransferAllCall<T>),
}

/// Relaychain utility.as_derivative call arguments
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct UtilityAsDerivativeCall<RelaychainCall> {
    /// derivative index
    pub index: u16,
    /// call
    pub call: RelaychainCall,
}

/// Relaychain utility.batch_all call arguments
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct UtilityBatchAllCall<RelaychainCall> {
    /// calls
    pub calls: Vec<RelaychainCall>,
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum UtilityCall<RelaychainCall> {
    #[codec(index = 1)]
    AsDerivative(UtilityAsDerivativeCall<RelaychainCall>),
    #[codec(index = 2)]
    BatchAll(UtilityBatchAllCall<RelaychainCall>),
}