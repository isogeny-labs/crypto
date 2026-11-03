# Frozen N83 K=16,627 root-propagation discriminator

**Status:** frozen by the protocol commit on this follow-on branch, before solver edits or execution. It follows the `UNKNOWN_wall_cap` complete-witness arm in `research/n83_k16627_m4_witness_20261018/`; preserve that arm unchanged.

## Question and frozen input

The seven-block complete witness imported the exact K=16,627 signed-Frobenius base, passed group preflight, installed the full four-summand orbit domain and entered `Solver::solve()` at 40.267 seconds, but its 1,800-second outer cap ended without a solver return. Determine whether the wall is spent in the initial root `propagate()` call or later CDCL search. Do not infer either phase from CPU use or a missing conflict count.

Use the same primary curve `icv1-f2m83-tm6151469093347-debefd74`, `public_x_hash` base with seed `2026100801`, compressed object BLAKE3 `1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d` and point-set BLAKE3 `564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0`. Use signed-orbit columns 0, 1, 2 and 3, stored indices `[0,166,332,498]`, their two native intermediate sums and target from the prior phase record; independently recompute them from the retained object. Require affine partial sums, distinct four summand x coordinates, checked rank one and rejected `R+G` tamper before building SAT. A mismatch is `INVALID_INPUT_OR_PREFLIGHT`, with no replacement target.

## Instrumentation and controls

Install the same full K=16,627 orbit-domain encoding with at most 250,000 variables and 6,000,000 domain clauses, identity mask zero and no pair ordering. Check/pin the seven x blocks via the named encoder API, with the target block checked against the builder's fixed value. Require the same model counts before root propagation: 201,598 variables, 5,915,238 clauses and 3,901 XOR rows. Any mismatch is `INVALID_MODEL_CONSTRUCTION`.

Expose a native `Solver::propagate_root()` operation that calls the current root `propagate()` once, preserves all solver state for a subsequent `solve()`, and reports fixpoint or conflict. The worker writes and `fsync`s append-only records `root_propagation_entered` immediately before the call and `root_propagation_returned` immediately after it, with elapsed wall, cgroup current memory, source/object digests and snapshots of `SolverStats` (especially clause visits, XOR passes, repivots and row operations). The worker must not emit `root_propagation_returned` before the method actually returns. A root conflict against the constructed complete witness is `INVALID_UNSAT_PLANTED`.

Before this degree-83 run, use small native SAT/field fixtures to check that `propagate_root(); solve()` and direct `solve()` agree on SAT/UNSAT and that returned SAT models pass the original-field and group replay. The new method must not silently clear or rebuild solver state. On a root fixpoint, enter ordinary `solve()` with the same 100,000-conflict, one-model and 1,800-second outer caps; sync `sat_entered`, `sat_returned`, group lift and rank records as in the prior control. A passing positive result still requires SAT, original-field verification, an exact seven-block x match, group relation, rank one and `R+G` tamper rejection. No unpinned arm starts without that positive result.

## Envelope and decisions

Build offline from a clean source commit in the pinned arm64 `rust:1.93.1-bookworm` image. Check source/input hashes, including the modified SAT solver source, new solver and touched exporter tests, then run one CPU, a hard 4 GiB cgroup, zero swap, no network, read-only root and a fresh output directory. Preserve exact commands, image digest, source/binary hashes, raw phase JSONL, stdout/stderr, container exit/OOM/peak and UTC interval. The outer wall cap is 1,800 seconds from process start; a missing worker result remains `UNKNOWN_wall_cap` with unknown conflicts/models unless a completed checkpoint explicitly states them. Keep prior construction and capacity costs separate and do not form a complete IC total or speed ratio.

If `root_propagation_returned` is absent at the wall cap, the next control instruments progress within CNF and XOR root propagation. If root propagation completes and the later solver call caps, the next control adds completed conflict checkpoints or changes the search policy under a new protocol. If a checked SAT relation returns, run only the separately frozen unpinned/natural controls and then a matched useful-rank/cold-cost comparison. This experiment alone selects no factor base and changes no IC scoreboard row.
