#![allow(dead_code, unused)]

use crate::types::Mismatch;
use ethers_core::types::{Bytes, ParseBytesError};
use ethers_solc::CompilerOutput;
use std::str::FromStr;
use thiserror::Error;

/// Errors that may occur during initial [`Verifier`] setup
/// with input data provided by the requester.
#[derive(Clone, Debug, PartialEq, Error)]
pub(crate) enum InitializationError {
    #[error(
        "creation transaction input is invalid (either is empty or is not a valid hex string): {0}"
    )]
    InvalidCreationTxInput(String),
    #[error("deployed bytecode is invalid (either is empty or is not a valid hex string): {0}")]
    InvalidDeployedBytecode(String),
    #[error("creation transaction input has different metadata hash to deployed bytecode. {0}")]
    MetadataHashMismatch(Mismatch<Bytes>),
}

/// Errors that may occur during bytecode comparison step.
#[derive(Clone, Debug, Error)]
pub(crate) enum VerificationError {}

/// Wrapper under `evm.deployedBytecode` from the standard output JSON
/// (https://docs.soliditylang.org/en/latest/using-the-compiler.html#output-description).
///
/// Provides an interface to retrieve parts the deployed bytecode consists of:
/// actual bytecode participating in EVM transaction execution and optionally metadata hash.
#[derive(Clone, Debug, PartialEq)]
struct DeployedBytecode {}

impl FromStr for DeployedBytecode {
    type Err = InitializationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

/// Wrapper under `evm.bytecode.object` from the standard output JSON
/// (https://docs.soliditylang.org/en/latest/using-the-compiler.html#output-description)
/// excluding metadata hash and optionally including constructor arguments used on a contract creation.
#[derive(Clone, Debug, PartialEq)]
struct BytecodeWithConstructorArgs {}

impl BytecodeWithConstructorArgs {
    /// Initializes the structure from string and parsed deployed bytecode.
    /// It extracts metadata hash from the provided string and extracts
    /// constructor arguments used on a contract creation if possible.
    ///
    /// Deployed bytecode is required to extract metadata hash from the string.
    pub fn from_str(
        s: &str,
        deployed_bytecode: &DeployedBytecode,
    ) -> Result<Self, InitializationError> {
        todo!()
    }
}

/// Verifier used in contract verification.
///
/// Contains input data provided by the requester that will
/// further be used in verification process.
#[derive(Clone, Debug)]
pub(crate) struct Verifier {
    /// Name of the contract to be verified
    contract_name: String,
    /// File path contract to be verified is located at
    /// (useful if multiple files contain contract with `contract_name`)
    file_path: Option<String>,
    /// Bytecode used on the contract creation transaction
    bc_creation_tx_input: BytecodeWithConstructorArgs,
    /// Bytecode stored in the chain and being used by EVM
    bc_deployed_bytecode: DeployedBytecode,
}

impl Verifier {
    /// Instantiates a new verifier instance with input data provided by the requester.
    ///
    /// Returns [`InitializationError`] inside [`Err`] if either `deployed_bytecode` or `creation_tx_input` are invalid.
    pub fn new(
        contract_name: String,
        file_path: Option<String>,
        creation_tx_input: &str,
        deployed_bytecode: &str,
    ) -> Result<Self, InitializationError> {
        let deployed_bytecode = DeployedBytecode::from_str(deployed_bytecode)?;
        let bytecode =
            BytecodeWithConstructorArgs::from_str(creation_tx_input, &deployed_bytecode)?;

        Ok(Self {
            contract_name,
            file_path,
            bc_deployed_bytecode: deployed_bytecode,
            bc_creation_tx_input: bytecode,
        })
    }

    /// Verifies input data provided on initialization by comparing it
    /// with compiler output received when compiling source data locally.
    ///
    /// If verification succeeds return [`Ok`], otherwise when verification
    /// fails return an [`VerificationError`] inside [`Err`].
    pub fn verify(&self, output: CompilerOutput) -> Result<(), VerificationError> {
        todo!()
    }
}

#[cfg(test)]
mod verifier_initialization_tests {
    use super::*;
    use const_format::concatcp;

    const DEFAULT_CONTRACT_NAME: &'static str = "Contract";

    const DEFAULT_CONSTRUCTOR_ARGS: &'static str =
        "0000000000000000000000000000000000000000000000000000000000000fff";
    // {"ipfs": h'1220EB23CE2C13EA8739368F952F6C6A4B1F0623D147D2A19B6D4D26A61AB03FCD3E', "solc": 0.8.14}
    const DEFAULT_ENCODED_METADATA_HASH: &'static str = "a2646970667358221220eb23ce2c13ea8739368f952f6c6a4b1f0623d147d2a19b6d4d26a61ab03fcd3e64736f6c634300080e0033";
    const DEFAULT_BYTECODE_WITHOUT_METADATA_HASH: &'static str = "608060405234801561001057600080fd5b5060405161022038038061022083398101604081905261002f91610074565b600080546001600160a01b0319163390811782556040519091907f342827c97908e5e2f71151c08502a66d44b6f758e3ac2f1de95f02eb95f0a735908290a35061008d565b60006020828403121561008657600080fd5b5051919050565b6101848061009c6000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063893d20e81461003b578063a6f9dae11461005a575b600080fd5b600054604080516001600160a01b039092168252519081900360200190f35b61006d61006836600461011e565b61006f565b005b6000546001600160a01b031633146100c35760405162461bcd60e51b815260206004820152601360248201527221b0b63632b91034b9903737ba1037bbb732b960691b604482015260640160405180910390fd5b600080546040516001600160a01b03808516939216917f342827c97908e5e2f71151c08502a66d44b6f758e3ac2f1de95f02eb95f0a73591a3600080546001600160a01b0319166001600160a01b0392909216919091179055565b60006020828403121561013057600080fd5b81356001600160a01b038116811461014757600080fd5b939250505056fe";
    const DEFAULT_DEPLOYED_BYTECODE_WITHOUT_METADATA_HASH: &'static str =  "608060405234801561001057600080fd5b50600436106100365760003560e01c8063893d20e81461003b578063a6f9dae11461005a575b600080fd5b600054604080516001600160a01b039092168252519081900360200190f35b61006d61006836600461011e565b61006f565b005b6000546001600160a01b031633146100c35760405162461bcd60e51b815260206004820152601360248201527221b0b63632b91034b9903737ba1037bbb732b960691b604482015260640160405180910390fd5b600080546040516001600160a01b03808516939216917f342827c97908e5e2f71151c08502a66d44b6f758e3ac2f1de95f02eb95f0a73591a3600080546001600160a01b0319166001600160a01b0392909216919091179055565b60006020828403121561013057600080fd5b81356001600160a01b038116811461014757600080fd5b939250505056fe";

    const DEFAULT_CREATION_TX_INPUT: &'static str = concatcp!(
        DEFAULT_BYTECODE_WITHOUT_METADATA_HASH,
        DEFAULT_ENCODED_METADATA_HASH,
        DEFAULT_CONSTRUCTOR_ARGS
    );
    const DEFAULT_DEPLOYED_BYTECODE: &'static str = concatcp!(
        DEFAULT_DEPLOYED_BYTECODE_WITHOUT_METADATA_HASH,
        DEFAULT_ENCODED_METADATA_HASH
    );

    #[test]
    #[should_panic] // TODO: remove when implemented
    fn test_initialization_with_valid_data() {
        let verifier = Verifier::new(
            DEFAULT_CONTRACT_NAME.to_string(),
            None,
            DEFAULT_CREATION_TX_INPUT,
            DEFAULT_DEPLOYED_BYTECODE,
        );
        assert!(verifier.is_ok(), "Initialization with \"0x\" prefix failed");

        let verifier = Verifier::new(
            DEFAULT_CONTRACT_NAME.to_string(),
            None,
            &concatcp!("0x", DEFAULT_CREATION_TX_INPUT),
            &concatcp!("0x", DEFAULT_DEPLOYED_BYTECODE),
        );
        assert!(
            verifier.is_ok(),
            "Initialization without \"0x\" prefix failed"
        );
    }

    #[test]
    #[should_panic] // TODO: remove when implemented
    fn test_initialization_with_empty_creation_tx_input_should_fail() {
        let verifier = Verifier::new(
            DEFAULT_CONTRACT_NAME.to_string(),
            None,
            "",
            DEFAULT_DEPLOYED_BYTECODE,
        );
        assert!(verifier.is_err(), "Verifier initialization should fail");
        assert_eq!(
            verifier.unwrap_err(),
            InitializationError::InvalidCreationTxInput("".to_string())
        )
    }

    #[test]
    #[should_panic] // TODO: remove when implemented
    fn test_initialization_with_creation_tx_input_as_invalid_hex_should_fail() {
        let invalid_input = "0xabcdefghij";
        let verifier = Verifier::new(
            DEFAULT_CONTRACT_NAME.to_string(),
            None,
            invalid_input,
            DEFAULT_DEPLOYED_BYTECODE,
        );
        assert!(verifier.is_err(), "Verifier initialization should fail");
        assert_eq!(
            verifier.unwrap_err(),
            InitializationError::InvalidCreationTxInput(invalid_input.to_string())
        )
    }

    #[test]
    #[should_panic] // TODO: remove when implemented
    fn test_initialization_with_empty_deployed_bytecode_should_fail() {
        let verifier = Verifier::new(
            DEFAULT_CONTRACT_NAME.to_string(),
            None,
            DEFAULT_CREATION_TX_INPUT,
            "",
        );
        assert!(verifier.is_err(), "Verifier initialization should fail");
        assert_eq!(
            verifier.unwrap_err(),
            InitializationError::InvalidDeployedBytecode("".to_string())
        )
    }

    #[test]
    #[should_panic] // TODO: remove when implemented
    fn test_initialization_with_deployed_bytecode_as_invalid_hex_should_fail() {
        let invalid_input = "0xabcdefghij";
        let verifier = Verifier::new(
            DEFAULT_CONTRACT_NAME.to_string(),
            None,
            DEFAULT_CREATION_TX_INPUT,
            invalid_input,
        );
        assert!(verifier.is_err(), "Verifier initialization should fail");
        assert_eq!(
            verifier.unwrap_err(),
            InitializationError::InvalidDeployedBytecode(invalid_input.to_string())
        )
    }

    #[test]
    #[should_panic] // TODO: remove when implemented
    fn test_initialization_with_metadata_hash_mismatch_should_fail() {
        // {"ipfs": h'1220EB23CE2C13EA8739368F952F6C6A4B1F0623D147D2A19B6D4D26A61AB03FCD3E', "solc": 0.8.0}
        let another_metadata_hash = "a2646970667358221220eb23ce2c13ea8739368f952f6c6a4b1f0623d147d2a19b6d4d26a61ab03fcd3e64736f6c63430008000033";
        let verifier = Verifier::new(
            DEFAULT_CONTRACT_NAME.to_string(),
            None,
            &format!(
                "{}{}",
                DEFAULT_BYTECODE_WITHOUT_METADATA_HASH, another_metadata_hash
            ),
            DEFAULT_DEPLOYED_BYTECODE,
        );
        assert!(verifier.is_err(), "Verifier initialization should fail");
        assert_eq!(
            verifier.unwrap_err(),
            InitializationError::MetadataHashMismatch(Mismatch::expected(
                Bytes::from_str(DEFAULT_ENCODED_METADATA_HASH).unwrap()
            ))
        );
    }
}
