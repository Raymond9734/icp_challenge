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

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Policy {
    pub id: String,
    pub policy_holder: Principal,
    pub policy_type: String,
    pub coverage_amount: u64,
    pub active: bool,
    pub created_at: u64,
    pub last_modified: u64,
}

impl Policy {
    pub fn new(
        id: String,
        policy_holder: Principal,
        policy_type: String,
        coverage_amount: u64,
    ) -> Self {
        let now = ic_cdk::api::time();
        Self {
            id,
            policy_holder,
            policy_type,
            coverage_amount,
            active: true,
            created_at: now,
            last_modified: now,
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ClaimHistory {
    pub status: ClaimStatus,
    pub timestamp: u64,
    pub notes: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Claim {
    id: String,
    claimant: Principal,
    policy_type: String,
    claim_amount: u64,
    description: String,
    supporting_documents: Vec<String>,
    status: ClaimStatus,
    timestamp: u64,
    pub history: Vec<ClaimHistory>,
}

#[derive(Default)]
struct InsuranceClaimProcessor {
    claims: HashMap<String, Claim>,
    policies: HashMap<String, Policy>,
}

thread_local! {
    static STORAGE: std::cell::RefCell<InsuranceClaimProcessor> = std::cell::RefCell::default();
}

impl InsuranceClaimProcessor {
    fn generate_claim_id() -> String {
        // Generate a unique claim ID 
        format!("claim_{}", ic_cdk::api::time())
    }

    fn verify_claim_eligibility(claim: &Claim, policy: &Policy) -> bool {
        // Check policy is active
        if !policy.active {
            return false;
        }

        // Check claim amount within coverage
        if claim.claim_amount > policy.coverage_amount {
            return false;
        }

        true
    }
}

ic_cdk::export_candid!();
