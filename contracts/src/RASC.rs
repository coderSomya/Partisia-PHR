// SPDX-License-Identifier: MIT

use std::collections::HashMap;

mod shared;
use shared::{RAMember, Person, Hospital, RGTO};

pub struct RASC {
    // State variables
    owner: Option<address>,
    ramembers: HashMap<address, RAMember>,
    persons: HashMap<address, Person>,
    hospitals: HashMap<address, Hospital>,
    rgtos: HashMap<address, RGTO>, // Not Important
}

impl RASC {
    // Modifier
    fn only_owner(&self, sender: address) -> Result<(), &'static str> {
        if let Some(owner_address) = self.owner {
            if sender == owner_address {
                return Ok(());
            }
        }
        Err("Only regulatory agency smart contract owner can call this function")
    }

    fn not_owner(&self, sender: address) -> Result<(), &'static str> {
        if let Some(owner_address) = self.owner {
            if sender != owner_address {
                return Ok(());
            }
        }
        Err("Regulatory agency smart contract owner cannot call this function")
    }

    fn not_registered(&self, sender: address) -> Result<(), &'static str> {
        if !self.ramembers.contains_key(&sender)
            && !self.persons.contains_key(&sender)
            && !self.hospitals.contains_key(&sender)
            && !self.rgtos.contains_key(&sender)
        {
            return Ok(());
        }
        Err("Only unregistered entities can call this function")
    }

    fn only_ra_member(&self, sender: address) -> Result<(), &'static str> {
        if let Some(_ra_member) = self.ramembers.get(&sender) {
            return Ok(());
        }
        Err("Only regulatory agency members can call this function")
    }

    fn only_person(&self, sender: address) -> Result<(), &'static str> {
        if let Some(_person) = self.persons.get(&sender) {
            return Ok(());
        }
        Err("Only a person can call this function")
    }

    fn only_hospital(&self, sender: address) -> Result<(), &'static str> {
        if let Some(_hospital) = self.hospitals.get(&sender) {
            return Ok(());
        }
        Err("Only hospitals can call this function")
    }

    fn only_rgto(&self, sender: address) -> Result<(), &'static str> {
        if let Some(_rgto) = self.rgtos.get(&sender) {
            return Ok(());
        }
        Err("Only RGTOs can call this function")
    }

    // Constructor
    pub fn new() -> Self {
        Self {
            owner: None,
            ramembers: HashMap::new(),
            persons: HashMap::new(),
            hospitals: HashMap::new(),
            rgtos: HashMap::new(),
        }
    }

    // Register
    pub fn register_ra_member(&mut self, ramember_address: address) -> Result<(), &'static str> {
        self.only_owner(msg.sender)?;

        self.ramembers.insert(ramember_address, RAMember { registered: true });

        // Emit event
        println!("newRAMemberRegistered: {:?}", ramember_address); // Placeholder for emitting event in Rust

        Ok(())
    }

    pub fn register_patient(&mut self, sender: address) -> Result<(), &'static str> {
        self.not_registered(sender)?;
        self.not_owner(sender)?;

        let person = self.persons.entry(sender).or_insert(Person {
            registered: false,
            patient_verified: Verification {
                made: false,
                any_verifier: false,
                num_required_verifiers: VERIFIERS_PATIENT_COUNT,
                oldest_date: VERIFIERS_PATIENT_OLDEST,
            },
            doctor_verified: Verification::default(),
        });

        if person.patient_verified.made {
            return Err("This person already made a claim to be a patient");
        }

        person.registered = true;
        person.patient_verified.made = true;
        person.patient_verified.any_verifier = true;
        person.patient_verified.num_required_verifiers = Shared::VERIFIERS_PATIENT_COUNT;
        person.patient_verified.oldest_date = Shared::VERIFIERS_PATIENT_OLDEST;

        // Emit event
        println!("newPatientRegistered: {:?}", sender); // Placeholder for emitting event in Rust

        Ok(())
    }

    pub fn register_doctor(&mut self, sender: address) -> Result<(), &'static str> {
        self.only_person(sender)?;
        let person = self.persons.get_mut(&sender).ok_or("Person not found")?;

        if person.doctor_verified.made {
            return Err("This person already made a claim to be a doctor");
        }

        if !Shared::is_claim_verified(&person.patient_verified) {
            return Err("To register as a doctor, person must be a verified patient");
        }

        person.doctor_verified.made = true;
        person.doctor_verified.any_verifier = true;
        person.doctor_verified.num_required_verifiers = Shared::VERIFIERS_DOCTOR_COUNT;
        person.doctor_verified.oldest_date = Shared::VERIFIERS_DOCTOR_OLDEST;

        // Emit event
        println!("newDoctorRegistered: {:?}", sender); // Placeholder for emitting event in Rust

        Ok(())
    }

    pub fn register_hospital(&mut self, sender: address) -> Result<(), &'static str> {
        self.not_registered(sender)?;
        self.not_owner(sender)?;

        let hospital = self.hospitals.entry(sender).or_insert(Hospital {
            registered: false,
            hospital_verified: Verification {
                made: true,
                any_verifier: true,
                num_required_verifiers: Shared::VERIFIERS_HOSPITAL_COUNT,
                oldest_date: Shared::VERIFIERS_HOSPITAL_OLDEST,
            },
        });

        hospital.registered = true;

        // Emit event
        println!("newHospitalRegistered: {:?}", sender); // Placeholder for emitting event in Rust

        Ok(())
    }

    pub fn register_rgto(&mut self, sender: address) -> Result<(), &'static str> {
        self.not_registered(sender)?;
        self.not_owner(sender)?;

        let rgto = self.rgtos.entry(sender).or_insert(RGTO {
            registered: false,
            average_contract_rating: 0,
            contract_rating_count: 0,
            average_doctor_rating: 0,
            doctor_rating_count: 0,
        });

        rgto.registered = true;

        // Emit event
        println!("newRGTORegistered: {:?}", sender); // Placeholder for emitting event in Rust

        Ok(())
    }

    pub fn verify_patient(&mut self, person_address: address, sender: address) -> Result<(), &'static str> {
        self.only_ra_member(sender)?;
        let person = self.persons.get_mut(&person_address).ok_or("Person not found")?;
        Shared::verify_claim(&mut person.patient_verified)?;

        // Emit event
        println!("newPatientVerification: {:?}, {:?}", person_address, sender); // Placeholder for emitting event in Rust

        Ok(())
    }

    pub fn verify_doctor(&mut self, person_address: address, sender: address) -> Result<(), &'static str> {
        self.only_ra_member(sender)?;
        let person = self.persons.get_mut(&person_address).ok_or("Person not found")?;
        Shared::verify_claim(&mut person.doctor_verified)?;

        // Emit event
        println!("newDoctorVerification: {:?}, {:?}", person_address, sender); // Placeholder for emitting event in Rust

        Ok(())
    }

    pub fn add_guardian(&mut self, guardian_address: address, sender: address) -> Result<(), &'static str> {
        self.only_person(sender)?;
        let person = self.persons.get_mut(&sender).ok_or("Person not found")?;

        if !Shared::is_claim_verified(&person.patient_verified) {
            return Err("Patient must be verified before adding a guardian");
        }
        if person.guardians_address.len() >= Shared::MAX_GUARDIANS_PER_PATIENT {
            return Err("Maximum number of guardians reached");
        }

        if person.guardians_address.contains(&guardian_address) {
            return Err("Guardian is already added");
        }

        person.guardians_address.push(guardian_address);

        let mut guardian_claim = Claim {
            made: true,
            any_verifier: false,
            num_required_verifiers: Shared::VERIFIERS_GUARDIAN_CLAIM_COUNT,
            oldest_date: Shared::VERIFIERS_GUARDIAN_CLAIM_OLDEST,
            required_verifiers: vec![guardian_address],
        };

        person.guardians_claim.push(guardian_claim);

        // Emit event
        println!("guardianshipAdded: {:?}, {:?}", guardian_address, sender); // Placeholder for emitting event in Rust

        Ok(())
    }

    pub fn remove_guardian(&mut self, guardian_address: address, sender: address) -> Result<(), &'static str> {
        self.only_person(sender)?;
        let person = self.persons.get_mut(&sender).ok_or("Person not found")?;

        if person.guardians_address.is_empty() {
            return Err("Patient must have a guardian added already");
        }

        if let Some(index) = person.guardians_address.iter().position(|&g| g == guardian_address) {
            if index < person.guardians_address.len() {
                person.guardians_address[index] = person.guardians_address[person.guardians_address.len() - 1];
                person.guardians_claim[index] = person.guardians_claim[person.guardians_claim.len() - 1];
            }
            person.guardians_address.pop();
            person.guardians_claim.pop();

            // Emit event
            println!("guardianshipRemoved: {:?}, {:?}", guardian_address, sender); // Placeholder for emitting event in Rust

            return Ok(());
        }

        Err("Guardian not found for removal")
    }

    pub fn verify_guardian(&mut self, person_address: address, sender: address) -> Result<(), &'static str> {
        self.only_person(sender)?;
        let person = self.persons.get_mut(&person_address).ok_or("Person not found")?;

        if person.guardians_address.is_empty() {
            return Err("Patient must have a guardian added already");
        }

        for (i, &guardian_address) in person.guardians_address.iter().enumerate() {
            if guardian_address == sender {
                Shared::verify_claim(&mut person.guardians_claim[i])?;

                // Emit event
                println!("guardianshipVerified: {:?}, {:?}", sender, person_address); // Placeholder for emitting event in Rust

                return Ok(());
            }
        }

        Err("Guardian not found for verification")
    }

    pub fn add_ward(&mut self, ward_address: address, sender: address) -> Result<(), &'static str> {
        self.only_person(sender)?;
        let person = self.persons.get_mut(&sender).ok_or("Person not found")?;

        if !Shared::is_claim_verified(&person.patient_verified) {
            return Err("Patient must be verified before adding a guardian");
        }
        if person.wards_address.len() >= Shared::MAX_WARDS_PER_PATIENT {
            return Err("Maximum number of wards reached");
        }

        if person.wards_address.contains(&ward_address) {
            return Err("Ward is already added");
        }

        person.wards_address.push(ward_address);

        let ward_claim = Claim {
            made: true,
            any_verifier: true,
            num_required_verifiers: Shared::VERIFIERS_WARDS_CLAIM_COUNT,
            oldest_date: Shared::VERIFIERS_WARDS_CLAIM_OLDEST,
        };

        person.wards_claim.push(ward_claim);

        // Emit event
        println!("wardAdded: {:?}, {:?}", ward_address, sender); // Placeholder for emitting event in Rust

        Ok(())
    }

    pub fn remove_ward(&mut self, ward_address: address, sender: address) -> Result<(), &'static str> {
        self.only_person(sender)?;
        let person = self.persons.get_mut(&sender).ok_or("Person not found")?;

        if person.wards_address.is_empty() {
            return Err("Patient must have a ward added already");
        }

        if let Some(index) = person.wards_address.iter().position(|&w| w == ward_address) {
            if index < person.wards_address.len() {
                person.wards_address[index] = person.wards_address[person.wards_address.len() - 1];
                person.wards_claim[index] = person.wards_claim[person.wards_claim.len() - 1];
            }
            person.wards_address.pop();
            person.wards_claim.pop();

            // Emit event
            println!("wardRemoved: {:?}, {:?}", ward_address, sender); // Placeholder for emitting event in Rust

            return Ok(());
        }

        Err("Ward not found for removal")
    }

    pub fn verify_ward(&mut self, person_address: address, ward_address: address, sender: address) -> Result<(), &'static str> {
        self.only_ra_member(sender)?;
        let person = self.persons.get_mut(&person_address).ok_or("Person not found")?;

        if person.guardians_address.is_empty() {
            return Err("Patient must have a ward added already");
        }

        for (i, &ward) in person.wards_address.iter().enumerate() {
            if ward == ward_address {
                Shared::verify_claim(&mut person.wards_claim[i])?;

                // Emit event
                println!("wardVerified: {:?}, {:?}", ward_address, sender); // Placeholder for emitting event in Rust

                return Ok(());
            }
        }

        Err("Ward not found for verification")
    }

    pub fn add_md_patient(&mut self, bundle_hash: bytes32, default_num_ram_vers: uint, default_num_guard_vers: uint, sender: address) -> Result<(), &'static str> {
        self.only_person(sender)?;
        let person = self.persons.get_mut(&sender).ok_or("Person not found")?;

        if !Shared::is_claim_verified(&person.patient_verified) {
            return Err("Patient must be verified before adding a medical document");
        }

        person.bundle_hashes.push(bundle_hash);

        let md = MedDoc {
            exists: true,
            default_num_ram_vers: Shared::between(default_num_ram_vers, Shared::MIN_NUM_RA_VERS, Shared::MAX_NUM_RA_VERS),
            default_num_guard_vers: Shared::between(default_num_guard_vers, Shared::MIN_NUM_GUARD_VERS, Shared::MAX_NUM_GUARD_VERS),
        };

        person.meddocs.insert(bundle_hash, md);

        // Emit event
        println!("patientMDAdded: {:?}, {:?}, {:?}, {:?}", sender, bundle_hash, md.default_num_ram_vers, md.default_num_guard_vers); // Placeholder for emitting event in Rust

        Ok(())
    }

    // Requesting medical documents functions
    pub fn request_md_doctor(&mut self, patient_address: address, bundle_hash: bytes32, min_rgto_count: uint, max_rgto_count: uint, direct: bool, sender: address) -> Result<(), &'static str> {
        let patient = self.persons.get(&patient_address).ok_or("Patient not found")?;

        if !patient.meddocs.contains_key(&bundle_hash) {
            return Err("Requested medical document does not exist");
        }
        if min_rgto_count > max_rgto_count {
            return Err("Request requires minimum count of RGTOs to be less than maximum count of RGTOs");
        }

        let request = Request {
            exists: true,
            doctor: sender,
            request_time: now(), // You need to implement the 'now' function in Rust to get the current timestamp
            min_rgto_count: Shared::max(min_rgto_count, Shared::MIN_RGTO_COUNT),
            max_rgto_count: Shared::min(max_rgto_count, Shared::MAX_RGTO_COUNT),
            patient_verifications: if direct {
                Claim {
                    made: true,
                    any_verifier: false,
                    num_required_verifiers: 1,
                    oldest_date: 7 * 24 * 60 * 60, // 7 days in seconds
                    required_verifiers: vec![patient_address],
                }
            } else {
                Claim {
                    made: true,
                    any_verifier: true,
                    num_required_verifiers: patient.meddocs[&bundle_hash].default_num_ram_vers,
                    oldest_date: 3 * 24 * 60 * 60, // 3 days in seconds
                    required_verifiers: Vec::new(),
                }
            },
            ramember_verifications: Claim::default(),
            guardian_verifications: if direct {
                Claim::default()
            } else {
                Claim {
                    made: true,
                    any_verifier: true,
                    num_required_verifiers: patient.meddocs[&bundle_hash].default_num_guard_vers,
                    oldest_date: 3 * 24 * 60 * 60, // 3 days in seconds
                    required_verifiers: Vec::new(),
                }
            },
            rgtos_evaluated: false,
        };

        let meddoc = self.persons.get_mut(&patient_address).unwrap().meddocs.get_mut(&bundle_hash).unwrap();
        meddoc.requests.insert(meddoc.request_count, request);
        meddoc.request_count += 1;

        // Emit event
        println!("newMDRequest: {:?}, {:?}, {:?}, {:?}", patient_address, sender, bundle_hash, meddoc.request_count - 1); // Placeholder for emitting event in Rust

        Ok(())
    }

    pub fn verify_request_patient(&mut self, bundle_hash: bytes32, request_id: uint, sender: address) -> Result<(), &'static str> {
        self.only_person(sender)?;
        let person = self.persons.get_mut(&sender).ok_or("Person not found")?;
        let request = person.meddocs.get_mut(&bundle_hash).and_then(|meddoc| meddoc.requests.get_mut(&request_id)).ok_or("Request not found")?;

        if !request.patient_verifications.made {
            return Err("Patient verification was not asked for");
        }

        Shared::verify_claim(&mut request.patient_verifications)?;

        // Emit event
        let doctor_address = request.doctor;
        println!("patientVerifiedMDRequest: {:?}, {:?}, {:?}, {:?}", sender, doctor_address, bundle_hash, request_id); // Placeholder for emitting event in Rust

        Ok(())
    }

    pub fn verify_request_ra_member(&mut self, patient_address: address, bundle_hash: bytes32, request_id: uint, sender: address) -> Result<(), &'static str> {
        self.only_ra_member(sender)?;
        let patient = self.persons.get(&patient_address).ok_or("Patient not found")?;
        let request = patient.meddocs.get(&bundle_hash).and_then(|meddoc| meddoc.requests.get(&request_id)).ok_or("Request not found")?;

        if !request.ramember_verifications.made {
            return Err("Regulatory agency member verification was not asked for");
        }

        Shared::verify_claim(&mut request.ramember_verifications)?;

        // Emit event
        let doctor_address = request.doctor;
        println!("RAMVerifiedMDRequest: {:?}, {:?}, {:?}, {:?}", patient_address, doctor_address, bundle_hash, request_id); // Placeholder for emitting event in Rust

        Ok(())
    }

    pub fn verify_request_guardian(&mut self, patient_address: address, bundle_hash: bytes32, request_id: uint, sender: address) -> Result<(), &'static str> {
        self.only_person(sender)?;
        let patient = self.persons.get(&patient_address).ok_or("Patient not found")?;
        let request = patient.meddocs.get(&bundle_hash).and_then(|meddoc| meddoc.requests.get(&request_id)).ok_or("Request not found")?;

        if !request.guardian_verifications.made {
            return Err("Guardian verification was not asked for");
        }

        let mut is_valid_guardian = false;
        for i in 0..patient.guardians_address.len() {
            if patient.guardians_address[i] == sender {
                if Shared::is_claim_verified(&patient.guardians_claim[i]) {
                    is_valid_guardian = true;
                }
                break;
            }
        }

        if is_valid_guardian {
            Shared::verify_claim(&mut request.guardian_verifications)?;

            // Emit event
            let doctor_address = request.doctor;
            println!("guardianVerifiedMDRequest: {:?}, {:?}, {:?}, {:?}", patient_address, doctor_address, bundle_hash, request_id); // Placeholder for emitting event in Rust
        }

        Ok(())
    }

    pub fn check_request_status(&mut self, patient_address: address, bundle_hash: bytes32, request_id: uint) -> bool {
        let patient = self.persons.get(&patient_address).unwrap();
        let request = patient.meddocs.get(&bundle_hash).and_then(|meddoc| meddoc.requests.get(&request_id)).unwrap();

        if Shared::is_claim_verified(&request.patient_verifications) ||
            (Shared::is_claim_verified(&request.ramember_verifications) && Shared::is_claim_verified(&request.guardian_verifications))
        {
            if !self.rgtos.get(&msg.sender).map_or(false, |rgto| rgto.registered) {
                // Emit event
                println!("callForRGTOs"); // Placeholder for emitting event in Rust
            }
            return true;
        }

        false
    }

    pub fn add_rgto_response(&mut self, patient_address: address, bundle_hash: bytes32, request_id: uint, proof: bytes32, sender: address) {
        self.only_rgto(sender).expect("Only RGTOs can call this function");
        let request = self.persons[&patient_address].meddocs[&bundle_hash].requests[&request_id];

        // require(checkRequestStatus(patientAddress, bundleHash, requestId), "Granted request is required to call this function");
        assert!(!request.rgtos_evaluated, "Unevaluated request is required to call this function");

        let latency = (now() - request.request_time) as u16;

        // Conditions to accept new response
        if request.rgto_addresses.len() < request.min_rgto_count ||
            (request.rgto_addresses.len() >= request.min_rgto_count &&
                request.rgto_addresses.len() < request.max_rgto_count &&
                latency <= Shared::MAX_LATENCY as u16)
        {
            let is_hash_correct = if bundle_hash == proof { 1 } else { 0 };

            let input_start: u16 = 1;
            let input_end: u16 = 1 * 60 * 60; // 1 hour in seconds
            let output_start: u16 = 2_u16.pow(16) - 1;
            let output_end: u16 = 1;

            // TODO: make sure this is working correctly
            let mut rgto_rating = is_hash_correct;
            if latency < 1 {
                rgto_rating *= 2_u16.pow(16) - 1;
            } else if latency > 1 * 60 * 60 {
                rgto_rating *= 0;
            } else {
                rgto_rating *= output_start + ((output_end - output_start) / (input_end - input_start)) * (latency - input_start);
            }

            self.persons.get_mut(&patient_address).unwrap().meddocs.get_mut(&bundle_hash).unwrap().requests.get_mut(&request_id).unwrap().rgto_addresses.push(sender);
            self.persons.get_mut(&patient_address).unwrap().meddocs.get_mut(&bundle_hash).unwrap().requests.get_mut(&request_id).unwrap().rgto_ratings.insert(sender, rgto_rating);
        }

        // Conditions to evaluate request
        if (request.rgto_addresses.len() >= request.min_rgto_count &&
            request.request_time + Shared::MAX_LATENCY <= now()) ||
            request.rgto_addresses.len() == request.max_rgto_count
        {
            self.evaluate_rgto(patient_address, bundle_hash, request_id);
            self.persons.get_mut(&patient_address).unwrap().meddocs.get_mut(&bundle_hash).unwrap().requests.get_mut(&request_id).unwrap().rgtos_evaluated = true;
        }
    }

    fn evaluate_rgto(&mut self, patient_address: address, bundle_hash: bytes32, request_id: uint) {
        let request = &mut self.persons.get_mut(&patient_address).unwrap().meddocs.get_mut(&bundle_hash).unwrap().requests.get_mut(&request_id).unwrap();

        let mut best_rgto_address: Option<address> = None;
        let mut best_rgto_score: u64 = 0;

        for rgto_address in &request.rgto_addresses {
            let rgto_rating = request.rgto_ratings[&rgto_address];
            let rgto_reputation = (self.rgtos[&rgto_address].average_contract_rating + self.rgtos[&rgto_address].average_doctor_rating) / 2;

            let rgto_score = rgto_rating as u64 * (rgto_reputation as u64 + 1).pow(2);

            if rgto_score >= best_rgto_score {
                best_rgto_score = rgto_score;
                best_rgto_address = Some(*rgto_address);
            }
        }

        if let Some(best_rgto_address) = best_rgto_address {
            for rgto_address in &request.rgto_addresses {
                let rgto = &mut self.rgtos.get_mut(&rgto_address).unwrap();
                rgto.average_contract_rating = (rgto.contract_rating_count * rgto.average_contract_rating + request.rgto_ratings[&rgto_address]) / (rgto.contract_rating_count + 1);
                rgto.contract_rating_count += 1;
            }

            let token_id = keccak256(
                abi::encode_packed(request.doctor, best_rgto_address, now())
            );

            let mut do_file_token = Shared::DOFileToken {
                exists: true,
                rgto_address: best_rgto_address,
            };

            self.persons.get_mut(&request.doctor).unwrap().dof_token_ids.push(token_id);
            self.persons.get_mut(&request.doctor).unwrap().dof_tokens.insert(token_id, do_file_token);

            let mut od_file_token = Shared::ODFileToken {
                exists: true,
                doctor_address: request.doctor,
            };

            self.rgtos.get_mut(&best_rgto_address).unwrap().odf_token_ids.push(token_id);
            self.rgtos.get_mut(&best_rgto_address).unwrap().odf_tokens.insert(token_id, od_file_token);

            // Emit event
            println!("tokenCreatedDoctor: {:?}, {:?}", token_id, best_rgto_address); // Placeholder for emitting event in Rust
            println!("tokenCreatedRGTO: {:?}, {:?}", token_id, request.doctor); // Placeholder for emitting event in Rust
        }
    }

    // This function should be annotated with the appropriate modifier or removed if not needed.
    fn submit_doctor_rgto_rating(&mut self, token_id: bytes32, rgto_address: address, rating: u16, sender: address) {
        assert!(self.rgtos[&rgto_address].odf_tokens.contains_key(&token_id) &&
            self.persons[&sender].dof_tokens.contains_key(&token_id),
            "Token does not exist");

        assert!(self.rgtos[&rgto_address].odf_tokens[&token_id].doctor_address == sender &&
            self.persons[&sender].dof_tokens[&token_id].rgto_address == rgto_address,
            "Token is not valid");

        let rgto = &mut self.rgtos.get_mut(&rgto_address).unwrap();
        rgto.average_doctor_rating = (rgto.contract_rating_count * rgto.average_contract_rating + rating) / (rgto.contract_rating_count + 1);
        rgto.doctor_rating_count += 1;
    }

    
}
