//! Error types

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
    sanitize::SanitizeError,
};
use thiserror::Error;

/// Errors that may be returned by the Template program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum AudiusProgramError {
    /// Signature with an already met principal
    #[error("Signature with an already met principal")]
    SignCollission,

    /// Unexpected signer met
    #[error("Unexpected signer met")]
    WrongSigner,

    /// Wrong sender account
    #[error("Incorect sender account")]
    IncorectSenderAccount,

    /// Wrong manager account
    #[error("Incorect account manager")]
    IncorectManagerAccount,

    /// Wrong reward manager key
    #[error("Wrong reward manager key")]
    WrongRewardManagerKey,

    /// Wrong recipient Solana key
    #[error("Wrong recipient Solana key")]
    WrongRecipientKey,

    /// Isn't enough signers keys
    #[error("Isn't enough signers keys")]
    NotEnoughSigners,

    /// Secp256 instruction missing
    #[error("Secp256 instruction missing")]
    Secp256InstructionMissing,

    /// Instruction load error
    #[error("Instruction load error")]
    InstructionLoadError,

    /// Repeated senders
    #[error("Repeated sender")]
    RepeatedSenders,

    /// Signature verification failed
    #[error("Signature verification failed")]
    SignatureVerificationFailed,

    /// Some signers have same operators
    #[error("Some signers have same operators")]
    OperatorCollision,
}
impl From<AudiusProgramError> for ProgramError {
    fn from(e: AudiusProgramError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for AudiusProgramError {
    fn type_of() -> &'static str {
        "AudiusProgramError"
    }
}

impl PrintProgramError for AudiusProgramError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        msg!(&self.to_string())
    }
}

/// Convert SanitizeError to AudiusProgramError
pub fn to_audius_program_error(_e: SanitizeError) -> AudiusProgramError {
    AudiusProgramError::InstructionLoadError
}
