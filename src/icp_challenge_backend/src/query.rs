use ic_cdk::query;
use crate::{STORAGE, Claim, Policy};

#[query]
fn get_claim(claim_id: String) -> Option<Claim> {
    STORAGE.with(|storage| storage.borrow().claims.get(&claim_id).cloned())
}

#[query]
fn get_policy(policy_id: String) -> Option<Policy> {
    STORAGE.with(|storage| storage.borrow().policies.get(&policy_id).cloned())
}
