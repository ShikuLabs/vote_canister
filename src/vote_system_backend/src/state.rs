

// use vote_trait::*;
// use vote_types::*;
// use error::*;
use std::collections::{HashMap, HashSet};
use crate::{
    VoteId
    , WorkId,
    
};

use crate::VoteError::{
    ClaimError,
    VoteFailedError,
    Other,
};
use crate::vote_trait::VoteTrait;

use crate::{
    VoteLedger
};
const VOTE_NUMS: u32 = 3;

impl VoteTrait for VoteLedger {
    fn claim(&mut self, voter_id: &VoteId) -> Result<HashMap<VoteId, u32>, crate::VoteError> {
        if self.voter.contains_key(voter_id) {
            return Err(ClaimError(format!("voter {} claims failed", voter_id)));
        }
        self.voter.insert(*voter_id, VOTE_NUMS);
        Ok(self.voter.clone())
    }

    fn check_vote(&self, voter_id: &VoteId, work_id: &WorkId) -> Result<(), crate::VoteError> {
        if *self.voter.get(voter_id).unwrap() == 0u32 {
            return Err(VoteFailedError(format!("the voter {} has no vote amounts", voter_id)));
        }
        match self.vote_metadata.get(work_id) {
            Some(vote_set) => {
                if self.vote_metadata[work_id].contains(&voter_id) {
                    return Err(VoteFailedError(format!("the voter {} has voted this work, can not vote again", voter_id)));
                }
                Ok(())
            },
            None => return Err(Other("vote set is None".to_string()))
        }
        
    }

    fn get_vote_amount(&self, voter_id: &VoteId) -> u32 {
        match self.voter.get(voter_id) {
            Some(amount) => *amount,
            None => 0u32,
        }
    }

    fn get_work_vec(&self, work_id: &WorkId) -> HashSet<VoteId> {
        match self.vote_metadata.get(work_id) {
            Some(voter_id_set) => voter_id_set.clone(),
            None => HashSet::new(),
        }
    }

    fn vote(&mut self, work_id: &WorkId, voter_id: &VoteId) -> Result<HashSet<VoteId>, crate::VoteError> {
        let _ = self.check_vote(voter_id, work_id);
        let vote_amounut = self.get_vote_amount(voter_id);
        if vote_amounut == 0 {
            return Err(Other(format!("the voter {}'s amount is zero, please claim first.", &voter_id)));
        }
        self.voter.insert(*voter_id, vote_amounut);
        let mut hash_set = self.get_work_vec(work_id);
        hash_set.insert(*voter_id);
        let data = hash_set.clone();
        self.vote_metadata.insert(*work_id, hash_set);
        Ok(data)
    }

}

pub fn claim_votes(voter_id: &VoteId) -> Result<HashMap<VoteId, u32>, crate::VoteError> {
    crate::vote_ledeger_mut(|ledger| ledger.claim(voter_id))
}

pub fn vote(work_id: &WorkId, voter_id: &VoteId) -> Result<HashSet<VoteId>, crate::VoteError> {
    crate::vote_ledeger_mut(|ledger| ledger.vote(work_id, voter_id))
}