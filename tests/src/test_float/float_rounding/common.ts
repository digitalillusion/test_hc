import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleFloatRoundingEntry(cell: CallableCell, partialFloatRoundingEntry = {}) {
    return {
        ...{
	  value: 0.5,
        },
        ...partialFloatRoundingEntry
    };
}

export async function createFloatRoundingEntry(cell: CallableCell, floatRoundingEntry = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "float_rounding",
      fn_name: "create_float_rounding_entry",
      payload: floatRoundingEntry || await sampleFloatRoundingEntry(cell),
    });
}

