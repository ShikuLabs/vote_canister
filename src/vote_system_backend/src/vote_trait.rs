
use crate::VoteError;
use crate::{
    VoteId,
    WorkId,
};
use std::collections::{HashMap, HashSet};

pub trait VoteTrait {
    fn claim(&mut self, voter_id: &VoteId) -> Result<HashMap<VoteId, u32>, VoteError>;
    fn vote(&mut self, work_id: &WorkId, voter_id: &VoteId) -> Result<HashSet<VoteId>, VoteError>;
    fn check_vote(&self, voter_id: &VoteId, work_id: &WorkId) -> Result<(), VoteError>;
    fn get_vote_amount(&self, voter_id: &VoteId) -> u32;
    fn get_work_vec(&self, work_id: &WorkId) -> HashSet<VoteId>;
}

