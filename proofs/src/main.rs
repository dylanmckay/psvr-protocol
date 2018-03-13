extern crate hmdee;

mod proofs;

use hmdee::backend::psvr;
use std::process;

type Proof = fn(&mut psvr::Psvr);

const PROOFS: &[Proof] = &[
    proofs::errors_are_returned_as_ascii_strings,
];

fn main() {
    let hidapi = psvr::hidapi::HidApi::new().unwrap();

    match psvr::get(&hidapi).unwrap() {
        Some(mut psvr) => {
            for &proof in PROOFS.iter() {
                run_proof(proof, &mut psvr);
            }
        },
        None => {
            eprintln!("no psvr is plugged in");
            process::exit(1);
        },
    }

    drop(hidapi);
}

fn run_proof(proof: Proof, psvr: &mut psvr::Psvr) {
    proof(psvr);
    ::std::thread::sleep(::std::time::Duration::from_millis(100));

    let _ = psvr.receive_control().unwrap();
}

