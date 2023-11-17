use crate::utils::setup::GuildToken;
use fuels::prelude::*;
use fuels::programs::call_response::FuelCallResponse;

type Contract = GuildToken<WalletUnlocked>;

pub async fn mint(contract: &Contract, owner: Address, recipient: Address) -> FuelCallResponse<()> {
    todo!()
    // TODO who is the caller bruh??
    //contract.methods().mint()
}
