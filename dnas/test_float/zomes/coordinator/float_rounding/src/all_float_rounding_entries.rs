use float_rounding_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn get_all_float_rounding_entries(_: ()) -> ExternResult<Vec<Record>> {
    let path = Path::from("all_float_rounding_entries");
    let links = get_links(
        path.path_entry_hash()?,
        LinkTypes::AllFloatRoundingEntries,
        None,
    )?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(ActionHash::from(link.target).into(), GetOptions::default()))
        .collect();
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    Ok(records)
}
