# CT77 scoped research receipt — 2026-11-13 [pcc:unverified]

## What was done (commands, files, features)
- Reviewed root PR #3877, CT55/75/76 and stack #4015, today's session log, AGENTS, sibling cryptanalysis/autoresearcher work and standard Weierstrass/Frobenius sources. KB/Conductor/Lean not loaded in this runtime.
- Independently enumerated all 32 binary coefficient tuples (16 good, 16 singular) and all five attainable Frobenius traces. Checked CRT integer lifts preserving the source prime reduction and nonsingularity at p and 2. Computed exact minimal torsion field degrees ell=3/5/7/11/13 and direct GF8/GF32 point counts for winning ell=7/11/13 models.
- Python discriminants and binary-field point enumeration versus JavaScript derivative tests and alternate field bases agree in normal/optimized modes, 9 JSONL records SHA256 fd91d33f5b5b845a7255c92a79edc7144661600ff36e360aa924a7a440e61006.

## Decisions made (and why)
- CT76's fixed-model target bounds do not apply to other CRT choices; minimize f_some across all binary reductions but do not confuse the existence of some rational torsion with mapping an arbitrary oriented source generator. No EKS/GPU spend or native genus code edits.

## Open threads (unfinished, pick up next time)
- Confirm exact-head scoped CI, independent signed PCC and stack review. Construct and cost explicit global recognition/second-prime specialization for both arbitrary public inputs.

## Errors (what went wrong, how it was handled)
- Python/JS histogram initially had different JSON object key-order; replaced with sorted numeric pairs and reran all four modes. No arithmetic mismatch. Lean/Kb/Conductor unavailable.
