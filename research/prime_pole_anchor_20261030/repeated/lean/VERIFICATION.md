# Executed verification

2026-11-04, exit0 for the successful commands below. Work from this Lake
directory, with `/workspace/audit/toolchains/lean-4.35.0-linux/bin` on PATH.

```
lake build
lake env leanchecker --fresh --verbose AnchorBudget
```

The final build is build-004.log. Official import-inclusive fresh replay
finished successfully; fresh.log names AnchorBudget. This is leanchecker,
not a claim of running lean4checker. Three named user theorems pass.

From the actual repository root, after staging the actual source:

```
bash tools/lean/proof-gate.sh --out /workspace/audit/anchor-budget-gate \
  research/prime_pole_anchor_20261030/repeated/lean
```

Exit0, PASS. The transitive audit checks 4 declarations: the 3 named user
theorems plus the generated Feasible.brecOn declaration. There are 0 failing
and 0 native-trusted declarations. Accepted foundations are propext,
Classical.choice and Quot.sound. Actual gate receipts are copied in gate/.
The gate's String.mk deprecation warning is retained and does not invalidate
the proof. The unsuccessful build003 and its sorryAx diagnostic remain
failed evidence; they are not counted as a proof.

Native strict compile and two executions succeeded, including an independent
rebuild with check.sh into /workspace/audit/anchor-budget-independent-replay.
Both outputs match replay.log exactly. The executable hash is recorded in
../binary.sha256. This is a separate rebuild and replay of the same checker,
not an independently implemented mathematical proof or an ECDLP solve.

No Rust source, manifest, lockfile or library input is changed. Those inputs
still match aaab886ec25b8cd5a54c97ca75c916b5f7dbb0ef. The existing affected
Rust receipt (429 passing tests and 1 default ignored across two disjoint
filters) is reused with its exact scope, not described as a new full-suite
run; see ../../../prime_common_kernel_20261030/receipts/ROOT-VALIDATION.md.
