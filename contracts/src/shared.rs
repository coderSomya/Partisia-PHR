use chrono::Duration;
use std::collections::HashMap;

mod shared {
    use std::collections::HashMap;
    use chrono::{DateTime, Utc, Duration};

    // Constants
    pub const VERIFIERS_PATIENT_COUNT: u64 = 2;
    pub const VERIFIERS_PATIENT_OLDEST: Duration = Duration::weeks(52);

    pub const MAX_GUARDIANS_PER_PATIENT: u64 = 5;
    pub const VERIFIERS_GUARDIAN_CLAIM_COUNT: u64 = 1;
    pub const VERIFIERS_GUARDIAN_CLAIM_OLDEST: Duration = Duration::weeks(2 * 52);

    pub const MAX_WARDS_PER_PATIENT: u64 = 5;
    pub const VERIFIERS_WARDS_CLAIM_COUNT: u64 = 3;
    pub const VERIFIERS_WARDS_CLAIM_OLDEST: Duration = Duration::weeks(52);

    pub const VERIFIERS_DOCTOR_COUNT: u64 = 4;
    pub const VERIFIERS_DOCTOR_OLDEST: Duration = Duration::weeks(3 * 52);

    pub const VERIFIERS_HOSPITAL_COUNT: u64 = 4;
    pub const VERIFIERS_HOSPITAL_OLDEST: Duration = Duration::weeks(52);

    pub const MIN_NUM_RA_VERS: u64 = 2;
    pub const MAX_NUM_RA_VERS: u64 = 5;

    pub const MIN_NUM_GUARD_VERS: u64 = 1;
    pub const MAX_NUM_GUARD_VERS: u64 = 5;

    pub const MIN_RGTO_COUNT: u64 = 2;
    pub const MAX_RGTO_COUNT: u64 = 20;

    pub const MAX_LATENCY: Duration = Duration::hours(1);

    // Pure functions
    pub fn min(a: u64, b: u64) -> u64 {
        if a < b {
            a
        } else {
            b
        }
    }

    pub fn max(a: u64, b: u64) -> u64 {
        if a > b {
            a
        } else {
            b
        }
    }

    pub fn between(x: u64, a: u64, b: u64) -> u64 {
        if x < a {
            a
        } else if x > b {
            b
        } else {
            x
        }
    }

    // Claim struct and functions
    #[derive(Debug)]
    pub struct Claim {
        pub made: bool,
        pub any_verifier: bool,
        pub num_required_verifiers: u64,
        pub oldest_date: Duration,
        pub required_verifiers: Vec<String>,
        pub verifiers: Vec<String>,
        pub dates: Vec<DateTime<Utc>>,
    }

    impl Claim {
        pub fn verify_claim(&mut self, sender: &str) {
            assert!(self.made, "Claim must be made before being verified");

            let mut did_verify = false;
            let mut i = 0;
            for (index, verifier) in self.verifiers.iter().enumerate() {
                if verifier == sender {
                    did_verify = true;
                    i = index;
                    break;
                }
            }

            let mut is_valid_verifier = self.any_verifier;
            if !is_valid_verifier {
                let mut j = 0;
                for required_verifier in &self.required_verifiers {
                    if required_verifier == sender {
                        is_valid_verifier = true;
                        j = 0;
                        break;
                    }
                }
            }

            if is_valid_verifier {
                if !did_verify {
                    self.verifiers.push(sender.to_string());
                    self.dates.push(Utc::now());
                } else {
                    self.dates[i] = Utc::now();
                }
            }
        }

        pub fn revoke_claim(&mut self, sender: &str) {
            if let Some(index) = self.verifiers.iter().position(|x| x == sender) {
                if index < self.verifiers.len() - 1 {
                    self.verifiers.swap_remove(index);
                    self.dates.swap_remove(index);
                } else {
                    self.verifiers.pop();
                    self.dates.pop();
                }
            }
        }

        pub fn is_claim_verified(&self) -> bool {
            if !self.made || self.verifiers.len() < self.num_required_verifiers as usize {
                return false;
            }

            let mut verifications = 0;
            let mut required_verifications = 0;

            for (i, date) in self.dates.iter().enumerate() {
                if Utc::now().signed_duration_since(*date) <= self.oldest_date {
                    verifications += 1;

                    if !self.any_verifier {
                        let required_verifier_found = self.required_verifiers.contains(&self.verifiers[i]);
                        if required_verifier_found {
                            required_verifications += 1;
                        }
                    }
                }
            }

            if self.any_verifier {
                verifications >= self.num_required_verifiers as usize
            } else {
                (verifications >= self.num_required_verifiers as usize)
                    && (required_verifications >= self.required_verifiers.len())
            }
        }
    }

    // Regulatory agency member struct
    #[derive(Debug)]
    pub struct RAMember {
        pub registered: bool,
    }

    // Person struct
    #[derive(Debug)]
    pub struct Person {
        pub registered: bool,
        pub patient_verified: Claim,
        pub doctor_verified: Claim,
        pub bundle_hashes: Vec<String>,
        pub meddocs: HashMap<String, MedDoc>,
        pub guardians_address: Vec<String>,
        pub guardians_claim: Vec<Claim>,
        pub pdr_token_ids: Vec<String>,
        pub pdr_tokens: HashMap<String, PDReputationToken>,
        pub wards_address: Vec<String>,
        pub wards_claim: Vec<Claim>,
        pub dof_token_ids: Vec<String>,
        pub dof_tokens: HashMap<String, DOFileToken>,
    }

    #[derive(Debug)]
    pub struct MedDoc {
        pub exists: bool,
        pub request_count: u64,
        pub requests: HashMap<u64, Request>,
        pub default_num_ram_vers: u64,
        pub default_num_guard_vers: u64,
    }

    #[derive(Debug)]
    pub struct Request {
        pub exists: bool,
        pub doctor: String,
        pub request_time: DateTime<Utc>,
        pub min_rgto_count: u64,
        pub max_rgto_count: u64,
        pub patient_verifications: Claim,
        pub ra_member_verifications: Claim,
        pub guardian_verifications: Claim,
        pub rgtos_evaluated: bool,
        pub rgto_addresses: Vec<String>,
        pub rgto_ratings: HashMap<String, u16>,
    }

    #[derive(Debug)]
    pub struct PDReputationToken {
        pub exists: bool,
        pub doctor_address: String,
    }

    #[derive(Debug)]
    pub struct DOFileToken {
        pub exists: bool,
        pub rgto_address: String,
    }

 // Hospital struct
#[derive(Debug)]
pub struct Hospital {
    pub registered: bool,
    pub hospital_verified: Claim,
}

// Reputation-governed Trusted Oracle
#[derive(Debug)]
pub struct RGTO {
    pub registered: bool,
    pub average_contract_rating: u16,
    pub contract_rating_count: u16,
    pub average_doctor_rating: u16,
    pub doctor_rating_count: u16,
    pub odf_token_ids: Vec<String>, // rgto->doctor file token IDs
    pub odf_tokens: HashMap<String, ODFileToken>,
}

#[derive(Debug)]
pub struct ODFileToken {
    pub exists: bool,
    pub doctor_address: String,
    // TODO: maybe here we should have info about the file
}

}    
