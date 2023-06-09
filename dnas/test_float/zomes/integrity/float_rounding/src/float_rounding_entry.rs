use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct FloatRoundingEntry {
    pub value: f32,
}
pub fn validate_create_float_rounding_entry(
    action: EntryCreationAction,
    _float_rounding_entry: FloatRoundingEntry,
) -> ExternResult<ValidateCallbackResult> {
    if must_get_agent_activity(
            action.author().clone(),
            ChainFilter::new(action.prev_action().clone()),
        )?
        .iter()
        .any(|activity| {
            activity.action.action().action_type() == action.action_type()
                && activity
                    .action
                    .action()
                    .entry_type()
                    .map_or(false, |entry_type| entry_type == action.entry_type())
        })
    {
        Ok(
            ValidateCallbackResult::Invalid(
                String::from("A FloatRoundingEntry created by this agent already exists"),
            ),
        )
    } else {
        Ok(ValidateCallbackResult::Valid)
    }
}
pub fn validate_update_float_rounding_entry(
    _action: Update,
    _float_rounding_entry: FloatRoundingEntry,
    _original_action: EntryCreationAction,
    _original_float_rounding_entry: FloatRoundingEntry,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Float Rounding Entries cannot be updated"),
        ),
    )
}
pub fn validate_delete_float_rounding_entry(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_float_rounding_entry: FloatRoundingEntry,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Float Rounding Entries cannot be deleted"),
        ),
    )
}
pub fn validate_create_link_all_float_rounding_entries(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _float_rounding_entry: crate::FloatRoundingEntry = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_all_float_rounding_entries(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("AllFloatRoundingEntries links cannot be deleted"),
        ),
    )
}
