pub mod query;
pub mod update;

use candid::{CandidType, Deserialize, Principal};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum ClaimStatus {
    Submitted,
    UnderReview,
    Verified,
    Rejected,
    Approved,
    Paid,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Claim {
    id: String,
    claimant: Principal,
    policy_type: String,
    claim_amount: u64,
    description: String,
    supporting_documents: Vec<String>,
    status: ClaimStatus,
    timestamp: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Policy {
    id: String,
    policy_holder: Principal,
    policy_type: String,
    coverage_amount: u64,
    active: bool,
}

#[derive(Default)]
struct InsuranceClaimProcessor {
    claims: HashMap<String, Claim>,
    policies: HashMap<String, Policy>,
    claim_counter: u64,
    policy_counter: u64,
}

thread_local! {
    static STORAGE: std::cell::RefCell<InsuranceClaimProcessor> = std::cell::RefCell::default();
}

impl InsuranceClaimProcessor {
    fn generate_claim_id(&mut self) -> String {
        self.claim_counter += 1;
        format!("claim_{}", self.claim_counter)
    }

    fn generate_policy_id(&mut self) -> String {
        self.policy_counter += 1;
        format!("policy_{}", self.policy_counter)
    }

    fn verify_claim_eligibility(claim: &Claim, policy: &Policy) -> bool {
        if !policy.active {
            return false;
        }
        if claim.claim_amount > policy.coverage_amount {
            return false;
        }
        true
    }
}

ic_cdk::export_candid!();