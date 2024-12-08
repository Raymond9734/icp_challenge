use crate::{Claim, ClaimStatus, InsuranceClaimProcessor, Policy, STORAGE};
use ic_cdk::update;

#[update]
fn submit_claim(
    policy_id: String,
    claim_amount: u64,
    description: String,
    supporting_documents: Vec<String>,
) -> Result<String, String> {
    if policy_id.is_empty() || description.is_empty() || supporting_documents.is_empty() {
        return Err("All fields are required".to_string());
    }

    STORAGE.with(|storage| {
        let mut storage_mut = storage.borrow_mut();

        // Retrieve the policy
        let policy = storage_mut
            .policies
            .get(&policy_id)
            .ok_or_else(|| "Policy not found".to_string())?
            .clone(); // Clone to drop the immutable borrow

        // Check if the caller is authorized
        if policy.policy_holder != ic_cdk::caller() {
            return Err("Unauthorized claim submission".to_string());
        }

        // Validate the claim
        if !InsuranceClaimProcessor::verify_claim_eligibility(
            &Claim {
                id: String::new(),
                claimant: ic_cdk::caller(),
                policy_type: policy.policy_type.clone(),
                claim_amount,
                description: description.clone(),
                supporting_documents: supporting_documents.clone(),
                status: ClaimStatus::Submitted,
                timestamp: ic_cdk::api::time(),
            },
            &policy,
        ) {
            return Err("Claim is not eligible".to_string());
        }

        // Create the new claim
        let new_claim = Claim {
            id: storage_mut.generate_claim_id(),
            claimant: ic_cdk::caller(),
            policy_type: policy.policy_type,
            claim_amount,
            description,
            supporting_documents,
            status: ClaimStatus::Submitted,
            timestamp: ic_cdk::api::time(),
        };

        // Store the claim
        let claim_id = new_claim.id.clone();
        storage_mut.claims.insert(claim_id.clone(), new_claim);

        Ok(claim_id)
    })
}

#[update]
fn review_claim(claim_id: String, decision: bool) -> Result<ClaimStatus, String> {
    if claim_id.is_empty() {
        return Err("Claim ID is required".to_string());
    }

    STORAGE.with(|storage| {
        let mut storage_mut = storage.borrow_mut();

        let claim = storage_mut
            .claims
            .get_mut(&claim_id)
            .ok_or_else(|| "Claim not found".to_string())?;

        claim.status = if decision {
            ClaimStatus::Verified
        } else {
            ClaimStatus::Rejected
        };

        Ok(claim.status.clone())
    })
}

#[update]
fn process_claim(claim_id: String) -> Result<(), String> {
    if claim_id.is_empty() {
        return Err("Claim ID is required".to_string());
    }

    STORAGE.with(|storage| {
        let mut storage_mut = storage.borrow_mut();

        let claim = storage_mut
            .claims
            .get_mut(&claim_id)
            .ok_or_else(|| "Claim not found".to_string())?;

        if claim.status != ClaimStatus::Verified {
            return Err("Claim cannot be processed".to_string());
        }

        claim.status = ClaimStatus::Paid;
        Ok(())
    })
}

#[update]
fn register_policy(policy_type: String, coverage_amount: u64) -> Result<String, String> {
    if policy_type.is_empty() || coverage_amount == 0 {
        return Err("Policy type and coverage amount are required".to_string());
    }

    STORAGE.with(|storage| {
        let mut storage_mut = storage.borrow_mut();

        let policy_id = storage_mut.generate_policy_id();

        let new_policy = Policy {
            id: policy_id.clone(),
            policy_holder: ic_cdk::caller(),
            policy_type,
            coverage_amount,
            active: true,
        };

        storage_mut.policies.insert(policy_id.clone(), new_policy);

        Ok(policy_id)
    })
}

#[update]
fn deactivate_policy(policy_id: String) -> Result<(), String> {
    if policy_id.is_empty() {
        return Err("Policy ID is required".to_string());
    }

    STORAGE.with(|storage| {
        let mut storage_mut = storage.borrow_mut();

        let policy = storage_mut
            .policies
            .get_mut(&policy_id)
            .ok_or_else(|| "Policy not found".to_string())?;

        policy.active = false;
        Ok(())
    })
}

#[update]
fn update_policy_coverage(policy_id: String, new_coverage: u64) -> Result<(), String> {
    if policy_id.is_empty() || new_coverage == 0 {
        return Err("Policy ID and new coverage amount are required".to_string());
    }

    STORAGE.with(|storage| {
        let mut storage_mut = storage.borrow_mut();

        let policy = storage_mut
            .policies
            .get_mut(&policy_id)
            .ok_or_else(|| "Policy not found".to_string())?;

        policy.coverage_amount = new_coverage;
        Ok(())
    })
}
