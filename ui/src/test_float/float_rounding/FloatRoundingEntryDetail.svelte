<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { FloatRoundingEntry } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';

const dispatch = createEventDispatcher();

export let floatRoundingEntryHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let floatRoundingEntry: FloatRoundingEntry | undefined;


  
$:  error, loading, record, floatRoundingEntry;

onMount(async () => {
  if (floatRoundingEntryHash === undefined) {
    throw new Error(`The floatRoundingEntryHash input is required for the FloatRoundingEntryDetail element`);
  }
  await fetchFloatRoundingEntry();
});

async function fetchFloatRoundingEntry() {
  loading = true;
  error = undefined;
  record = undefined;
  floatRoundingEntry = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'test_float',
      zome_name: 'float_rounding',
      fn_name: 'get_float_rounding_entry',
      payload: floatRoundingEntryHash,
    });
    if (record) {
      floatRoundingEntry = decode((record.entry as any).Present.entry) as FloatRoundingEntry;
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
<span>Error fetching the float rounding entry: {error.data.data}</span>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Value:</strong></span>
    <span style="white-space: pre-line">{ floatRoundingEntry.value }</span>
  </div>

</div>
{/if}

