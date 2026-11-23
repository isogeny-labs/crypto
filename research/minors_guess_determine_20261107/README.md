# Minors / guess-and-determine ECDLP: charged comparison against matched rho (slug `minors_gd`)

All numbers below are `[pcc:unverified]` (no pcc claim has been filed; nothing here has been
through the claim ledger, an ecbench session, replay certification or the validation gate).
Evidence classes are given per item: cited (from a paper), measured-dev (a development run on this
box, not evidence), derived (our arithmetic), heuristic, or planned.

## Goal

Atlas lead `LEAD-EXO-minors-guess-and-determine` (priority 3a): turn the published Mahalanobis line
(arXiv:1703.07544, 2005.05039, 2310.04132, 2607.09814) into a charged, falsifiable comparison with
matched negation rho at 20-40-bit group orders, on the same planted instances, every minor,
determinant, kernel and linear-algebra step charged in one counted unit, with a scaling-exponent
interval. Deliver the frozen design, the implementation, the correctness checks and a ready-to-run
panel; do not run the panel here.

## Sources checked

* The four papers, read in full (PDFs and `pdftotext` extractions in `sources/`, none blocked):
  `sources/NOTES.md` has the algorithm, stated complexity, claimed experiments and page/section locators
  per paper, and says where the papers are vague. Main finding: the earlier draft's description
  ("zero n x n minor of a Riemann-Roch evaluation matrix with n-1 guessed columns and a cofactor scan")
  is not what the papers say; the actual objects are an `l x 2l` left kernel `K` (`l = 3n'`), a zero
  minor of its dense `l x l` part, and, in the 2026 paper, a guess of `l/2` columns followed by a hashed
  search of penultimate hyperplane intersections with a defect `d` (details in `NOTES.md`).
* The 2026 paper itself states "no better than exhaustive search" for the parameters that give constant
  success probability (sec. 1.1, Table 2); the 2020/2023 papers report "kernels", never total work,
  and the 2020 paper calls the growth of its own Schur-complement data exponential (sec. 4.1). The
  order-2^50 toy solve belongs to the 2020 paper, in binary fields (cited, not re-measured).
* research-kb: not available here (per the shared rules), so no `source_id`s were recorded.
* Atlas tooling: `python3 -I tools/lead_atlas.py --match "minors guess determine"` and
  `tools/build_frontier_map.py --match "minors"` in the crypto-autoresearcher checkout both failed
  with `ModuleNotFoundError: No module named 'orchestration'` (sparse checkout; also with
  `PYTHONPATH=.`), so there are no atlas match hits. The lead file itself and
  `docs/reports/ecdlp-special-structure-leads-20261031/literature.md` were read (it lists the lead as
  "NOT in atlas", abstract-level, expects the search to be worse than rho). The design draft named in the
  lead (`coordination/design/.../minors-guess-and-determine/design.md`) is not in the sparse checkout.
* Conductor is not installed; no task was attached (noted, not installed).

## What was done

1. Fetched and read the four papers; wrote `sources/NOTES.md`.
2. Froze `PROTOCOL.md` (arms, counted unit, instances, seeds, sizes, caps, estimators, decision rule,
   controls, declared limits) before any panel record; hashes in `FROZEN_SHA256.txt`.
3. Implemented natively in Rust, standalone, no dependencies: `minors_gd/` (`field.rs` counted `F_p`;
   `curve.rs` affine arithmetic, BSGS point counting, planted prime-order instances; `rho.rs` batched
   negation rho; `kernel.rs` the reduction (`C = M2 M1^-1` on the Riemann-Roch basis);
   `scan.rs` `mgd.all2`, `mgd.gesc`, `mgd.apm2`; `gd.rs` the guess-and-determine arm; `main.rs` CLI,
   self-test, controls, panel runner). The unit is `F_p` multiplications, inversion priced at
   `1.5*bits(p)` mults, additions and hashing counted but uncharged (section 3 of the protocol).
   ecbench was not extended: the ecbench-extend route would change shared files (`methods.rs` and the
   IC candidate identity), needs the whole 70k-line crate built on a shared 4-core box with ~9 GB free,
   and the prime-field `GF(p)` path is a `CountedGroup` that cannot charge a minors/linear-algebra
   method in `F_p` multiplications anyway; the standalone count prices rho in the same unit instead.
4. Correctness checks (development, local, seconds): `minors_gd selftest` (log in `dev/selftest.txt`):
   field arithmetic against `u128`; 15 planted instances (8-22 bits) checked (`#E` prime and `n G = O`,
   `Q = kG`, on curve); every arm recovers the planted `k` on 6/6 instances at 8-12 bits (rho also 16, 20)
   with the explicit point sum of every reported minor checked to be `O` (`--dev-check`); the verifier
   rejects `k+1`. Control `minors_gd controls` (`dev/controls_uniform_vs_curve.txt`): zero `2x2`-minor
   counts of the curve-derived `C` against i.i.d. uniform matrices and the uniform expectation `C(l,2)^2/(p-1)`.
5. Development calibration runs (labelled not-a-result) to set caps and the `d` knob, on instance indices
   disjoint from the panel (`idx` 0-96, 200-230, 300-307, 1000-1005, 5000-5039; panel is `10000-10199`).
   `PROTOCOL.md` (frozen, hashed) says development used `idx <= 96`; the later rho re-tune and the
   final-binary table used 200-230 and 300-307. All are below 10000, so the panel instances were never seen;
   the protocol text was not edited afterwards because it is hashed.

## Results (all development observations, NOT results; no panel was run)

* Correctness (measured-dev): every arm solves every self-test instance with the planted `k`; the
  support of every reported minor sums to `O`; zero 2x2-minor counts of the curve-derived `C` at 12/14/16
  bits are 3625 / 2121 / 1051 against 3671 / 2089 / 999 for i.i.d. uniform matrices and uniform
  expectations 3624 / 2091 / 1039. So in prime fields at these sizes `C` shows no structure beyond the
  uniform "each tested minor is zero with probability about `1/p`" model (supports the cost model below;
  does not rule out structure at larger `l`).
* Mean charged cost per solve, development run `dev/dev_final_binary_idx300-307.jsonl` (8 instances per
  size, caps 1e8/5e8/2e9/2e9, Garwood/bootstrap intervals in `dev/dev_final_binary_analysis_NOT_A_RESULT.*`;
  `>` = no success, lower bound), units: charged `F_p` mults:

  | bits | rho | mgd.all2 | mgd.gesc | mgd.apm2 | gd | gd_dp1 | gd_half |
  |---|---|---|---|---|---|---|---|
  | 20 | 1.15e5 | 4.7e5 | 4.8e5 | 5.0e5 | 4.7e6 | 2.1e6 | 7.1e6 |
  | 24 | 1.9e5 | 1.9e6 | 1.2e6 | 1.2e6 | 9.0e7 | 2.9e7 | 1.7e8 |
  | 28 | 3.6e5 | 3.3e7 | 1.2e7 | 7.5e6 | 1.6e9 | 5.5e8 | 1.7e9 |
  | 32 | 8.1e5 | 1.4e8 | 1.7e8 | 8.3e7 | 4.7e9 | 4.4e9 | >5.3e9 |

  Ratio to rho (point estimates): the best minors arm goes from about 4x at 20 bits to about 100x at
  32 bits; the guess-and-determine family from about 20-40x to about 5000x. The A/A control
  (`rho_aa/rho`) is 1.00, 1.08, 0.96, 1.45 with intervals about +-15% to +-30% at 8 instances (200 in the
  panel), i.e. the development noise floor is large and these ratios are read at order of magnitude.
  Dev exponent fits (4 sizes, 8 instances) are 0.62-0.72 for `mgd.*` (all sizes) and 0.77-0.90 for their
  top three sizes, 0.85-0.93 for `gd`/`gd_dp1`, 0.23-0.26 for rho (depressed by rho's fixed set-up at
  20-28 bits). Interpretation: heuristic, consistent with "cost per solution is about `p / (minors tested
  per unit work)`", where minors per unit work is a polynomial in `log p` (hash-based scanning tests about
  `l` to `l/d` minors per operation, never `sqrt(p)`).
* The panel has not been run. **Whether the lead closes is therefore open**; the protocol's mechanical
  rule (section 8) is the only thing that may close it, and only for this reconstruction. If the panel
  behaves as the development runs, the extrapolated gap at 40 bits is about three orders of magnitude for
  the cheapest arm and five for `gd` (derived/heuristic, from development per-attempt costs and the
  uniform-minor model); the panel is what confirms or refutes that.
* Development observation on the question of step 4 of the task: yes, the papers' algorithm is clearly
  worse than rho in the counted unit already at 20-32 bits (4x to 5000x depending on arm), and the gap
  widens with size in every run. That is a development observation, not a result. What the frozen panel
  needs to confirm it: the sizes 20-40 in `PROTOCOL.md` section 5 with 200 matched instances, the
  exponent fits with bootstrap intervals, the A/A noise floor and the inversion/unpriced-work sensitivity
  bands, and (outside this directory) an independent replay and an ecbench `rho.negation` cross-check.
* Observations worth keeping: (i) up to about 28-32 bits a single kernel (one attempt) usually contains a
  zero `2x2` minor because `C(l,2)^2 >= p` there, so the cost is just the polynomial cost of building and
  scanning one `l x l` matrix and is within a factor of a few to tens of rho; beyond 32 bits the
  expected number of zero minors per matrix falls below 1 and cost per solution grows like `p`; this is
  the same transition the 2020 paper sees in "kernels" (113 -> 4406 for 40 -> 46 bits). (ii) the smallest `d`
  of the paper's Eq. (4) (`gd`) is not the cheapest: `d+1` (`gd_dp1`) is 2-3x cheaper at 20-28 bits; both
  are in the panel. (iii) the three-deviation APM claim ("one kernel up to 47 bits") is, by the paper's own
  `O(l^7)` per kernel, about `1e15` operations at 47 bits against rho's about `1e8`; "kernels" is not a cost.

## Decisions made (and why)

* Standalone native count instead of an ecbench method: see What was done 3. Rho is charged in the same
  unit on the same instances; the panel is not an ecbench session and so cannot feed the leaderboard.
* Riemann-Roch basis instead of the papers' complete degree-`n'` monomials: identical left kernel, hence
  identical `A` and minors, cheaper build; this favours the method.
* Slope/line hashing for `2x2`/`(d-1)`-subset scans instead of naive determinants: cheapest correct
  reading; the naive count is recorded (`naive_minor_evals`) but not charged.
* Prime fields only; `n' = bits`; symmetric `l` P-rows and `l` Q-rows; distinct points enforced (a P/Q
  coincidence is an accident the papers say to ignore, and without redrawing it made 8-12-bit self-tests
  produce false relations).
* Censored-MLE per size (work per success) rather than forcing every arm to finish: the beyond-32-bit
  arms cannot finish; zero-success sizes give lower bounds and drop out of slope fits.
* Declared gap: no `apm3`; a three-deviation scan is about `1e12` operations per attempt at 40 bits.
* Rho re-tuned (R = 32, 16 walkers, one-multiplication restarts) before freezing because the first
  version spent most of its cost on set-up at 20 bits (a strawman). Dev rho: 1.1e5 (20 bits), 1.3e7 (40).

## Open threads

* Run the panel (below). Then: read `analysis.json` per the protocol, file pcc claims, cross-check `rho`
  against ecbench `rho.negation` GAE on the same curves (`minors_gd gen --bits B --idx I` prints
  `p, a, b, n, G, Q, k`; ecbench GAE x `(3 + 1.5*bits)` converts to charged mults), consider an
  independent replay of the minors arms (a second implementation), and decide whether `apm3` or the
  binary-field setting of the papers' own tables deserves a separate lead.
* `d` and `n'` are knobs of the method; the panel covers three settings, not the optimum.
* If the arms at 36-40 bits give zero or few successes for every minors arm the exponent comes from
  lower bounds only (the script reports them); a longer run with larger caps would tighten it.
* Validation gate: nothing here has a replay certificate, validator report or `VALIDATION.json`.

## Errors

* First `cargo build` waited on the shared target-dir lock held by other agents' large builds and was
  killed at the tool timeout; rebuilt with a private `target-local/` (788 KB, git-ignored) instead of the
  shared target directory (the crate has no dependencies).
* Instance generator looped forever at 8 bits (empty prime window); window margin fixed.
* Self-test first showed false minor relations at 8-12 bits (colliding P/Q points); fixed by enforcing
  distinct points; then every reported support sums to `O`.
* `controls` first returned early on matrices with a zero entry, biasing the 2x2 counts low; fixed.
* A 40-bit `gd` run with `d = 8` tried to allocate 16 GB (leaf table); `d >= 8` is now rejected;
  panel `d` is at most 6.
* Atlas matching tools failed (missing module); research-kb unavailable (noted above).
* Nothing was lost or retried silently; no git state was changed.

## Ready-to-run

Nothing was launched. Artifacts (all in this directory):

* `minors_gd/` (Rust, no dependencies): `cd minors_gd && cargo build --release --offline` ->
  `target/release/minors_gd`. Source and tool hashes: `FROZEN_SHA256.txt` (binary of this box included;
  rebuild on the executor and record its own SHA-256).
* `run_panel.sh BIN OUTDIR JOBS` runs the frozen panel (48 shards of 25 instances, 8 arms, 6 sizes),
  merges, hashes (`SHA256SUMS`) and runs `minors_gd_analyze.py` (output `analysis.json`).
* `minors_gd_analyze.py` (stdlib Python; run with `python3 -I`) is the pre-specified estimator and
  mechanical decision rule; tested on development files only.
* Estimated cost: about `7e12` charged units, 25-55 core-hours at the development throughput of
  4-8e7 units/core-second (derived/heuristic); under 4 h on 16 vCPU; memory under 1 GB per process
  (gd_dp1/gd_half at 36-40 bits about 0.5 GB). The counts are deterministic and host-independent, so the executor's
  isolation grade only affects the informational wall times.
* NOT done (the main session must do before launch): Conductor attach; `scripts/checkpoint.sh init`
  (`.resume/RESUME.md`, required before any run); `tools/benchrec.py init|run|add-raw|metric|close` if
  the panel is to be treated as a benchmark record; an OCI image or executor that carries the binary
  and a `isolab.eks.request/v1` (or `isolab_k8s_run` argv) for it plus `remote/validate-request`;
  choosing the route (this is a count, not a timing, so the ecbench L2/A-A admission does not apply,
  but the isolab-eks default and the experiment-director card still do); the commit/push of source and
  protocol that `check-experiment-git` requires. No request JSON was written because the repository's
  validator needs an image digest and source commit that do not exist yet.
* Exact command on the executor, after building the binary:
  `./run_panel.sh minors_gd/target/release/minors_gd OUT 16 && python3 -I minors_gd_analyze.py OUT/records.jsonl --label panel --boot 5000`

## Shared files edited

None. New files only, all under `research/minors_guess_determine_20261107/`. (Build artifacts in
`minors_gd/target-local/` are git-ignored via `minors_gd/.gitignore`.)
