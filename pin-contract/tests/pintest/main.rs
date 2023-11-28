#![warn(clippy::all)]
#![warn(clippy::dbg_macro)]

mod claim;
mod contract;
mod fee;
mod init;
mod owner;
mod parameters;
mod signer;
mod treasury;
mod utils;

use fuels::core::traits::{Parameterize, Tokenizable};
use fuels::programs::call_response::FuelCallResponse;
use fuels::types::errors::Error;

use std::fmt::Debug;

fn check_error(error: Error, expected: &str) {
    match error {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, expected);
        }
        _ => panic!("invalid error type"),
    }
}

fn check_event<R, T>(response: FuelCallResponse<R>, expected: T)
where
    T: 'static + Debug + PartialEq + Parameterize + Tokenizable,
{
    let events = response.decode_logs_with_type::<T>().unwrap();
    assert_eq!(events, vec![expected]);
}
