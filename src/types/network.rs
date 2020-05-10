/// Bitcoin/shitcoin network
///
/// This enum is `non_exhaustive` to allow adding new kinds of test or shitcoin networks in the
/// future.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "displaydoc", derive(displaydoc::Display))]
#[non_exhaustive]
pub enum Network {
    /// mainnet
    Mainnet,
    /// testnet
    Testnet,
    /// regtest
    Regtest,
    /// namecoin
    Namecoin,
}
