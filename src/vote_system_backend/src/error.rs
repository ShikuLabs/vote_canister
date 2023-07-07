use thiserror;
use candid::CandidType;
#[derive(Debug, thiserror::Error, CandidType, Clone)]
pub enum VoteError {

    #[error("claim failed: `{0}`")]
    ClaimError(String),

    #[error("vote failed error: `{0}`")]
    VoteFailedError(String),

    #[error("other: `{0}`")]
    Other(String),
}