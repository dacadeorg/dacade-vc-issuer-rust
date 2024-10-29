use ic_canister_sig_creation::extract_raw_root_pk_from_der;
use ic_cdk::{init, post_upgrade};
use candid::{candid_method, CandidType, Principal};
use serde::Deserialize;
use crate:: SETTINGS;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct SettingsInput {
    pub ic_root_key_der: Vec<u8>,
    pub ii_canister_id: Principal,
}

pub struct Settings {
    pub ic_root_key_raw: Vec<u8>,
    pub ii_canister_id: Principal,
}

// Init and upgrade
#[init]
#[candid_method(init)]
async fn init(settings_input: SettingsInput) {
    save_settings(settings_input);
}

#[post_upgrade]
fn upgrade(settings_input: SettingsInput) {
    save_settings(settings_input);
}

fn save_settings(settings_input: SettingsInput) {
    SETTINGS.with_borrow_mut(|settings| {
        *settings = Some(Settings {
            ii_canister_id: settings_input.ii_canister_id,
            ic_root_key_raw: extract_raw_root_pk_from_der(&settings_input.ic_root_key_der).unwrap(),
        });
    });
}