#![doc = include_str!("../README.md")]
#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;
extern crate pbc_contract_common;
extern crate pbc_lib;

use pbc_contract_common::address::Address;
use pbc_contract_common::context::ContractContext;
use pbc_contract_common::events::EventGroup;
use pbc_contract_common::mpc::{MpcClosed, MpcInputDef, MpcState, MpcStateChange};
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

/// Patient's basic details
#[derive(ReadWriteState, ReadWriteRPC, Debug)]
struct PatientDetails {
    blood_pressure: String,
    height: u32,
    weight: u32,
}

/// Doctor's prescription
#[derive(ReadWriteState, ReadWriteRPC, Debug)]
struct Prescription {
    title: String,
    message: String,
}

/// This contract's state
#[state]
struct ContractState {
    patients: Vec<Address>,
    doctors: Vec<Address>,
}

/// Initializes contract
#[init(mpc = true)]
fn initialize(ctx: ContractContext, mpc_state: MpcState) -> ContractState {
    ContractState {
        patients: vec![],
        doctors: vec![],
    }
}

/// Registers a new patient
#[action(shortname = 0x01)]
fn register_patient(context: ContractContext, mut state: ContractState) -> ContractState {
    state.patients.push(context.sender);
    state
}

/// Registers a new doctor
#[action(shortname = 0x02)]
fn register_doctor(context: ContractContext, mut state: ContractState) -> ContractState {
    state.doctors.push(context.sender);
    state
}

/// Patient submits basic details
#[mpc_on_secret_input(shortname = 0x40)]
fn submit_patient_details(
    context: ContractContext,
    state: ContractState,
    mpc_state: MpcState,
) -> (
    ContractState,
    Vec<EventGroup>,
    MpcInputDef<PatientDetails>,
) {
    assert!(state.patients.contains(&context.sender), "Sender must be a registered patient");
    let input_def = MpcInputDef::with_metadata(PatientDetails {
        blood_pressure: "".to_string(),
        height: 0,
        weight: 0,
    });
    (state, vec![], input_def)
}

/// Patient requests a prescription from a doctor
#[mpc_on_secret_input(shortname = 0x41)]
fn request_prescription(
    context: ContractContext,
    state: ContractState,
    mpc_state: MpcState,
) -> (
    ContractState,
    Vec<EventGroup>,
    MpcInputDef<Prescription>,
) {
    assert!(state.patients.contains(&context.sender), "Sender must be a registered patient");
    let input_def = MpcInputDef::with_metadata(Prescription {
        title: "".to_string(),
        message: "".to_string(),
    });
    (state, vec![], input_def)
}

/// Doctor submits a prescription
#[mpc_on_secret_input(shortname = 0x42)]
fn submit_prescription(
    context: ContractContext,
    state: ContractState,
    mpc_state: MpcState,
) -> (
    ContractState,
    Vec<EventGroup>,
    MpcInputDef<Prescription>,
) {
    assert!(state.doctors.contains(&context.sender), "Sender must be a registered doctor");
    let input_def = MpcInputDef::with_metadata(Prescription {
        title: "".to_string(),
        message: "".to_string(),
    });
    (state, vec![], input_def)
}

/// Automatically called when a variable is confirmed on chain.
#[mpc_on_variable_inputted]
fn inputted_variable(
    context: ContractContext,
    state: ContractState,
    mpc_state: MpcState,
) -> ContractState {
    state
}

/// Automatically called when the computation is completed
#[mpc_on_compute_complete]
fn compute_complete(
    context: ContractContext,
    state: ContractState,
    mpc_state: MpcState,
) -> (ContractState, Vec<EventGroup>, Vec<MpcStateChange>) {
    (state, vec![], vec![])
}

/// Automatically called when a variable is opened/declassified.
#[mpc_on_variables_opened]
fn variables_opened(
    context: ContractContext,
    state: ContractState,
    mpc_state: MpcState,
) -> (ContractState, Vec<EventGroup>, Vec<MpcStateChange>) {
    (state, vec![], vec![])
}