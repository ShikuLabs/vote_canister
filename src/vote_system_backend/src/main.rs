use ic_cdk_macros::{update, query};
use candid::{candid_method};
use std::collections::{HashMap, HashSet};

pub mod state;
pub mod vote_types;
pub mod error;
pub mod vote_trait;


use crate::state::{claim_votes, vote};
use crate::vote_types::*;
use crate::error::*;

#[update]
#[candid_method(update)]
pub fn shiku_vote(vote_req: VoteRequest) -> VoteResponse {
    match vote(&vote_req.work_id, &vote_req.voter_id) {
        Ok(vote_set) => {
            VoteResponse::ok(vote_set) 
        },
        Err(_) => VoteResponse::err(VoteError::VoteFailedError("Vote failed ".to_string()))
    }
}


#[update]
#[candid_method(update)]
pub fn shiku_claim_votes(claim_req: ClaimRequest) -> ClaimResponse {
    match claim_votes(&claim_req.to) {
        Ok(result) => {
            ClaimResponse::ok(result)
        },
        Err(_) => ClaimResponse::err(VoteError::ClaimError("Claim failed".to_string()))
    }
}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    // The line below generates did types and service definition from the
    // methods annotated with `candid_method` above. The definition is then
    // obtained with `__export_service()`.
    candid::export_service!();
    std::print!("{}", __export_service());
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}