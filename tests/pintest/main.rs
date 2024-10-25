#![warn(clippy::all)]
#![warn(clippy::dbg_macro)]

// NOTE burn is disabled
//pub mod burn;
pub mod claim;
pub mod fee;
pub mod init;
pub mod metadata;
pub mod owner;
pub mod signer;
pub mod treasury;

use fuels::core::traits::{Parameterize, Tokenizable};
use fuels::programs::call_response::FuelCallResponse;
use fuels::types::errors::transaction::Reason;
use fuels::types::errors::Error;

use std::fmt::Debug;

fn check_error(error: Error, expected: &str) {
    match error {
        Error::Transaction(Reason::Reverted { reason, .. }) => {
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
