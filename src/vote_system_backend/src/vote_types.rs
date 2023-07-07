use candid::{CandidType};
use std::collections::{HashMap, HashSet};
use ic_cdk::export::{Principal};
use std::cell::RefCell;
use ic_cdk::export::candid::{ Deserialize, Nat};
use serde::Serialize;
use crate::error::VoteError;


// claim request for voters
#[derive(Debug, Clone, CandidType, Serialize , Deserialize)]
pub struct ClaimRequest {
    // vote id: the icp address
    pub to: VoteId,
}

// vote request for voters
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct VoteRequest {
    // work id: the uploaded work for architecture
    pub work_id: WorkId,
    // voter id: the voter who wants to vote the specific arch work.
    pub voter_id: VoteId,
}

// vote response
#[derive(Debug, Clone, CandidType)]
pub enum VoteResponse {
    ok(HashSet<VoteId>),
    err(VoteError),
}

// claim response
#[derive(Debug, Clone, CandidType)]
pub enum ClaimResponse {
    ok(HashMap<VoteId, u32>),
    err(VoteError),
}

// vote id type definition: icp address
pub type VoteId = Principal;
// arch work id
pub type WorkId = u32;

// vote ledger: Record every transaction for each voter
#[derive(Debug, Clone, CandidType)]
pub struct VoteLedger {
    pub voter: HashMap<VoteId, u32>,
    pub vote_metadata: HashMap<WorkId, HashSet<VoteId>>,
}

// voter ledger initialization
impl VoteLedger {
    pub fn new() -> Self {
        Self {
            voter: HashMap::new(),
            vote_metadata: HashMap::new(),
        }
    }
}

// thread local set up
thread_local!(
    static VOTE_LEDGER: RefCell<VoteLedger> = RefCell::new(VoteLedger::new());
);

// public read function, denote as: vote_ledger
pub fn vote_ledger<T, F: FnOnce(&VoteLedger) -> T>(f: F) -> T {
    VOTE_LEDGER.with(|ledger| f(&ledger.borrow()))
}

// public write function: denote as: vote_ledger_mut
pub fn vote_ledeger_mut<T, F: FnOnce(&mut VoteLedger) -> T>(f: F) -> T {
    VOTE_LEDGER.with(|ledger| f(&mut ledger.borrow_mut()))
}
