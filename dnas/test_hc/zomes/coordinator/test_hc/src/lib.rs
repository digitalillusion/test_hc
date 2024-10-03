use hdk::prelude::*;
use test_hc_integrity::*;


/// Called the first time a zome call is made to the cell containing this zome
#[hdk_extern]
pub fn init() -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}

/// Don't modify this enum if you want the scaffolding tool to generate appropriate signals for your entries and links
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Signal {
}

/// Whenever an action is committed, we emit a signal to the UI elements to reactively update them
#[hdk_extern(infallible)]
pub fn post_commit(committed_actions: Vec<SignedActionHashed>) {
    /// Don't modify this loop if you want the scaffolding tool to generate appropriate signals for your entries and links
    for action in committed_actions {
        if let Err(err) = signal_action(action) {
            error!("Error signaling new action: {:?}", err);
        }
    }
}

/// Don't modify this function if you want the scaffolding tool to generate appropriate signals for your entries and links
fn signal_action(action: SignedActionHashed) -> ExternResult<()> {
    Ok(())
}

#[hdk_extern]
pub fn test() -> ExternResult<()> {
    let agent_info = agent_info()?;
    let input: AgentPubKeyB64 = agent_info.agent_latest_pubkey.to_owned().into();
    let serialized = serde_json::to_vec(&input).unwrap();
    let output: AgentPubKeyB64 = serde_json::from_slice(&serialized).map_err(|err| wasm_error!(
        WasmErrorInner::Guest(format!("Deserialization error: {:?}", err)))
    )?;
    Ok(())
}