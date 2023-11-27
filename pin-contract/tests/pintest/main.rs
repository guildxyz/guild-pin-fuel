mod contract;
mod init;
mod parameters;


use fuels::types::errors::Error;

fn check_error(error: Error, expected: &str) {
    match error {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, expected);
        }
        _ => panic!("invalid error type"),
    }
}
