# Local ell=8 cap continuation

2026-10-25. Pre-register this continuation before its native outputs. The
earlier 140,000-column refusals remain inconclusive and are preserved. The
completed balanced k=3 draw-0 cells used at most 6,969,224 KiB RSS under
the existing 12-GiB address-space limit. This observed resource use motivates
a bounded feasibility probe; it does not establish that larger cells fit.

Use the identical frozen native instrument (built from d67f6085; its later
source-only parity lint change is not in that binary), unchanged equations,
original byte-identical seven fixtures, balanced variable ordering and
degree-four dense closure. Raise only the outer column admission cap to
200,000. Keep internal limits at 1,000,000 columns and 50,000,000 rows,
12 GiB address space, 900 seconds per branch, four Rayon threads. This
shared host is L0; elapsed/CPU/RSS are diagnostics, not physical speed claims.
Only one large measurement process runs at a time.

First probe k=0 on original UNSAT draw 6. Preserve all outcomes. Then execute
the remaining k=0/1/2 cells on draws 0,3,5,6, the remaining k=3 cells on draws
3,5,6, and k=4 on all four draws. Counts are 28 k<=2 cells, 24 new k=3 cells,
and 64 k=4 cells: 116 branch attempts including the probe. Retain original
completed k=3 draw-0 results separately, without rerunning or pooling their
wall times. A failing process does not cancel the remaining registered cells.

Guess sets: none; [7]; [7,15]; [7,15,23]; [7,15,23,24]. Every branch has a
separate command, input/binary/protocol hashes, timestamps, resource receipt,
stdout/stderr and exit status. A nonzero exit with no JSON is a failed or
censored attempt, never a non-refutation. A saturated verdict applies to
this exact fixed-degree system and guess set, not every subset of that size.
All branch work is charged; no positive branch is selected post hoc.

Draws 4 and 5 do not lift on the original E0 curve (separate finite audit).
Draw 5 stays in the original UNSAT panel as a Boolean-system control; this
continuation makes no claim that every input is an ordinary subgroup query.
Curve/subgroup relevance and the fresh-query follow-up remain open.

The runner is run-guess-cap200k.sh. Freeze its source before execution; copy
and hash the existing binary to an immutable local path and pass it using
IC_FROZEN_BIN. It waits for the already running quartic/HKY queue to finish,
then runs sequentially. This is a local queued process only when actually
started. No paid VM or cloud spend is authorized beyond the existing $100
aggregate cap. Cloud execution, if needed, still requires ephemeral access.

Retain the complete raw tree in a new deterministic archive, independently
recompute counts and costs, and produce an inspected source-linked visual
report. Do not rewrite the earlier archive or infer a global minimum from
these balanced sets. Fresh ell=9/four-summand cells and alternative sets
remain distinct registered requirements.
