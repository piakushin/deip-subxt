#![allow(clippy::too_many_arguments)]

mod app;
mod cli;
mod config;
mod runtime;

#[macro_use]
extern crate log;

use anyhow::Result;
use clap::Parser;
use log::LevelFilter;
use parity_scale_codec::Decode;
use sp_keyring::AccountKeyring;
use std::{path::Path, time::Duration};

use crate::{
    app::{
        develop::App as DevelopApp,
        master::{display_event as master_display_event, App as MasterApp},
    },
    cli::{
        Args, AssetsStorage, Call as CliCall, DeipFNFTStorage, EndPoint, PalletCall, PalletStorage,
        RuntimeVersion,
    },
    config::Interval,
    runtime::master_v104::api::runtime_types::{
        appchain_deip_runtime::Call, frame_system::pallet::Call as SystemCall,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    env_logger::builder().filter_level(LevelFilter::Info).init();
    info!("Hello, DEIP world!");

    match args.runtime_version {
        RuntimeVersion::Master { endpoint } => {
            let app = MasterApp::build().await.unwrap();
            info!("App built [master]");
            let tx = app.client.tx();
            match endpoint {
                EndPoint::Tx { pallet } => match pallet {
                    PalletCall::Assets { call } => {
                        let assets = tx.assets();
                        match call {
                            CliCall::Create {
                                id,
                                admin,
                                min_balance,
                            } => {
                                let admin: AccountKeyring = admin.into();
                                assets
                                    .create(id, admin.to_account_id().into(), min_balance)
                                    .unwrap()
                                    .sign_and_submit_then_watch_default(&app.signer(admin))
                                    .await
                                    .unwrap()
                                    .wait_for_in_block()
                                    .await
                                    .unwrap()
                                    .fetch_events()
                                    .await
                                    .unwrap()
                                    .iter()
                                    .map(|event| event.unwrap())
                                    .for_each(master_display_event);
                            }
                            _ => todo!(),
                        }
                    }
                    PalletCall::Sudo { call } => {
                        let sudo = tx.sudo();
                        match call {
                            CliCall::SudoUncheckedWeight { account } => sudo
                                .sudo_unchecked_weight(runtime_upgrade().await, 0)
                                .unwrap()
                                .sign_and_submit_then_watch_default(&app.signer(account.into()))
                                .await
                                .unwrap()
                                .wait_for_in_block()
                                .await
                                .unwrap()
                                .fetch_events()
                                .await
                                .unwrap()
                                .iter()
                                .map(|event| event.unwrap())
                                .for_each(master_display_event),
                            _ => todo!(),
                        }
                    }
                },
                EndPoint::Storage { pallet } => match pallet {
                    PalletStorage::Assets { storage } => match storage {
                        AssetsStorage::Asset { key } => {
                            let mut key_iter = app
                                .client
                                .storage()
                                .assets()
                                .asset_iter(None)
                                .await
                                .unwrap();
                            while let Some((storage_key, _)) = key_iter.next().await.unwrap() {
                                let mut storage_key = &storage_key.0[48..52];
                                let storage_key = u32::decode(&mut storage_key).unwrap();
                                info!("key: {key} - storage_key: {storage_key:?}");
                                assert_eq!(storage_key, key);
                            }
                        }
                    },
                    PalletStorage::DeipFNFT { storage: _ } => todo!(),
                },
            }
        }
        RuntimeVersion::Develop { endpoint } => {
            let app = DevelopApp::build().await.unwrap();
            info!("App built [develop]");
            match endpoint {
                EndPoint::Tx { pallet: _ } => todo!(),
                EndPoint::Storage { pallet } => match pallet {
                    PalletStorage::Assets { storage } => match storage {
                        AssetsStorage::Asset { key } => {
                            let mut key_iter = app
                                .client
                                .storage()
                                .assets()
                                .asset_iter(None)
                                .await
                                .unwrap();
                            while let Some((storage_key, _)) = key_iter.next().await.unwrap() {
                                let mut storage_key = &storage_key.0[48..52];
                                let storage_key = u32::decode(&mut storage_key).unwrap();
                                info!("key: {key} - storage_key: {storage_key:?}");
                                assert_eq!(storage_key, key);
                            }
                        }
                    },
                    PalletStorage::DeipFNFT { storage } => todo!(),
                },
            }
        }
    }

    Ok(())
}

async fn _wait_for(interval: Interval, duration: Option<Duration>) {
    match interval {
        Interval::Input => {
            let _ = std::io::Read::read(&mut std::io::stdin(), &mut [0u8]).unwrap();
        }
        Interval::None => {}
        Interval::Timer => tokio::time::sleep(duration.unwrap()).await,
    }
}

async fn runtime_upgrade() -> Call {
    let path = "/Users/piakushin/rust/deip-node/target/release/wbuild/appchain-deip-runtime/appchain_deip_runtime.compact.compressed.wasm";
    let path = Path::new(path);
    println!("{}", path.display());
    let path = path.canonicalize().unwrap();
    let code = tokio::fs::read(path).await.unwrap();
    let call = SystemCall::set_code { code };
    Call::System(call)
}
