# V3 serial launch sequence

This is preparation only. The branch-coefficient payload is active; no other payload may overlap it.

1. Reconcile the branch terminal receipt and coordinator ledger. Its proposed 3,600-second charge would bring the campaign from 7,140 to 10,740 seconds; this is conditional until recorded.
2. Publish/read back the parser qualification and v3 snapshot sources. Run the independently prepared 60-second parser qualification once, serially. Use the real Python 3.9 parser and heartbeat path. No new compiler or native log fixture is requested.
3. On the qualified source and independently reviewed cache plan, run snapshot-cache.py once in a native 60-second envelope with one CPU and 1 GiB. Require terminal cleanup and its successful result. Preserve every snapshot output and failure. Do not run the compiler if the snapshot fails.
4. Retain the original c5178 materialized-inputs manifest via existing artifact custody; bind the produced current-cache.json, snapshot result/terminal, parser result/terminal, independent review, live lease and current ledger. Publish large inputs by manifest/URI if required; do not regenerate immutable input evidence.
5. Populate final admission.json from admission-draft.json. Recompute recipe_files for actual inputs and receipts; bind current_cache_sha256, verified publication and actual current campaign allowance. The anticipated balance after a charged branch plus both 60-second steps is 32,340 seconds, with 30,540 unreserved after reserving the 1,800-second validation. These figures are conditional, not a current ledger update.
6. Independently review final bindings, publish/read back exact final files, confirm no active payload and fresh task lease, then create the unique caller control directory from launch-draft.json and execute its exact argv. Retain stdout, stderr, outer exit and native terminal receipts. No automatic retry.

The exact future validation argv is:

```text
/usr/bin/timeout --signal=TERM --kill-after=5s 1795s /usr/bin/python3 -B /home/ec2-user/crypto-pencil-evidence-recovery-20261105/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/root-library-validation-v3-20261109/durable-launcher.py
```

Admission remains disabled. The build gate remains `cargo test --release --lib --locked --offline --no-run --message-format=json`, followed by inventory, the original four ordered exact controls and the full unfiltered library suite. Resource limits remain two CPUs, 16 GiB, no swap and 1,800 total seconds. The deadline remains 2026-11-10 05:11:39 UTC. No additional test selection or scientific collection is introduced.
