# Published-IC reference arm for prime fields (prep, 2026-10-29)

Track of the prime IC program prep workflow. This is **preparation, not a
measurement**: no code was written or run and no experiment was launched.
Every number below is either *measured* (with the file it came from),
*derived* (computed here from measured values, with the formula) or *plan*.

Files in this directory:

| file | what it is |
|:--|:--|
| `README.md` | this note: the recommendation, the resource estimate, the route, the owner decisions |
| [`PROTOCOL.md`](PROTOCOL.md) | the draft protocol in prose: arms, panels, seeds, band, gates, decision rules |
| [`protocol.draft.json`](protocol.draft.json) | the same as an `experiment.protocol/v1` record, `freeze.status = "draft"`; it passes `tools/experiment_protocol.py validate` (crypto-autoresearcher) |
| [`IMPLEMENTATION.md`](IMPLEMENTATION.md) | file-by-file implementation spec, the msolve subprocess contract, and how msolve's work is counted and priced |

> **Status of the owner decisions (cross-track review, 2026-10-29).** After this
> note was written, [`../DECISIONS.md`](../DECISIONS.md) recorded: **D1** yes,
> tentatively (`pub-m2` is the counted algebraic reference); **D2** yes (the
> Table B incumbent may be ranked against it); **D3** allowed, delegated (the
> pinned counter patch); **D4** approved (`D-EKS`, about 49 CPU-hours, range
> 15–150). **R1** classifies `rootfind-*`, `mitm` and the Table B pair tables
> as a counted *generic* track (Option C of `../ruling/README.md`). So `inc`
> and `inc-m3` are generic-track arms, not "table, reference only" or
> "unclassified" as §4 and `PROTOCOL.md` still say. One conflict remains open:
> Option C's text says the two tracks are "never ranked against each other",
> but D2 says to rank them. Decision rule 3 (§4; `PROTOCOL.md` C3;
> `protocol.draft.json` line 46) has been left as "context only" on purpose.
> The owner, or the main session, has to settle which of the two wins before
> the freeze. D5 is still open. None of this changes what is measured.

## 1. The recommendation in one paragraph

**Reference configuration.** Use Semaev's naive prime-field index calculus as
the published reference arm:

- the small-abscissa factor base `{P : x(P) ∈ V}`, where V holds the N
  smallest abscissae that carry a point. This is the existing
  `prime-abscissa:size=N`, the same base the Table B incumbent uses;
- m = 2 (and m = 3 at small sizes);
- the full system `⟨S_(m+1)(x_1, …, x_m, x_R), F_V(x_1), …, F_V(x_m)⟩` with
  `F_V = ∏_{c∈V}(x − c)`, solved per trial point by **msolve**'s F4 over F_p
  (`-P 1`, rational parametrisation);
- cold whole-solve S in GAE, with msolve's F_p multiply-adds priced in the
  AGENTS.md §5a band at 0, 0.1 and 1 GAE each.

**Second published arm (phase 2).** Petit–Kosters–Messeng's tower base, using
the existing `pkm-semaev` oracle on `prime-kummer` with an msolve engine.

**Symmetrisation.** FGHR-style symmetrisation over F_p is a *derived*
extension, not a published configuration here.

**Implementation.** One new oracle (`semaev-naive`), an F_p solver plug point
that does not exist yet (`FpSystemSolver`, as designed in the PKM note §4.1),
and a pinned, statically linked, counter-patched msolve in the ecbench EKS
image. Route: isolab-eks after merge.

## 2. Why this configuration is the right reference

1. **It is the published prime-field configuration, and its complexity is on
   record.** The repository's notes cite Semaev (ePrint 2004/031) for the
   small-abscissa base and `S_(m+1)`. They cite Yokoyama, Yasuda, Takahashi
   and Kogure (J. Math. Cryptol. 14 (2020) 460–485) for a lower bound on
   exactly this ideal, with the conclusion that the naive method cannot beat
   rho or even brute force. Sources:
   `research/notes/ecdlp-general/RESEARCH_ECDLP_STATE_OF_THE_ART.md` §8.2 and
   §8.5, and the paraphrase in `research/ecdlp_autolab/yokoyama_lower_bound.md`.
   That paraphrase was not checked against the paper's full text, as
   `research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md` §1.3 already
   notes.
2. **It meets the owner's 2026-10-24 criterion as written.** Relations come
   from solving a summation-polynomial system over a base defined by a
   polynomial (`F_V`), with no table collisions (`docs/ic/SWEEPER.md`,
   Tracks). Whether a dense degree-N membership polynomial counts as
   "algebraically defined" is the owner's call (D1, §8). PKM's tower base is
   the unambiguous case. That is why it is the second arm.
3. **It isolates one factor against the incumbent.** The Table B incumbent
   (*measured*, `docs/ic/leaderboard.json`, recipe
   `mitm_m2_negfold_walk_balanced`) is `prime-abscissa` + negation-folded
   pair table + walked targets, m = 2. The reference arm is the same base and
   the same walk with the pair table replaced by an msolve solve. So the two
   arms see the same trial points, and their difference is the oracle alone.
   This makes the comparison exact-checkable: the hit sets must be equal.
4. **msolve is the solver AGENTS.md names.** §15 says msolve is "the open
   reference for F4". The desk review records that the in-house F4 needs
   10³–10⁷ more operations than a brute grid on PKM systems, so it is not a
   published engine (`research/ic_sweeper_20261024/README.md` §2.5, citing
   `E3.md` on branch `prime-semaev-20261019`). The newer `f4_fp` engine is
   near msolve on generic systems (*measured*:
   msolve ÷ f4_fp = 0.96–1.22 single-threaded,
   `research/f4_fp_msolve_parity_20261011/README.md`). It runs only as the §15
   engine comparison, never as the reference.
5. **The reference is not expected to be competitive, and that is fine.** The
   only measurement of this configuration in the program is a Python harness
   calling msolve 0.6.5 at p = 62401 (*measured*,
   `crypto-autoresearcher:src/crypto_autoresearcher/index_calculus/README.md`
   §2):
   - per-target msolve time grows like `|F|^3.80 [3.73, 3.88]` at m = 2 and
     `|F|^4.41` at m = 3;
   - medians were 0.120 s at |F| = 104 and 3.55 s at |F| = 258 (m = 2), and
     6.97 s at |F| = 25 and 52.9 s at |F| = 39 (m = 3);
   - msolve agreed with enumeration on 685/685 targets.

   Its value is that the program can then say, at matched (p, base, m), what
   published IC costs on its own instrument. Algebraic-track prime candidates
   are ranked against that from then on, the precondition P4b sets for saying
   "beats published" (`research/ic_sweeper_20261024/README.md`).

**What was set aside.**

- **FGHR's 2-torsion invariants.** They need rational 2-torsion. Every Table B
  prime curve has odd order (*measured*, `docs/curves/registry.json`: orders
  65003, 1046999, 10933753 for the 16, 20 and 24-bit rows), and
  `prime_search` curves have cofactor 1.
- **FGHR's S_m symmetrisation.** It is published for F_(q^n). Over F_p it can
  be written (membership as `F_V(T) mod χ_e(T)`), but that is a construction
  of this note, so it is a phase-2 *derived* arm.
- **Amadori–Pintore–Sala's one-relation variant.** It is a different
  algorithm (Las Vegas, about 0.6 success per the state-of-the-art note §8.5),
  not a decomposition oracle, so it does not fit the arm.
- **m ≥ 4.** The PKM note §3.7 shows only m ≥ 4 can matter asymptotically,
  and `S_5` has degree 8 in each variable. It becomes a feasibility probe at
  ≤ 12 bits (phase 2).

## 3. The solver: msolve, and what installing it takes

- **Do not use the distribution package.** crypto-autoresearcher's `msolve.py`
  docstring records that Ubuntu 24.04's `msolve 0.6.5-1build2` is wrong above
  p = 2^16:
  - it segfaults on the m = 2 decomposition system at p = 260317;
  - it segfaults on `{x1^2 − 1, x2^2 − 1}` at p = 65537 and 2^31 − 1;
  - it returns `{x1 − 2}` for the inconsistent `{x1 − 1, x1 − 2}` at
    p = 65537.

  The Table B roster runs to about p = 2^24.
- **Build from source at a pinned commit.** The candidate pin is
  `a18d0de45550363405f3dd8747abbf33f4384a06`, which crypto-autoresearcher
  built with 71/71 upstream tests passing
  (`research/snfs_ecdlp_ledger_20261017/toolchain/math-readiness/README.md`
  there). The build:
  - needs GMP, MPFR and FLINT, linked statically;
  - uses `-march=x86-64-v3`, not the host's flags. The readiness build was
    host-specific, and Spot nodes vary;
  - carries a small patch that counts F_p multiply-adds in the linear algebra
    and FGLM. Without it msolve reports no operation count, only matrix shapes
    in `-v 2`;
  - is checked with `make check` and `readelf` (no `INTERP`: the EKS
    executable must be static, `taskq/deploy/eks-karpenter/build-bundle`).
- **Self-test before every session.** Run the three failing checks above at
  that session's p, and compare against the grid (`ecbench solver-selftest`,
  IMPLEMENTATION.md §2.10). No build in this repository has been tested above
  2^16. The other msolve here, 0.9.5, ran only at p = 17 and 29
  (`research/radical_pair_image_20260921/contract.json`).
- **Not in this container.** msolve is absent here, and this container has
  about 900 MB free disk, too little for a FLINT and msolve source build. The
  build belongs on the image build host.

## 4. Draft protocol, summary

The full draft is in [`PROTOCOL.md`](PROTOCOL.md) and
[`protocol.draft.json`](protocol.draft.json).

**Arms:**

- `rho-neg`: the matched reference, `rho.negation`, with its A/A twin;
- `inc`: the Table B incumbent, with base size N filled at freeze;
- `pub-m2`: the reference arm at the incumbent's N;
- `pub-m2-tuned`: the reference arm at its own train-selected N;
- `pub-m2-aa`;
- `pub-m2-pos`: positive control, 1000 planted `P_i ± P_j`;
- `pub-m2-grid`: negative control, the counted exhaustive grid on the same
  system. It must give byte-identical relations, and msolve must say "no
  solution" on every grid-negative trial point;
- `pub-m2-f4fp`: the AGENTS.md §15 engine comparison;
- at m = 3, `pub-m3-tuned` against `inc-m3` (`rootfind-subtract:m=3`,
  unclassified track).

**Sizes:**

- m = 2: the Table B roster curves at 10, 12, 14, 16, 18, 20, 22 and 24 bits,
  plus today's tuning holdout curves at 20 and 24 bits;
- m = 3: 10–18 bits;
- targets: 48 × 2 rounds at ≤ 18 bits, falling to 8 × 2 at 24 bits.

**Seeds:** from the frozen protocol hash. The holdout curve seed 20261018202
is fixed on purpose.

**Band:** every arm, rho included, at w = 0, 0.1 and 1 GAE per native
operation (`band.rs`: `G − C + w·(U + n_C)`). msolve's multiply-adds are part
of U through the `solver_*_uncharged` path. They are never priced by wall time
inside S.

**Decision rules:**

1. **Admission.** All exact checks hold, replay is identical, and the counted
   A/A ratios are exactly 1.000. Otherwise the arm is not established.
2. **Distance remaining.** `Σ S_pub / Σ S_rho` per size and weight, with 95%
   cluster-bootstrap intervals and α fits. This is a distance, not a gate.
3. **Incumbent against the reference: context only.** The band order (or
   `unresolved_in_band`) is reported, but `docs/ic/SWEEPER.md` keeps the table
   track unranked against the algebraic track, so neither arm gets an
   improvement verdict until the owner rules (D2).
4. **Engine.** f4_fp against msolve as a reported win or loss, on equal
   outputs only.

## 5. Implementation, summary

See [`IMPLEMENTATION.md`](IMPLEMENTATION.md). The touch points:

**New files.**

- `src/cryptanalysis/ic_framework/fp_solvers.rs`: the `FpSystem`,
  `FpVerdict` and `FpSystemSolver` plug point, and the `msolve`, `f4-fp` and
  `fp-grid` engines with the `fp_solver_by_name` registry. Today
  `stages.rs::SystemSolver` speaks only `BooleanSystem` over F_2.
- `src/cryptanalysis/ic_framework/msolve.rs`: the subprocess contract.
- `src/cryptanalysis/ic_framework/semaev_fp.rs`: S₃ and S₄ over u64 F_p,
  copied from `pkm_semaev.rs` so as not to move that file's bound hash.
- `src/cryptanalysis/ic_framework/semaev_naive.rs`: `SemaevNaiveOracle`, a
  `DecompositionOracle<PrimeCurve>` named `semaev-naive`.

**Edits.**

- `ic_framework/mod.rs`: the module lines only. Pricing is unchanged.
- `ic_framework/stages.rs`: one default trait method, `detail()`, so the
  engine's hash reaches the record.
- `ecbench/methods.rs`: the `solve_ic_prime` match arm, the
  `IC_PIPELINE_PARAMS` help text, and the engine detail in `ic_report`.
- `ecbench/claim.rs`: bind the new files and the msolve sha256 into the IC1
  candidate identity.
- `ecbench/audit.rs`: a replay without the pinned msolve reads
  `unavailable`, never identical.
- `src/bin/ecbench.rs`: the `solver-selftest` and `oracle-check` subcommands.
- `src/bin/ic/bench.rs::run_prime`: the same match arm.
- `src/bin/ic/sweep/space.rs`: a `prime-algebraic` space on
  `Track::Algebraic` with this arm as its baseline.

**Build and CI.**

- `taskq/deploy/eks-karpenter/{build-msolve (new), build-bundle,
  Dockerfile.ecbench}`.
- A request validator check for the helper binary's hash.
- `.github/workflows/{ecbench,audit-runs-daily}.yml`: install the pinned
  msolve before replays.

**Docs.** `docs/ecbench/README.md` §7, `docs/ic/FRAMEWORK.md` and
`docs/ic/SWEEPER.md`.

## 6. Disk, CPU and RAM

All runtime figures here are *derived*, not measured. They extrapolate the
crypto-autoresearcher msolve fit, which was measured on a different host, with
msolve 0.6.5, at one 16-bit prime, and is valid down to medians of about
0.05 s. The model:

- per-call time at m = 2: `T₂(N) = 0.120 s · (N/104)^3.80 + 0.004 s`
  (start-up);
- per-call time at m = 3: `T₃(N) = 6.97 s · (N/25)^4.41 + 0.004 s`;
- trials per solve: `r/(2N)` at m = 2 and `3r/(4N²)` at m = 3, the hit law
  for a negation-folded base.

The trial law overestimates the measured Table B trial count by about 1.7×
at 20 bits: 6196 measured trials (`docs/ic/leaderboard.json`, 6195.7 mean) against
about 10,265 from the model (r/(2N) with r = 1046999, N = 51; the table below
uses r ≈ 2^20 and shows 10,280), because the linear algebra stops early. So
read these as upper-side estimates within roughly ×3 either way. The
preflight gate in PROTOCOL.md §7 replaces them with measurements before
freeze.

| bits (log₂ r) | N_inc (Table B columns) | trials/solve, derived | msolve s/call, derived | **pub-m2 s/solve**, derived | pub-m2-tuned s/solve (N ≈ 32), derived |
|--:|--:|--:|--:|--:|--:|
| 9.7 | 5 | 83 | 0.004 | 0.3 | 0.1 |
| 11.9 | 16 | 119 | 0.004 | 0.5 | 0.3 |
| 14.0 | 13 | 630 | 0.004 | 2.5 | 1.4 |
| 16.0 | 20 | 1,638 | 0.004 | 6.9 | 5.5 |
| 18.0 | 32 | 4,096 | 0.005 | 22 | 22 |
| 20.0 | 51 | 10,280 | 0.012 | 123 | 88 |
| 21.7 | 74 | 23,019 | 0.037 | 850 | 285 |
| 23.4 | 111 | 49,860 | 0.158 | **7,863** | 927 |

At m = 3 with a small tuned base (N ≈ 8), the derived per-solve time is
0.5 s at 10 bits, 2.2 s at 12, 9.6 s at 14, 38 s at 16 and 153 s at 18.

**CPU (derived).** Single-thread CPU, summed over the planned executions:

| item | CPU |
|:--|--:|
| `pub-m2` at 10–22 bits (96/96/96/96/96/48/24 executions) | about 29,400 s |
| `pub-m2` at 24 bits (8 executions) | about 62,900 s |
| `pub-m2-tuned` | about 28,700 s |
| `pub-m2-aa` (25% of 10–22 bits) | about 7,300 s |
| `pub-m3-tuned` | about 19,500 s |
| `pub-m2-f4fp` (≤ 20 bits, assuming msolve parity) | about 9,000 s |
| train selection | about 20,000 s |
| rho, `inc`, grid and positive-control arms | seconds to minutes |
| **total** | **about 177,000 s ≈ 49 CPU-hours** (`protocol.draft.json` `estimated_total_seconds`), plausible range about 15–150 CPU-hours |

**Wall time.** Each execution is single-threaded and independent. At the
28-CPU job cap noted in `research/prime_fall_degree_20261027/isolab/LAUNCH.md`,
expect about 4–8 h of wall time. The floor is the 24-bit `pub-m2`
executions at about 2.2 h each. The per-execution timeout is 6 h.

**RAM.** Not measured for these systems. The plan is 2 GiB per execution until
the preflight's peak RSS replaces it. The m = 2 systems have three equations
in three variables with membership degree N ≤ 111. Their Macaulay matrices
should stay far below that, but this is unverified.

**Disk.** The image grows by one static msolve binary (estimate tens of MB;
the current ecbench image is 147,172,601 bytes per
`taskq/deploy/eks-karpenter/image.json`). The records hold about 9,000
executions at a few KB each, so under 100 MB with logs. The `-v 2` text of
each msolve call is parsed and discarded; keep a 1% sample, gzipped. Temporary
`.ms` files live in a per-execution temp directory and are deleted per call.
Building ecbench and static FLINT/msolve needs several GB of build disk on the
build host (estimate), which rules out this container.

## 7. Route

**Default route: isolab-eks** (CLAUDE.md; `.claude/skills/isolab-eks`), via
`isolab_k8s_benchmark`, the audited ecbench contract. The steps, in order:

1. The PR lands on `main`. The trusted publisher builds the image only from
   `main`, and `isolab.eks.request/v1` runs only ecbench specs. That is
   exactly why the reference arm is an `ic.pipeline` oracle and not a
   separate tool, unlike the blocked prime fall-degree study
   (`research/prime_fall_degree_20261027/isolab/LAUNCH.md`).
2. The image is built with msolve, and `build-info.json` carries both
   hashes.
3. The protocol is frozen with the preflight runtimes filled in.
4. `.resume/RESUME.md` is checkpointed (`scripts/checkpoint.sh init`).
5. The director writes a card and the executor submits it unchanged. One
   x86 instance family is used for the whole study, because the wall metrics
   compare within a host.
6. Isolation L2 is requested, because of the AGENTS.md §15 wall comparison.
   The primary metrics are counted.

**Local development.** Unit tests (fixture p = 113, the `fp-grid` /
`f4-fp` agreement), the msolve self-test and a 12-bit scratch session with
`verify --replay` stay local on a development machine with msolve installed
and enough disk. They are not experiments.

**Second host.** Before any row goes to `docs/ic/LEADERBOARD.md` or
`benchmarks/README.md`, the validation gate needs a replay certificate, a
validator report and the matched rho reference with the band
(`docs/VALIDATION_GATE.md`). The counted values replay on any host that has
the pinned msolve.

## 8. Owner decisions needed

- **D1.** Does the small-abscissa base with the dense membership polynomial
  `F_V = ∏(x − c)` count as "an algebraically defined factor base" under the
  2026-10-24 criterion? If not, the counted reference becomes the PKM tower
  arm (phase 2) and the naive arm is reference-only.
- **D2.** May the table-track incumbent be ranked against the published
  algebraic reference? The stated goal (2026-10-22) is to beat published IC at
  matched (q, n, m), but `docs/ic/SWEEPER.md` keeps the table track unranked
  against the algebraic track. Until the owner rules, C3 is descriptive.
- **D3.** Is a counter patch to msolve acceptable? It is a small, pinned,
  vendored patch whose sha256 binds the candidate identity. Without it,
  msolve's work can only be priced in a proxy unit (matrix entries from `-v 2`),
  or by wall time, which ecbench already removes from S.
- **D4.** Authorisation and budget for roughly 15–150 CPU-hours on EKS Spot,
  plus the image change (msolve in the ecbench image) and the CI change
  (msolve in the replay workflows).
- **D5.** Should the derived F_p symmetrisation arm and the m = 4 probe be
  in scope at all (phase 2)?

## 9. Blockers

- **No code.** No `semaev-naive` oracle, no F_p solver plug point, no
  `msolve` engine. Implementation needs a session with a Rust toolchain and
  several GB of disk. This container has about 900 MB.
- **No sound msolve build above 2^16** exists in or for this repository yet.
  The Ubuntu package is known-bad; a pinned static build with the self-test is
  needed.
- **The ecbench EKS image is static and carries only `ecbench`.** It needs
  `build-msolve` and possibly a request-validator extension for the second
  hash.
- **Pending incumbent value.** The 20-bit incumbent (N = 45) awaits a
  second-host replay (`research/prime_base_oracle_tuning_20261029`), so
  panel H's `inc` at 20 bits must say which incumbent it is.
- **Owner rulings D1 and D2** decide how the result is labelled, not whether
  it can be measured.

## 10. Sources read

- `docs/ic/LEADERBOARD.md`: Table B; the bitop/v1 prime ladder.
- `docs/ic/leaderboard.json`: the 20-bit recipe.
- `docs/ic/SWEEPER.md`: Tracks.
- `docs/curves/registry.json`.
- `research/ic_sweeper_20261024/README.md`: §2.5, P4b.
- `research/prime_base_oracle_tuning_20261029/`: README, the specs
  `holdout-P20/24/28.json`, VALIDATION.json.
- `research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md`: §1.1–1.3,
  §3.7, §4.1–4.3, references.
- `research/notes/ecdlp-general/RESEARCH_ECDLP_STATE_OF_THE_ART.md`: §8.
- `research/ecdlp_autolab/yokoyama_lower_bound.md`.
- `research/f4_fp_msolve_parity_20261011/{README.md,run.sh}`.
- `research/radical_pair_image_20260921/{README.md,contract.json,results/run-003/raw/*.stdout}`.
- `research/prime_fall_degree_20261027/isolab/{LAUNCH.md,Dockerfile}`.
- `src/cryptanalysis/ic_framework/{stages.rs,plugins.rs,mod.rs,pkm_semaev.rs,solvers.rs}`.
- `src/cryptanalysis/ecbench/{methods.rs,band.rs,claim.rs,workload.rs}`.
- `src/bin/ic/{bench.rs,sweep/space.rs}`.
- `examples/gb_cas_export.rs`.
- `taskq/deploy/eks-karpenter/{Dockerfile.ecbench,build-bundle,image.json}`.
- `.agents/skills/isolab-eks/references/submission.md`.
- `.claude/skills/ecbench-extend/SKILL.md`.
- `AGENTS.md`: "Implementation language", §5a, §15, "Performance results
  also land in benchmarks/".
- crypto-autoresearcher:
  - `src/crypto_autoresearcher/index_calculus/{msolve.py,README.md}`;
  - `schemas/experiment-protocol.v1.json`;
  - `tools/experiment_protocol.py`;
  - `examples/experiment-protocol/ecbench-l2-smoke-subset/protocol.json`;
  - `research/snfs_ecdlp_ledger_20261017/toolchain/math-readiness/README.md`.

No paper was read in full for this note. The literature attributions are the
repository notes' own, and the protocol's `literature` block marks them
`verified_by: null`.

## 11. Visual report

No graph or PDF was produced. This is a design note with no recorded values
to plot, and this container has no PDF engine (`pandoc` is present but no
LaTeX or typst). Per `research-visuals`, the reference-arm run will leave its
run report, graphs (S against bits per arm and weight, distance to rho,
msolve multiply-adds against N) and PDF in `reports/`, and its `benchmarks/`
entry.
