use clap::{Parser, Subcommand, ValueEnum};
use sp_keyring::AccountKeyring;

#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub runtime_version: RuntimeVersion,
}

#[derive(Subcommand)]
pub enum RuntimeVersion {
    Master {
        #[clap(subcommand)]
        endpoint: EndPoint,
    },
    Develop {
        #[clap(subcommand)]
        endpoint: EndPoint,
    },
}

#[derive(Subcommand)]
pub enum EndPoint {
    Tx {
        #[clap(subcommand)]
        pallet: PalletCall,
    },
    Storage {
        #[clap(subcommand)]
        pallet: PalletStorage,
    },
}

#[derive(Subcommand)]
pub enum PalletCall {
    Assets {
        #[clap(subcommand)]
        call: Call,
    },
    Sudo {
        #[clap(subcommand)]
        call: Call,
    },
}

#[derive(Subcommand)]
pub enum PalletStorage {
    Assets {
        #[clap(subcommand)]
        storage: AssetsStorage,
    },
    DeipFNFT {
        #[clap(subcommand)]
        storage: DeipFNFTStorage,
    },
}

#[derive(Subcommand)]
pub enum AssetsStorage {
    Asset {
        #[clap(short, value_parser)]
        key: u32,
    },
}

#[derive(Subcommand)]
pub enum DeipFNFTStorage {}

#[derive(Subcommand)]
pub enum Call {
    Create {
        #[clap(short, value_parser)]
        id: u32,
        #[clap(short, value_parser)]
        admin: Account,
        #[clap(short, value_parser)]
        min_balance: u128,
    },
    SudoUncheckedWeight {
        #[clap(short, value_parser)]
        account: Account,
    },
}

#[derive(ValueEnum, Clone)]
pub enum Account {
    Alice,
}

impl From<Account> for AccountKeyring {
    fn from(account: Account) -> Self {
        match account {
            Account::Alice => AccountKeyring::Alice,
        }
    }
}
