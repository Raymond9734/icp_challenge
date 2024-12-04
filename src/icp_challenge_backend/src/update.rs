use crate::{Claim, ClaimStatus, InsuranceClaimProcessor, Policy, STORAGE, ClaimHistory};
use ic_cdk::update;

#[update]
fn submit_claim(
    policy_id: String,
    claim_amount: u64,
    description: String,
    supporting_documents: Vec<String>,
) -> Result<String, String> {
    STORAGE.with(|storage| {
        let mut storage_mut = storage.borrow_mut();

        // Retrieve policy
        let policy = storage_mut
            .policies
            .get(&policy_id)
            .ok_or_else(|| "Policy not found".to_string())?;

        // Check policy belongs to claimant
        if policy.policy_holder != ic_cdk::caller() {
            return Err("Unauthorized claim submission".to_string());
        }

        // Create new claim
        let new_claim = Claim {
            id: InsuranceClaimProcessor::generate_claim_id(),
            claimant: ic_cdk::caller(),
            policy_type: policy.policy_type.clone(),
            claim_amount,
            description,
            supporting_documents,
            status: ClaimStatus::Submitted,
            timestamp: ic_cdk::api::time(),
            history: vec![ClaimHistory {
                status: ClaimStatus::Submitted,
                timestamp: ic_cdk::api::time(),
                notes: "Claim submitted".to_string(),
            }],
        };

        // Validate claim
        if !InsuranceClaimProcessor::verify_claim_eligibility(&new_claim, policy) {
            return Err("Claim does not meet eligibility criteria".to_string());
        }

        // Store claim
        let claim_id = new_claim.id.clone();
        storage_mut.claims.insert(claim_id.clone(), new_claim);

        Ok(claim_id)
    })
}

#[update]
fn review_claim(claim_id: String, decision: bool, notes: String) -> Result<ClaimStatus, String> {
    STORAGE.with(|storage| {
        let mut storage_mut = storage.borrow_mut();

        let mut claim = storage_mut
            .claims
            .get(&claim_id)
            .ok_or_else(|| "Claim not found".to_string())?
            .clone();

        let new_status = if decision {
            ClaimStatus::Verified
        } else {
            ClaimStatus::Rejected
        };

        claim.status = new_status.clone();
        claim.history.push(ClaimHistory {
            status: new_status,
            timestamp: ic_cdk::api::time(),
            notes,
        });

        storage_mut.claims.insert(claim_id, claim.clone());
        Ok(claim.status)
    })
}

#[update]
fn process_claim(claim_id: String) -> Result<(), String> {
    STORAGE.with(|storage| {
        let mut storage_mut = storage.borrow_mut();

        // Retrieve claim
        let mut claim = storage_mut
            .claims
            .get(&claim_id)
            .ok_or_else(|| "Claim not found".to_string())?
            .clone();

        // Check claim is verified
        if claim.status != ClaimStatus::Verified {
            return Err("Claim cannot be processed".to_string());
        }

        // Update claim status to paid
        claim.status = ClaimStatus::Paid;

        // Update the claim in storage
        storage_mut.claims.insert(claim_id, claim);

        Ok(())
    })
}

#[update]
fn register_policy(policy_type: String, coverage_amount: u64) -> Result<String, String> {
    STORAGE.with(|storage| {
        let mut storage_mut = storage.borrow_mut();

        // Generate unique policy ID
        let policy_id = format!("policy_{}", ic_cdk::api::time());

        let new_policy = Policy {
            id: policy_id.clone(),
            policy_holder: ic_cdk::caller(),
            policy_type,
            coverage_amount,
            active: true,
            created_at: ic_cdk::api::time(),
            last_modified: ic_cdk::api::time(),
        };

        storage_mut.policies.insert(policy_id.clone(), new_policy);

        Ok(policy_id)
    })
}

#[update]
fn cancel_policy(policy_id: String) -> Result<(), String> {
    STORAGE.with(|storage| {
        let mut storage_mut = storage.borrow_mut();
        
        let mut policy = storage_mut
            .policies
            .get(&policy_id)
            .ok_or_else(|| "Policy not found".to_string())?
            .clone();

        // Verify caller is policy holder
        if policy.policy_holder != ic_cdk::caller() {
            return Err("Unauthorized: Only policy holder can cancel policy".to_string());
        }

        policy.active = false;
        policy.last_modified = ic_cdk::api::time();
        
        storage_mut.policies.insert(policy_id, policy);
        Ok(())
    })
}

#[update]
fn update_policy_coverage(policy_id: String, new_coverage_amount: u64) -> Result<(), String> {
    STORAGE.with(|storage| {
        let mut storage_mut = storage.borrow_mut();
        
        let mut policy = storage_mut
            .policies
            .get(&policy_id)
            .ok_or_else(|| "Policy not found".to_string())?
            .clone();

        // Verify caller is policy holder
        if policy.policy_holder != ic_cdk::caller() {
            return Err("Unauthorized: Only policy holder can update policy".to_string());
        }

        if !policy.active {
            return Err("Cannot update inactive policy".to_string());
        }

        policy.coverage_amount = new_coverage_amount;
        policy.last_modified = ic_cdk::api::time();
        
        storage_mut.policies.insert(policy_id, policy);
        Ok(())
    })
}
