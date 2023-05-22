import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeDnaHash, fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createFloatRoundingEntry, sampleFloatRoundingEntry } from './common.js';

test('create FloatRoundingEntry', async () => {
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

    // Alice creates a FloatRoundingEntry
    const record: Record = await createFloatRoundingEntry(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read FloatRoundingEntry', async () => {
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

    const sample = await sampleFloatRoundingEntry(alice.cells[0]);

    // Alice creates a FloatRoundingEntry
    const record: Record = await createFloatRoundingEntry(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created FloatRoundingEntry
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "float_rounding",
      fn_name: "get_float_rounding_entry",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);

    // Bob creates another FloatRoundingEntry which has a 2 digits float value
    const sample2 = await sampleFloatRoundingEntry(bob.cells[0], { value: 11.22 });
    const record2: Record = await createFloatRoundingEntry(bob.cells[0], sample2);
    assert.ok(record2);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created FloatRoundingEntry
    const createReadOutput2: Record = await bob.cells[0].callZome({
      zome_name: "float_rounding",
      fn_name: "get_float_rounding_entry",
      payload: record2.signed_action.hashed.hash,
    });
    assert.deepEqual(sample2, decode((createReadOutput2.entry as any).Present.entry) as any);
  });
});


