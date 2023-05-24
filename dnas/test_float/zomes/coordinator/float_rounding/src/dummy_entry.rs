use hdk::prelude::*;
use float_rounding_integrity::*;
#[hdk_extern]
pub fn create_dummy_entry(dummy_entry: DummyEntry) -> ExternResult<Record> {
    let dummy_entry_hash = create_entry(&EntryTypes::DummyEntry(dummy_entry.clone()))?;
    let record = get(dummy_entry_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created DummyEntry"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_dummy_entry(dummy_entry_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(dummy_entry_hash, GetOptions::default())
}
