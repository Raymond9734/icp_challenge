use ic_cdk::query;
use crate::{STORAGE, Claim, Policy, ClaimHistory};

#[query]
fn get_claim(claim_id: String) -> Option<Claim> {
    STORAGE.with(|storage| storage.borrow().claims.get(&claim_id).cloned())
}

#[query]
fn get_policy(policy_id: String) -> Option<Policy> {
    STORAGE.with(|storage| storage.borrow().policies.get(&policy_id).cloned())
}

#[query]
fn get_claim_history(claim_id: String) -> Result<Vec<ClaimHistory>, String> {
    STORAGE.with(|storage| {
        storage
            .borrow()
            .claims
            .get(&claim_id)
            .map(|claim| claim.history.clone())
            .ok_or_else(|| "Claim not found".to_string())
    })
}
