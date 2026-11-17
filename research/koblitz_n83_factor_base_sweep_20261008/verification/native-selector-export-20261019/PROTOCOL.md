# Native free-selector model export follow-up

Frozen before execution. The preceding boundary diagnostic is preserved:
the original worker returned SAT_verified_witness_x at 787 conflicts,
35307 decisions, 167735.758707 ms from solve entry and 167817.248441 ms
worker wall. Initial propagation completed at188.398658 ms; returned
timers report167575146396 ns XOR propagation and71556732 ns clause
propagation. Outer wall168213.674931 ms, exit0, uncensored, cleanup verified.
Its card stopped because the frozen native auditor deliberately rejected
a free-selector positive terminal without independently decoded selectors.
That missing audit is not retroactively filled or relabeled a pass.

This follow-up adds only post-solve selector export in a separate example,
`koblitz_n83_free_selector_export_control`: same original fixture, domains,
eight pinned x blocks, first-pair order, m5 identity mask0 and all original
clauses. It reads exactly-one representative and seven phase bits from the
actual SAT model after solve returns. It adds no representative or phase
units, no preprocessing inference, no branch-order change and no solver
change. Layout is checked against the frozen native domain allocation and
size formula; tests reject absent/multiple selections, phase83, zero wires
and out-of-model bounds. The prior source/control remains unchanged.

Repeat this one free-selector workload once under the same5,000-conflict,
180-second,1CPU/4GiB/zero-swap/networkless Docker envelope, same exact public
inputs and immutable image541096ae9b05d3b850341b78790e2ed31e94d5aa0ea086933c6ff0c09ffa2e63.
Added allocation180 seconds, separate from prior360-second allocation,
build/test/audit/render/storage overhead and all earlier spent work. No
automatic retry or changed cap. Native run command deadline240 seconds,
success exit0, fresh write-once path under already ignored
`../native-phase-20261019-v2/exec/v5/free`.

The previously passed fully pinned control supplies the separate encoding
gate: source5e9ee5d4b1aa6ca22368213e5bc05b343e00bfb4, verified SAT0conflicts,
actual model/group/domain replay, exact relay and stream custody confirmed.
Its initial native relay checker failed because JSON float reserialization
changed spelling. Preserve that failed check and impeded assessment. The
corrected native checker reads original receipt bytes after validating card
identity; its15tests include the exact original float. A separate corrected
assessment passes the original bytes and all output digests, without a
rerun. Carry that assessment and pinned audit as bound gate inputs.

Native independent arithmetic now verifies each actual decoded representative
index/phase against the full exact domain, the pinned summand x, and the
original five-point group sum. Require a complete Sat trace with matching
terminal counters, frozen conflict cap and eight fixed x blocks. Retain
positive model, stderr trace, specs, binary/source/input inventories,
resources, custody, final inspection and cleanup. Coherently rehashed phase
forgery must be rejected. A missing export, invalid model, missing replay,
wall censoring, conflict cap or producer error keeps the original control
gate unresolved. Record the precise outcome without automatic rerun.

Use the required director → mechanical → director relay with native cards
only, exact clean HEAD and all argv/input/binary/protocol hashes. No research
Python. No other task builds/tests during the control. L0 descriptive wall,
n=1; no speedup against the older censored run, no fastest-base or cold-cost
claim. Root-library source bytes remain identical to the full serial suite
that just passed4397tests plus19exporter tests and1selector test. Validate
the new example and native runner before pushing; do not rerun unchanged
root-library tests merely for a post-solve example/exporter addition.

Keep this new run, preceding stopped native audit, original180s censoring,
and all previous natural zero-yield observations separate. Publish a
source-linked report/diagram/graph with inspected PDF and benchmark copies.
Full-cold scoreboards remain unchanged. Passing this control establishes
only this original known-witness free-selector workload; natural yield,
rank, broad parameter sweep and complete-runtime selection remain open.
