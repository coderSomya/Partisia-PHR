use std::collections::HashMap;

#[derive(Clone, Default)]
struct RAMember {
    registered: bool,
}

#[derive(Clone, Default)]
struct Person {
    registered: bool,
    patient_verified: Claim,
    doctor_verified: Claim,
    guardians_address: Vec<Address>,
    guardians_claim: Vec<Claim>,
    wards_address: Vec<Address>,
    wards_claim: Vec<Claim>,
    bundle_hashes: Vec<H256>,
    meddocs: HashMap<H256, MedDoc>,
}

#[derive(Clone, Default)]
struct Hospital {
    registered: bool,
    hospital_verified: Claim,
}

#[derive(Clone, Default)]
struct RGTO {
    registered: bool,
    average_contract_rating: u16,
    contract_rating_count: u64,
    average_doctor_rating: u16,
    doctor_rating_count: u64,
}

#[derive(Clone, Default)]
struct Claim {
    made: bool,
    any_verifier: bool,
    num_required_verifiers: u64,
    oldest_date: Timestamp,
    required_verifiers: Vec<Address>,
}

#[derive(Clone, Default)]
struct MedDoc {
    exists: bool,
    default_num_ra_vers: u64,
    default_num_guard_vers: u64,
    requests: Vec<Request>,
    request_count: u64,
}

#[derive(Clone, Default)]
struct Request {
    exists: bool,
    doctor: Address,
    request_time: Timestamp,
    min_rgto_count: u64, 
    max_rgto_count: u64,
    patient_verifications: Claim,
    ra_member_verifications: Claim,
    guardian_verifications: Claim, 
    rgto_addresses: Vec<Address>,
    rgto_ratings: HashMap<Address, u16>,
    rgtos_evaluated: bool,
}

#[derive(Clone, Default)]
struct DOFileToken {
    exists: bool, 
    rgto_address: Address,
}

#[derive(Clone, Default)]
struct ODFileToken {
    exists: bool,
    doctor_address: Address,  
}

type Address = H160;
type H256 = [u8; 32];
type Timestamp = u64;

const MAX_GUARDIANS_PER_PATIENT: usize = 10;

struct RASC {
    owner: Address,
    ramembers: HashMap<Address, RAMember>,
    persons: HashMap<Address, Person>,
    hospitals: HashMap<Address, Hospital>,
    rgtos: HashMap<Address, RGTO>,
}

impl RASC {

    fn add_guardian(guardian_address: &Address, person: &mut Person) {
        assert!(person.guardians_address.len() < Shared::MAX_GUARDIANS_PER_PATIENT, "Maximum number of guardians reached");

        let is_guardian_new = !person.guardians_address.contains(guardian_address);
        assert!(is_guardian_new, "Guardian is already added");

        person.guardians_address.push(guardian_address.clone());

        let mut guardian_claim = Claim {
            made: true,
            any_verifier: false,
            num_required_verifiers: Shared::VERIFIERS_GUARDIAN_CLAIM_COUNT,
            oldest_date: Shared::VERIFIERS_GUARDIAN_CLAIM_OLDEST,
            required_verifiers: HashSet::new(),
        };
        guardian_claim.required_verifiers.insert(guardian_address.clone());
        person.guardians_claim.push(guardian_claim);

        println!("Guardianship added: {} to {}", guardian_address, "current person");
    }

    fn remove_guardian(guardian_address: &Address, person: &mut Person) {
        assert!(!person.guardians_address.is_empty(), "Patient must have a guardian added already");

        if let Some(index) = person.guardians_address.iter().position(|addr| addr == guardian_address) {
            person.guardians_address.swap_remove(index);
            person.guardians_claim.swap_remove(index);
            println!("Guardianship removed: {} from {}", guardian_address, "current person");
        }
    }

    fn verify_guardian(person_address: &Address, verifier_address: &Address, persons: &mut Vec<Person>) {
        if let Some(person) = persons.iter_mut().find(|p| p.guardians_address.iter().any(|addr| addr == verifier_address)) {
            if let Some(index) = person.guardians_address.iter().position(|addr| addr == verifier_address) {
                Shared::verify_claim(&mut person.guardians_claim[index]);
                println!("Guardianship verified: {} for {}", verifier_address, person_address);
            }
        }
    }

    fn add_ward(ward_address: &Address, person: &mut Person) {
        assert!(Person::is_claim_verified(person.patient_verified), "Patient must be verified before adding a ward");
        assert!(person.wards_address.len() < Shared::MAX_WARDS_PER_PATIENT, "Maximum number of wards reached");

        let is_ward_new = !person.wards_address.contains(ward_address);
        assert!(is_ward_new, "Ward is already added");

        person.wards_address.push(ward_address.clone());

        let ward_claim = Claim {
            made: true,
            any_verifier: true,
            num_required_verifiers: Shared::VERIFIERS_WARDS_CLAIM_COUNT,
            oldest_date: Shared::VERIFIERS_WARDS_CLAIM_OLDEST,
            required_verifiers: HashSet::new(),
        };
        person.wards_claim.push(ward_claim);

        println!("Ward added: {} to {}", ward_address, "current person");
    }

    fn remove_ward(ward_address: &Address, person: &mut Person) {
        assert!(!person.wards_address.is_empty(), "Patient must have a ward added already");

        if let Some(index) = person.wards_address.iter().position(|addr| addr == ward_address) {
            person.wards_address.swap_remove(index);
            person.wards_claim.swap_remove(index);
            println!("Ward removed: {} from {}", ward_address, "current person");
        }
    }

    fn add_md_patient(&mut self, bundle_hash: String, default_num_ra_vers: u32, default_num_guard_vers: u32) {

        self.bundle_hashes.push(bundle_hash.clone());

        let md = MedDoc {
            exists: true,
            default_num_ra_vers: Shared::between(default_num_ra_vers, Shared::MIN_NUM_RA_VERS, Shared::MAX_NUM_RA_VERS),
            default_num_guard_vers: Shared::between(default_num_guard_vers, Shared::MIN_NUM_GUARD_VERS, Shared::MAX_NUM_GUARD_VERS),
        };
        self.meddocs.insert(bundle_hash, md);

        println!(
            "Patient MD Added: {}, Bundle Hash: {}, RAM Versions: {}, Guard Versions: {}",
            "current person",
            bundle_hash,
            md.default_num_ra_vers,
            md.default_num_guard_vers
        );
    }

    fn request_md_doctor(
        &mut self,
        patient_address: &Address,
        bundle_hash: &Address,
        min_rgto_count: u32,
        max_rgto_count: u32,
        direct: bool,
    ) {
        assert!(
            self.meddocs.contains_key(patient_address) && self.meddocs[patient_address].exists,
            "Requested medical document does not exist"
        );
        assert!(min_rgto_count <= max_rgto_count, "Request requires minimum count of RGTOs to be less than maximum count of RGTOs");

        let mut request = Shared::Request {
            exists: true,
            doctor: "current doctor".to_string(),
            request_time: 0, // Replace with actual timestamp logic
            min_rgto_count: Shared::max(min_rgto_count, Shared::MIN_RGTO_COUNT),
            max_rgto_count: Shared::min(max_rgto_count, Shared::MAX_RGTO_COUNT),
            patient_verifications: Default::default(),
            ramember_verifications: Default::default(),
            guardian_verifications: Default::default(),
            rgtos_evaluated: false,
        };

        if direct {
            request.patient_verifications.made = true;
            request.patient_verifications.any_verifier = false;
            request.patient_verifications.num_required_verifiers = 1;
            request.patient_verifications.oldest_date = 7 * 24 * 60 * 60; // 7 days in seconds
            request.patient_verifications.required_verifiers.insert(patient_address.clone());
        } else {
            request.ramember_verifications.made = true;
            request.ramember_verifications.any_verifier = true;
            request.ramember_verifications.num_required_verifiers = self.meddocs[patient_address].default_num_ram_vers;
            request.ramember_verifications.oldest_date = 3 * 24 * 60 * 60; // 3 days in seconds

            request.guardian_verifications.made = true;
            request.guardian_verifications.any_verifier = true;
            // not really, verifyRequestGuardian ensures only guardians of the patient can verify the claim
            request.guardian_verifications.num_required_verifiers = self.meddocs[patient_address].default_num_guard_vers;
            request.guardian_verifications.oldest_date = 3 * 24 * 60 * 60; // 3 days in seconds
        }

        self.meddocs
            .entry(patient_address.clone())
            .and_modify(|meddoc| {
                meddoc.request_count += 1;
                meddoc.requests.insert(meddoc.request_count, request.clone());
            });

        // Replace with actual timestamp logic
        let request_index = self.meddocs[patient_address].request_count - 1;
        println!(
            "New MD Request: Patient: {}, Doctor: {}, Bundle Hash: {}, Request ID: {}",
            patient_address,
            "current doctor",
            bundle_hash,
            request_index
        );
    }

    fn verify_request_ramember(&mut self, patient_address: &Address, bundle_hash: &Address, request_id: usize) {
        assert!(
            self.meddocs.contains_key(patient_address) && self.meddocs[patient_address].exists,
            "Requested medical document does not exist"
        );

        let ramember_verifications = &self.meddocs[patient_address].requests[&request_id].ramember_verifications;
        assert!(ramember_verifications.made, "Regulatory agency member verification was not asked for");

        Shared::verify_claim(ramember_verifications);

        let doctor = &self.meddocs[patient_address].requests[&request_id].doctor;
        println!(
            "RAM Verified MD Request: Patient: {}, Doctor: {}, Bundle Hash: {}, Request ID: {}",
            patient_address, doctor, bundle_hash, request_id
        );
    }

    fn verify_request_guardian(&mut self, patient_address: &Address, bundle_hash: &Address, request_id: usize) {
        assert!(
            self.meddocs.contains_key(patient_address) && self.meddocs[patient_address].exists,
            "Requested medical document does not exist"
        );

        let guardian_verifications = &self.meddocs[patient_address].requests[&request_id].guardian_verifications;
        assert!(guardian_verifications.made, "Guardian verification was not asked for");

        let mut is_valid_guardian = false;
        let mut guardian_index = 0;
        for (index, guardian_address) in self.guardians_address.iter().enumerate() {
            if guardian_address == msg.sender {
                if Shared::is_claim_verified(&self.guardians_claim[index]) {
                    is_valid_guardian = true;
                }
                guardian_index = index;
                break;
            }
        }

        if is_valid_guardian {
            Shared::verify_claim(guardian_verifications);
        }

        let doctor = &self.meddocs[patient_address].requests[&request_id].doctor;
        println!(
            "Guardian Verified MD Request: Patient: {}, Doctor: {}, Bundle Hash: {}, Request ID: {}",
            patient_address, doctor, bundle_hash, request_id
        );
    }

    fn check_request_status(&mut self, patient_address: &Address, bundle_hash: &Address, request_id: usize) -> bool {
        let request = &self.meddocs[patient_address].requests[&request_id];
        if Shared::is_claim_verified(&request.patient_verifications) ||
            (Shared::is_claim_verified(&request.ramember_verifications) && Shared::is_claim_verified(&request.guardian_verifications))
        {
            if !rgtos.contains_key(msg.sender) {
                println!("callForRGTOs");
            }
            return true;
        }
        false
    }

    fn add_rgto_response(&mut self, patient_address: &Address, bundle_hash: &Address, request_id: usize, proof: &Address) {
        let request = &self.meddocs[patient_address].requests[&request_id];

        // require(checkRequestStatus(patientAddress, bundleHash, requestId), "Granted request is required to call this function");
        assert!(!request.rgtos_evaluated, "Unevaluated request is required to call this function");

        let latency = (/* now - */ request.request_time) as u16;

        // Conditions to accept new response
        if request.rgto_addresses.len() < request.min_rgto_count
            || (request.rgto_addresses.len() >= request.min_rgto_count
                && request.rgto_addresses.len() < request.max_rgto_count
                && latency <= Shared::MAX_LATENCY)
        {
            let is_hash_correct = if bundle_hash == proof { 1 } else { 0 };
            let input_start = 1;
            let input_end = 1 * 60 * 60; // 1 hour in seconds
            let output_start = (1 << 16) - 1;
            let output_end = 1;

            let mut rgto_rating = is_hash_correct;
            if latency < 1 {
                rgto_rating *= (1 << 16) - 1;
            } else if latency > 1 * 60 * 60 {
                rgto_rating *= 0;
            } else {
                rgto_rating *= output_start
                    + ((output_end - output_start) / (input_end - input_start))
                        * (latency - input_start);
            }

            self.meddocs[patient_address].requests.get_mut(&request_id).map(|req| {
                req.rgto_addresses.push(msg.sender.clone());
                req.rgto_ratings.insert(msg.sender.clone(), rgto_rating);
            });
        }

        // Conditions to evaluate request
        if (request.rgto_addresses.len() >= request.min_rgto_count && request.request_time + Shared::MAX_LATENCY <= /* now */ 0)
            || request.rgto_addresses.len() == request.max_rgto_count
        {
            self.evaluate_rgtos(patient_address, bundle_hash, request_id);
            self.meddocs[patient_address].requests.get_mut(&request_id).map(|req| req.rgtos_evaluated = true);
        }
    }

    fn evaluate_rgtos(&self, patient_address: &Address, bundle_hash: &Address, request_id: usize) {
        // Implement the logic to evaluate RGTOs
        // You may need to adjust this based on your specific requirements
    }

    fn evaluate_rgtos(persons: &mut HashMap<Address, Person>, rgtos: &mut HashMap<Address, RGTO>, patient_address: &Address, bundle_hash: &Bytes32, request_id: usize) {
        let request = &mut persons[patient_address].meddocs[bundle_hash].requests[&request_id];
    
        let mut best_rgto_address = String::new();
        let mut best_rgto_score = 0;
    
        for rgto_address in &request.rgto_addresses {
            let rgto_rating = request.rgto_ratings[rgto_address];
            let rgto_reputation = (rgtos[rgto_address].average_contract_rating + rgtos[rgto_address].average_doctor_rating) / 2;
    
            let rgto_score = rgto_rating * (rgto_reputation + 1).pow(2);
    
            if rgto_score >= best_rgto_score {
                best_rgto_score = rgto_score;
                best_rgto_address = rgto_address.clone();
            }
        }
    
        for rgto_address in &request.rgto_addresses {
            let rgto = &rgtos[rgto_address];
            rgtos.get_mut(rgto_address).map(|rgto| {
                rgto.average_contract_rating =
                    (rgto.contract_rating_count * rgto.average_contract_rating + request.rgto_ratings[rgto_address]) / (rgto.contract_rating_count + 1);
                rgto.contract_rating_count += 1;
            });
        }
    
        let token_id = format!("{:x}", md5::compute(format!("{}{}{}", request.doctor, best_rgto_address, 0))); // Replace with actual hashing function
    
        let do_file_token = DOFileToken {
            exists: true,
            rgto_address: best_rgto_address.clone(),
        };
    
        persons.get_mut(&request.doctor).map(|person| {
            person.dof_token_ids.push(token_id.clone());
            person.dof_tokens.insert(token_id.clone(), do_file_token);
        });
    
        let od_file_token = ODFileToken {
            exists: true,
            doctor_address: request.doctor.clone(),
        };
    
        rgtos.get_mut(&best_rgto_address).map(|rgto| {
            rgto.odf_token_ids.push(token_id.clone());
            rgto.odf_tokens.insert(token_id.clone(), od_file_token);
        });
    
        println!("tokenCreatedDoctor: tokenID={}, rgtoAddress={}", token_id, best_rgto_address);
        println!("tokenCreatedRGTO: tokenID={}, doctorAddress={}", token_id, request.doctor);
    }

    fn submit_doctor_rgto_rating(rgtos: &mut HashMap<Address, Shared::RGTO>, persons: &mut HashMap<Address, Person>, rgto_address: &Address, token_id: &Bytes32, rating: u16) {
        assert!(
            rgtos[rgto_address].odf_tokens[token_id].exists
                && persons[&msg.sender].dof_tokens[token_id].exists,
            "Token does not exist"
        );
    
        assert!(
            rgtos[rgto_address].odf_tokens[token_id].doctor_address == msg.sender
                && persons[&msg.sender].dof_tokens[token_id].rgto_address == rgto_address,
            "Token is not valid"
        );
    
        let rgto = &rgtos[rgto_address];
        rgtos.get_mut(rgto_address).map(|rgto| {
            rgto.average_doctor_rating =
                (rgto.contract_rating_count * rgto.average_contract_rating + rating) / (rgto.contract_rating_count + 1);
            rgto.contract_rating_count += 1;
        });
    }

}