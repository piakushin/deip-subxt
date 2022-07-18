use anyhow::Result;
use futures::StreamExt;
use parity_scale_codec::Decode;
use sp_keyring::{sr25519::sr25519::Pair, AccountKeyring};
use subxt::{ClientBuilder, DefaultConfig, EventDetails, PairSigner, PolkadotExtrinsicParams};

use crate::runtime::master_v104::api::{
    runtime_types::{
        frame_system::pallet::{Error as SystemError, Event as SystemEvent},
        pallet_sudo::pallet::Event as SudoEvent,
        sp_runtime::DispatchError,
    },
    Event, RuntimeApi, PALLETS,
};

pub struct App {
    pub client: RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>,
}

impl App {
    pub async fn build() -> Result<Self> {
        let client = ClientBuilder::new()
            .build()
            .await?
            .to_runtime_api::<RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();
        Ok(Self { client })
    }

    pub async fn spawn_events_listener(&self) {
        tokio::spawn(async {
            let client = ClientBuilder::new()
                .build()
                .await
                .unwrap()
                .to_runtime_api::<RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();
            let mut events_sub = client.events().subscribe().await.unwrap();
            while let Some(events) = events_sub.next().await {
                let events = events.unwrap();
                let hash = events.block_hash();
                info!("Events in block {hash:?}\n");
                for event in events.iter() {
                    let event = event.unwrap();
                    let event = event.event;
                    info!("    {event:?}\n");
                }
            }
        });
    }

    pub fn signer(&self, account: AccountKeyring) -> PairSigner<DefaultConfig, Pair> {
        PairSigner::new(account.pair())
    }
}

pub fn display_event(event: EventDetails<Event>) {
    let event = event.event;
    match event {
        Event::System(event) => match event {
            SystemEvent::ExtrinsicSuccess(info) => info!(" - Success::{info:?}"),
            SystemEvent::ExtrinsicFailed(error, _) => match error {
                DispatchError::Other => todo!(),
                DispatchError::CannotLookup => todo!(),
                DispatchError::BadOrigin => todo!(),
                DispatchError::Module { index, error } => decode_module_error(index, error),
                DispatchError::ConsumerRemaining => todo!(),
                DispatchError::NoProviders => todo!(),
                DispatchError::Token(error) => info!(" - System::Failed::Token::{error:?}"),
                DispatchError::Arithmetic(_) => todo!(),
            },
            SystemEvent::CodeUpdated => info!(" - CodeUpdated"),
            SystemEvent::KilledAccount(_) => todo!(),
            SystemEvent::Remarked(_, _) => todo!(),
            system_event => info!(" - {system_event:?}"),
        },
        Event::Balances(event) => info!(" - Balances::{event:?}"),
        Event::OctopusAppchain(_) => todo!(),
        Event::OctopusLpos(_) => todo!(),
        Event::OctopusUpwardMessages(_) => todo!(),
        Event::Session(_) => todo!(),
        Event::Grandpa(_) => todo!(),
        Event::Sudo(event) => match event {
            SudoEvent::Sudid { sudo_result } => {
                if let Err(err) = sudo_result {
                    match err {
                        DispatchError::Other => todo!(),
                        DispatchError::CannotLookup => todo!(),
                        DispatchError::BadOrigin => todo!(),
                        DispatchError::Module { index, error } => decode_module_error(index, error),
                        DispatchError::ConsumerRemaining => todo!(),
                        DispatchError::NoProviders => todo!(),
                        DispatchError::Token(_) => todo!(),
                        DispatchError::Arithmetic(_) => todo!(),
                    }
                }
            }

            SudoEvent::KeyChanged { .. } => todo!(),
            SudoEvent::SudoAsDone { .. } => todo!(),
        },
        Event::ImOnline(_) => todo!(),
        Event::Assets(event) => info!(" - Assets::{event:?}"),
        Event::Uniques(event) => info!(" - Uniques::{event:?}"),
        Event::Multisig(_) => todo!(),
        Event::Utility(_) => todo!(),
        Event::Deip(_) => todo!(),
        Event::DeipProposal(_) => todo!(),
        Event::DeipDao(_) => todo!(),
        Event::DeipVesting(_) => todo!(),
        Event::DeipInvestmentOpportunity(_) => todo!(),
    }
}

fn decode_module_error(index: u8, error: u8) {
    match index {
        0 => {
            let error = SystemError::decode(&mut [error].as_ref()).unwrap();
            info!(
                " - {}::Sudid::Err::Module::{error:?}",
                PALLETS[index as usize]
            );
        }
        index => panic!(" - Unknown index: {index}"),
    }
}
