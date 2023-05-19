use hdk::prelude::*;
use float_rounding_integrity::*;
#[hdk_extern]
pub fn create_float_rounding_entry(
    float_rounding_entry: FloatRoundingEntry,
) -> ExternResult<Record> {
    let float_rounding_entry_hash = create_entry(
        &EntryTypes::FloatRoundingEntry(float_rounding_entry.clone()),
    )?;
    let record = get(float_rounding_entry_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created FloatRoundingEntry"))
            ),
        )?;
    let path = Path::from("all_float_rounding_entries");
    create_link(
        path.path_entry_hash()?,
        float_rounding_entry_hash.clone(),
        LinkTypes::AllFloatRoundingEntries,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_float_rounding_entry(
    float_rounding_entry_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    get(float_rounding_entry_hash, GetOptions::default())
}
