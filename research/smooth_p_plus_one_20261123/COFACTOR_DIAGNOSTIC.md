# Retain the failing cofactor RUR before repair

2026-11-24. Separate diagnostic after the failed first msolve panel.

The first frozen F127 panel (source `5ea8e6940e`) exited 1: its first target
returned all nine reference rows; target (116,69) returned seven against ten.
That second result's exact sets and stderr are retained, but the old CLI
returned before emitting its input/RUR record. The remaining eleven planned
queries are unrun. [pcc:unverified]

Change only mismatch-record emission: print status `mismatch`, both signed
sets and the existing input/RUR diagnostics before the same nonzero return.
Freeze and publish a separate executable, then execute the F127 depth-four
cofactor-msolve panel once with the original three-second query deadline,
single worker, 2 GiB address-space ceiling and 60-second outer timeout. The
same fail-fast policy stops it at the first disagreement. No equation,
variable order, solver flag, acceptance condition or cap changes in this
diagnostic invocation.

This is a separately recorded diagnostic replay, not a successful rerun of
the initial panel. Keep both outcomes. If the mismatch recurs, inspect the
RUR dimension, parameter polynomial, variable names and projected coordinate
before choosing a repair. If it does not recur, retain that fact and investigate
solver nondeterminism without promoting either run to a performance result.

The earlier expanded-formulation record on this same target contains ten
accepted rows; see `evidence/native_f127_panel.jsonl`. It supplies a prior
exact-output control, not an explanation for the new discrepancy.
No further qualification or recovery run is part of this diagnostic.
