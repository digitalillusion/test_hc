pub mod dummy_entry;
pub use dummy_entry::*;
pub mod float_rounding_entry;
pub use float_rounding_entry::*;
use hdi::prelude::*;
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    FloatRoundingEntry(FloatRoundingEntry),
    DummyEntry(DummyEntry),
}
#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    AllFloatRoundingEntries,
}
#[hdk_extern]
pub fn genesis_self_check(
    _data: GenesisSelfCheckData,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_agent_joining(
    _agent_pub_key: AgentPubKey,
    _membrane_proof: &Option<MembraneProof>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreEntry(store_entry) => {
            match store_entry {
                OpEntry::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::FloatRoundingEntry(float_rounding_entry) => {
                            validate_create_float_rounding_entry(
                                EntryCreationAction::Create(action),
                                float_rounding_entry,
                            )
                        }
                        EntryTypes::DummyEntry(dummy_entry) => {
                            validate_create_dummy_entry(
                                EntryCreationAction::Create(action),
                                dummy_entry,
                            )
                        }
                    }
                }
                OpEntry::UpdateEntry { app_entry, action, .. } => {
                    match app_entry {
                        EntryTypes::FloatRoundingEntry(float_rounding_entry) => {
                            validate_create_float_rounding_entry(
                                EntryCreationAction::Update(action),
                                float_rounding_entry,
                            )
                        }
                        EntryTypes::DummyEntry(dummy_entry) => {
                            validate_create_dummy_entry(
                                EntryCreationAction::Update(action),
                                dummy_entry,
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterUpdate(update_entry) => {
            match update_entry {
                OpUpdate::Entry {
                    original_action,
                    original_app_entry,
                    app_entry,
                    action,
                } => {
                    match (app_entry, original_app_entry) {
                        (
                            EntryTypes::DummyEntry(dummy_entry),
                            EntryTypes::DummyEntry(original_dummy_entry),
                        ) => {
                            validate_update_dummy_entry(
                                action,
                                dummy_entry,
                                original_action,
                                original_dummy_entry,
                            )
                        }
                        (
                            EntryTypes::FloatRoundingEntry(float_rounding_entry),
                            EntryTypes::FloatRoundingEntry(original_float_rounding_entry),
                        ) => {
                            validate_update_float_rounding_entry(
                                action,
                                float_rounding_entry,
                                original_action,
                                original_float_rounding_entry,
                            )
                        }
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original and updated entry types must be the same"
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterDelete(delete_entry) => {
            match delete_entry {
                OpDelete::Entry { original_action, original_app_entry, action } => {
                    match original_app_entry {
                        EntryTypes::FloatRoundingEntry(float_rounding_entry) => {
                            validate_delete_float_rounding_entry(
                                action,
                                original_action,
                                float_rounding_entry,
                            )
                        }
                        EntryTypes::DummyEntry(dummy_entry) => {
                            validate_delete_dummy_entry(
                                action,
                                original_action,
                                dummy_entry,
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterCreateLink {
            link_type,
            base_address,
            target_address,
            tag,
            action,
        } => {
            match link_type {
                LinkTypes::AllFloatRoundingEntries => {
                    validate_create_link_all_float_rounding_entries(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            }
        }
        FlatOp::RegisterDeleteLink {
            link_type,
            base_address,
            target_address,
            tag,
            original_action,
            action,
        } => {
            match link_type {
                LinkTypes::AllFloatRoundingEntries => {
                    validate_delete_link_all_float_rounding_entries(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            }
        }
        FlatOp::StoreRecord(store_record) => {
            match store_record {
                OpRecord::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::FloatRoundingEntry(float_rounding_entry) => {
                            validate_create_float_rounding_entry(
                                EntryCreationAction::Create(action),
                                float_rounding_entry,
                            )
                        }
                        EntryTypes::DummyEntry(dummy_entry) => {
                            validate_create_dummy_entry(
                                EntryCreationAction::Create(action),
                                dummy_entry,
                            )
                        }
                    }
                }
                OpRecord::UpdateEntry {
                    original_action_hash,
                    app_entry,
                    action,
                    ..
                } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original action for an update must be a Create or Update action"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match app_entry {
                        EntryTypes::FloatRoundingEntry(float_rounding_entry) => {
                            let result = validate_create_float_rounding_entry(
                                EntryCreationAction::Update(action.clone()),
                                float_rounding_entry.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_float_rounding_entry: Option<
                                    FloatRoundingEntry,
                                > = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_float_rounding_entry = match original_float_rounding_entry {
                                    Some(float_rounding_entry) => float_rounding_entry,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_float_rounding_entry(
                                    action,
                                    float_rounding_entry,
                                    original_action,
                                    original_float_rounding_entry,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::DummyEntry(dummy_entry) => {
                            let result = validate_create_dummy_entry(
                                EntryCreationAction::Update(action.clone()),
                                dummy_entry.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_dummy_entry: Option<DummyEntry> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_dummy_entry = match original_dummy_entry {
                                    Some(dummy_entry) => dummy_entry,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_dummy_entry(
                                    action,
                                    dummy_entry,
                                    original_action,
                                    original_dummy_entry,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                    }
                }
                OpRecord::DeleteEntry { original_action_hash, action, .. } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original action for a delete must be a Create or Update action"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    let app_entry_type = match original_action.entry_type() {
                        EntryType::App(app_entry_type) => app_entry_type,
                        _ => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    let entry = match original_record.entry().as_option() {
                        Some(entry) => entry,
                        None => {
                            if original_action.entry_type().visibility().is_public() {
                                return Ok(
                                    ValidateCallbackResult::Invalid(
                                        "Original record for a delete of a public entry must contain an entry"
                                            .to_string(),
                                    ),
                                );
                            } else {
                                return Ok(ValidateCallbackResult::Valid);
                            }
                        }
                    };
                    let original_app_entry = match EntryTypes::deserialize_from_type(
                        app_entry_type.zome_index.clone(),
                        app_entry_type.entry_index.clone(),
                        &entry,
                    )? {
                        Some(app_entry) => app_entry,
                        None => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original app entry must be one of the defined entry types for this zome"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match original_app_entry {
                        EntryTypes::FloatRoundingEntry(original_float_rounding_entry) => {
                            validate_delete_float_rounding_entry(
                                action,
                                original_action,
                                original_float_rounding_entry,
                            )
                        }
                        EntryTypes::DummyEntry(original_dummy_entry) => {
                            validate_delete_dummy_entry(
                                action,
                                original_action,
                                original_dummy_entry,
                            )
                        }
                    }
                }
                OpRecord::CreateLink {
                    base_address,
                    target_address,
                    tag,
                    link_type,
                    action,
                } => {
                    match link_type {
                        LinkTypes::AllFloatRoundingEntries => {
                            validate_create_link_all_float_rounding_entries(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                    }
                }
                OpRecord::DeleteLink { original_action_hash, base_address, action } => {
                    let record = must_get_valid_record(original_action_hash)?;
                    let create_link = match record.action() {
                        Action::CreateLink(create_link) => create_link.clone(),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "The action that a DeleteLink deletes must be a CreateLink"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    let link_type = match LinkTypes::from_type(
                        create_link.zome_index.clone(),
                        create_link.link_type.clone(),
                    )? {
                        Some(lt) => lt,
                        None => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    match link_type {
                        LinkTypes::AllFloatRoundingEntries => {
                            validate_delete_link_all_float_rounding_entries(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                    }
                }
                OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterAgentActivity(agent_activity) => {
            match agent_activity {
                OpActivity::CreateAgent { agent, action } => {
                    let previous_action = must_get_action(action.prev_action)?;
                    match previous_action.action() {
                        Action::AgentValidationPkg(
                            AgentValidationPkg { membrane_proof, .. },
                        ) => validate_agent_joining(agent, membrane_proof),
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "The previous action for a `CreateAgent` action must be an `AgentValidationPkg`"
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
    }
}
