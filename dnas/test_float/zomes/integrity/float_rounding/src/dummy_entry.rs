use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct DummyEntry {
    pub test: String,
}
pub fn validate_create_dummy_entry(
    _action: EntryCreationAction,
    _dummy_entry: DummyEntry,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_dummy_entry(
    _action: Update,
    _dummy_entry: DummyEntry,
    _original_action: EntryCreationAction,
    _original_dummy_entry: DummyEntry,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Dummy Entries cannot be updated")))
}
pub fn validate_delete_dummy_entry(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_dummy_entry: DummyEntry,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Dummy Entries cannot be deleted")))
}
