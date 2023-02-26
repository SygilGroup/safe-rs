use std::fmt;

use ethers::types::{H160, H256};

#[derive(Debug)]
pub enum SafeRoute {
    ServiceInfo,
    EthereumRpcInfo,
    EthereumTracingRpcInfo,
    IndexingInfo,
    MasterCopiesInfo,
    Contracts(H160),
    DataDecoder,
    Delegates,
    Messages(H256),
    Modules(H160),
    MultisigTransactions(H256),
    MultisigTransactionsConfirmations(H256),
    SafesByOwner(H160),
    Safe(H160),
    SafeTransactions(H160),
    SafeBalances(H160),
    SafeIncomingTransfers(H160),
    SafeMessages(H160),
    SafeModuleTransactions(H160),
    SafeMultisigTransactions(H160),
    SafeTransfers(H160),
    Tokens,
    Token(H160),
}

impl SafeRoute {
    fn to_string(&self) -> String {
        match self {
            SafeRoute::ServiceInfo => "v1/about/".to_string(),
            SafeRoute::EthereumRpcInfo => "v1/about/ethereum-rpc/".to_string(),
            SafeRoute::EthereumTracingRpcInfo => "v1/about/ethereum-tracing-rpc/".to_string(),
            SafeRoute::IndexingInfo => "v1/about/indexing/".to_string(),
            SafeRoute::MasterCopiesInfo => "v1/about/master-copies/".to_string(),
            SafeRoute::Contracts(address) => format!("v1/contracts/{:?}/", address),
            SafeRoute::DataDecoder => "v1/data-decoder/".to_string(),
            SafeRoute::Delegates => "v1/delegates/".to_string(),
            SafeRoute::Messages(message_id) => format!("v1/messages/{:?}/", message_id),
            SafeRoute::Modules(module_address) => {
                format!("v1/modules/{:?}/safes", module_address)
            }
            SafeRoute::MultisigTransactions(multisig_transaction_id) => {
                format!("v1/multisig-transactions/{:?}/", multisig_transaction_id)
            }
            SafeRoute::MultisigTransactionsConfirmations(multisig_transaction_id) => format!(
                "v1/multisig-transactions/{:?}/confirmations/",
                multisig_transaction_id
            ),
            SafeRoute::SafesByOwner(owner_address) => {
                format!("v1/owners/{:?}/safes/", owner_address)
            }
            SafeRoute::Safe(safe_address) => format!("v1/safes/{}/", safe_address),
            SafeRoute::SafeTransactions(safe_address) => {
                format!("v1/safes/{:?}/all-transactions/", safe_address)
            }
            SafeRoute::SafeBalances(safe_address) => {
                format!("v1/safes/{:?}/balances/usd/", safe_address)
            }
            SafeRoute::SafeIncomingTransfers(safe_address) => {
                format!("v1/safes/{:?}/incoming-transfers/", safe_address)
            }
            SafeRoute::SafeMessages(safe_address) => {
                format!("v1/safes/{:?}/messages/", safe_address)
            }
            SafeRoute::SafeModuleTransactions(safe_address) => {
                format!("v1/safes/{:?}/module-transactions/", safe_address)
            }
            SafeRoute::SafeMultisigTransactions(safe_address) => {
                format!("v1/safes/{:?}/multisig-transactions/", safe_address)
            }
            SafeRoute::SafeTransfers(safe_address) => {
                format!("v1/safes/{:?}/transfers/", safe_address)
            }
            SafeRoute::Tokens => "v1/tokens/".to_string(),
            SafeRoute::Token(token_address) => format!("v1/tokens/{:?}/", token_address),
        }
    }
}

impl fmt::Display for SafeRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
