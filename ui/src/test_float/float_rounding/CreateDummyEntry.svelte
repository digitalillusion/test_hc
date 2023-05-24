<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { DummyEntry } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let test!: string;



let errorSnackbar: Snackbar;

$: test;
$: isDummyEntryValid = true;

onMount(() => {
  if (test === undefined) {
    throw new Error(`The test input is required for the CreateDummyEntry element`);
  }
});

async function createDummyEntry() {  
  const dummyEntryEntry: DummyEntry = { 
    test: test!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'test_float',
      zome_name: 'float_rounding',
      fn_name: 'create_dummy_entry',
      payload: dummyEntryEntry,
    });
    dispatch('dummy-entry-created', { dummyEntryHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the dummy entry: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create DummyEntry</span>
  


  <mwc-button 
    raised
    label="Create DummyEntry"
    disabled={!isDummyEntryValid}
    on:click={() => createDummyEntry()}
  ></mwc-button>
</div>
