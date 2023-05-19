<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { FloatRoundingEntry } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-slider';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();


let value: number = 0.0;

let errorSnackbar: Snackbar;

$: value;
$: isFloatRoundingEntryValid = true && true;

onMount(() => {
});

async function createFloatRoundingEntry() {  
  const floatRoundingEntryEntry: FloatRoundingEntry = { 
    value: value!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'test_float',
      zome_name: 'float_rounding',
      fn_name: 'create_float_rounding_entry',
      payload: floatRoundingEntryEntry,
    });
    dispatch('float-rounding-entry-created', { floatRoundingEntryHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the float rounding entry: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create FloatRoundingEntry</span>
  

  <div style="margin-bottom: 16px">
    <div style="display: flex; flex-direction: row">
      <span style="margin-right: 4px">Value</span>
    
      <mwc-slider style="width: 90%" step="0.01" value={ value } on:input={e => { value = e.detail.value; } }></mwc-slider>
      {value}
    </div>          
  </div>
            

  <mwc-button 
    raised
    label="Create FloatRoundingEntry"
    disabled={!isFloatRoundingEntryValid}
    on:click={() => createFloatRoundingEntry()}
  ></mwc-button>
</div>
