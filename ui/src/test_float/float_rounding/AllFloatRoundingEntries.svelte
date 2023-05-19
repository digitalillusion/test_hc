<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import FloatRoundingEntryDetail from './FloatRoundingEntryDetail.svelte';
import type { FloatRoundingSignal } from './types';


let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {

  await fetchFloatRoundingEntries();
  client.on('signal', signal => {
    if (signal.zome_name !== 'float_rounding') return;
    const payload = signal.payload as FloatRoundingSignal;
    if (payload.type !== 'EntryCreated') return;
    if (payload.app_entry.type !== 'FloatRoundingEntry') return;
    hashes = [...hashes, payload.action.hashed.hash];
  });
});

async function fetchFloatRoundingEntries() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'test_float',
      zome_name: 'float_rounding',
      fn_name: 'get_all_float_rounding_entries',
      payload: null,
    });
    hashes = records.map(r => r.signed_action.hashed.hash);
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
<span>Error fetching the float rounding entries: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No float rounding entries found.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes as hash}
    <div style="margin-bottom: 8px;">
      <FloatRoundingEntryDetail floatRoundingEntryHash={hash}  on:float-rounding-entry-deleted={() => fetchFloatRoundingEntries()}></FloatRoundingEntryDetail>
    </div>
  {/each}
</div>
{/if}

