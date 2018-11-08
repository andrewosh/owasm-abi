#![allow(dead_code)]

use owasm_abi_derive::eth_abi;
use owasm_abi::eth::EndpointInterface;

use pwasm_test::{ext_reset};

const PAYLOAD_BAZ: &[u8] = &[
	0xcd, 0xcd, 0x77, 0xc0,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
];

const PAYLOAD_BOO: &[u8] = &[
	0x5d, 0xda, 0xb4, 0xd4,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45,
];

#[eth_abi(NonPayableEndpoint)]
pub trait NonPayableContract {
	fn constructor(&mut self);
	fn baz(&mut self, _p1: u32, _p2: bool);
	fn boo(&mut self, _arg: u32) -> u32;
}

struct NonPayableContractInstance;

impl NonPayableContract for NonPayableContractInstance {
	fn constructor(&mut self) {}
	fn baz(&mut self, _p1: u32, _p2: bool) {}
	fn boo(&mut self, _arg: u32) -> u32 { 0 }
}

#[test]
#[should_panic]
fn non_payable_constructor_value() {
	ext_reset(|e| e.value(1.into()));
	NonPayableEndpoint::new(NonPayableContractInstance).dispatch_ctor(&[]);
}

#[test]
#[should_panic]
fn non_payable_value() {
	ext_reset(|e| e.value(1.into()));
	NonPayableEndpoint::new(NonPayableContractInstance).dispatch(PAYLOAD_BAZ);
}

#[test]
#[should_panic]
fn non_payable_with_ret_value() {
	ext_reset(|e| e.value(1.into()));
	NonPayableEndpoint::new(NonPayableContractInstance).dispatch(PAYLOAD_BOO);
}

#[test]
fn non_payable_constructor_no_value() {
	NonPayableEndpoint::new(NonPayableContractInstance).dispatch_ctor(&[]);
}

#[test]
fn non_payable_no_value() {
	NonPayableEndpoint::new(NonPayableContractInstance).dispatch(PAYLOAD_BAZ);
}

#[test]
fn non_payable_no_value_ret() {
	NonPayableEndpoint::new(NonPayableContractInstance).dispatch(PAYLOAD_BOO);
}

#[eth_abi(PayableEndpoint)]
pub trait PayableContract {
	#[payable]
	fn constructor(&mut self);
	#[payable]
	fn baz(&mut self, _p1: u32, _p2: bool);
	#[payable]
	fn boo(&mut self, _arg: u32) -> u32;
}

struct PayableContractInstance;

impl PayableContract for PayableContractInstance {
	fn constructor(&mut self) {}
	fn baz(&mut self, _p1: u32, _p2: bool) {}
	fn boo(&mut self, _arg: u32) -> u32 { 0 }
}

#[test]
fn payable_constructor() {
	ext_reset(|e| e.value(1.into()));
	PayableEndpoint::new(PayableContractInstance).dispatch_ctor(&[]);
}

#[test]
fn payable_method() {
	ext_reset(|e| e.value(1.into()));
	PayableEndpoint::new(PayableContractInstance).dispatch(PAYLOAD_BAZ);
}

#[test]
fn payable_method_ret() {
	ext_reset(|e| e.value(1.into()));
	PayableEndpoint::new(PayableContractInstance).dispatch(PAYLOAD_BOO);
}

#[test]
fn payable_constructor_no_value() {
	PayableEndpoint::new(PayableContractInstance).dispatch_ctor(&[]);
}

#[test]
fn payable_method_no_value() {
	PayableEndpoint::new(PayableContractInstance).dispatch(PAYLOAD_BAZ);
}

#[test]
fn payable_method_ret_no_value() {
	PayableEndpoint::new(PayableContractInstance).dispatch(PAYLOAD_BOO);
}
