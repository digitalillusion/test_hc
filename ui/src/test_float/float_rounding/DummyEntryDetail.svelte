<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { DummyEntry } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';

const dispatch = createEventDispatcher();

export let dummyEntryHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let dummyEntry: DummyEntry | undefined;


  
$:  error, loading, record, dummyEntry;

onMount(async () => {
  if (dummyEntryHash === undefined) {
    throw new Error(`The dummyEntryHash input is required for the DummyEntryDetail element`);
  }
  await fetchDummyEntry();
});

async function fetchDummyEntry() {
  loading = true;
  error = undefined;
  record = undefined;
  dummyEntry = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'test_float',
      zome_name: 'float_rounding',
      fn_name: 'get_dummy_entry',
      payload: dummyEntryHash,
    });
    if (record) {
      dummyEntry = decode((record.entry as any).Present.entry) as DummyEntry;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

</script>


{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the dummy entry: {error.data.data}</span>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
  </div>

</div>
{/if}

