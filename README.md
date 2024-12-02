# ICP Challenge Backend

This project is a backend service for managing insurance claims and policies on the Internet Computer. It provides functionalities to submit, review, and process insurance claims, as well as register new policies.

## Features

- **Submit Claims**: Users can submit claims against their insurance policies.
- **Review Claims**: Claims can be reviewed and either verified or rejected.
- **Process Claims**: Verified claims can be processed for payment.
- **Register Policies**: New insurance policies can be registered.

## Prerequisites

Before running the project, ensure you have the following installed:

- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **DFX SDK**: Install the DFX SDK from [internetcomputer.org](https://internetcomputer.org/docs/current/developer-docs/setup/install).

## Project Structure

- **src/icp_challenge_backend/src/lib.rs**: Contains the main data structures and logic for claims and policies.
- **src/icp_challenge_backend/src/update.rs**: Contains update functions for submitting, reviewing, and processing claims, as well as registering policies.
- **src/icp_challenge_backend/src/query.rs**: Contains query functions to retrieve claims and policies.

## Running the Project Locally

To test the project locally, follow these steps:

1. **Start the Internet Computer Replica**: This will run the replica in the background.
   ```bash
   dfx start --background
   ```

2. **Deploy the Canisters**: Deploy your canisters to the replica and generate the candid interface.
   ```bash
   dfx deploy
   ```

3. **Access the Application**: Once deployed, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

## Testing the Canister

To test the canister, you can use the following input data:

1. **Submit a Claim**:
   - **Policy Type**: A string representing the type of policy (e.g., "auto", "home").
   - **Claim Amount**: A numeric value representing the amount claimed (e.g., 1000).
   - **Description**: A string describing the claim (e.g., "Accident on highway").
   - **Supporting Documents**: A vector of strings representing document identifiers (e.g., ["doc1", "doc2"]).

2. **Review a Claim**:
   - **Claim ID**: A string representing the unique identifier of the claim.
   - **Verification Status**: A boolean indicating whether the claim is verified (true) or rejected (false).

3. **Process a Claim**:
   - **Claim ID**: A string representing the unique identifier of the claim to be processed.

4. **Register a Policy**:
   - **Policy Type**: A string representing the type of policy (e.g., "auto", "home").
   - **Coverage Amount**: A numeric value representing the coverage amount (e.g., 5000).

5. **Retrieve a Claim**:
   - **Claim ID**: A string representing the unique identifier of the claim to be retrieved.

6. **Retrieve a Policy**:
   - **Policy ID**: A string representing the unique identifier of the policy to be retrieved.

## Dependencies

- **candid**: For Candid type serialization and deserialization.
- **ic-cdk**: For interacting with the Internet Computer.
- **serde**: For serialization/deserialization.
- **serde_json**: For JSON handling.

