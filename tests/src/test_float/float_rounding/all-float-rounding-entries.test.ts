import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource,  fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import {createDummyEntry, createFloatRoundingEntry} from './common.js';

test('create a FloatRoundingEntry and get all float rounding entries', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/test_hc.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Bob gets all float rounding entries
    let collectionOutput: Record[] = await bob.cells[0].callZome({
      zome_name: "float_rounding",
      fn_name: "get_all_float_rounding_entries",
      payload: null
    });
    assert.equal(collectionOutput.length, 0);

    // Alice creates a DummyEntry that has no must_get_agent_activity validation to workaround
    // https://github.com/holochain/holochain/issues/2421
    const createdRecord0: Record = await createDummyEntry(alice.cells[0]);
    assert.ok(createdRecord0);

    // Alice creates a FloatRoundingEntry
    const createdRecord: Record = await createFloatRoundingEntry(alice.cells[0]);
    assert.ok(createdRecord);
    
    await pause(1200);
    
    // Bob gets all float rounding entries again
    collectionOutput = await bob.cells[0].callZome({
      zome_name: "float_rounding",
      fn_name: "get_all_float_rounding_entries",
      payload: null
    });
    assert.equal(collectionOutput.length, 1);
    assert.deepEqual(createdRecord, collectionOutput[0]);

    // Alice creates another FloatRoundingEntry and receives an agent activity validation error
    try {
      await createFloatRoundingEntry(alice.cells[0]);
      assert(false, "Should not reach");
    } catch (error) {
      assert(error.data.data.includes("A FloatRoundingEntry created by this agent already exists"));
    }

    await pause(1200);
  });
});

