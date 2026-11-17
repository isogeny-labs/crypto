# Lane C leads: prime fields, extension fields, covers and transfers (2026-11-11)

**Author role:** Idea Generator. Proposals only. Nothing here was run: this session had no shell, so every number below is read from a file on disk (path given) or is labelled *derived* (arithmetic on those numbers, shown). Every cited result is a producer-reported value, `[pcc:unverified]`, unless a `VALIDATION.json` is named. No result, hypothesis status or evidence file was edited. I did not append to `docs/session-logs/` because the task restricted edits to this one file; the coordinator should log it.

**Scope:** weak, undeployed classes only (full rational 2-torsion with `#E = 4l` for the Joux-Vitse cover; `4 | #E_0(F_{q^3})` for the GHS-quartic family). Nothing concerns a deployed curve or key recovery.

## Sources checked

- **research-kb MCP:** not available in this session (no `kb_status`/`search_knowledge` tool exposed). Repository files plus web search were used instead. The sibling atlas is vault-encrypted (VLT1); I did not touch it. Novelty below is therefore checked against the 173 sweep ids, the treatise and the study READMEs, **not** against the atlas disposition fields.
- **Repository (read):** `AGENTS.md` (Success criterion, Preserve scope, Name the obstruction, boundary/table/ratio, 5a, no Python); `papers/ecdlp-index-calculus-treatise/sections/{conclusion,prog_openproblems,prog_prime,prime_ext,transfer_ext,distance,prog_transfer,prog_frontier}.tex`; `docs/ic/LEADERBOARD.md`; `research/lead_sweep_20261110/SWEEP_REPORT.md`; `research/leads_20261111/EXISTING_LEADS.txt`; `research/prime_field_lead_assessment_20261107/README.md`; `research/jv_cover_next_leads_20261102/LEADS.md`; `research/jv_cover_fbline_20261102/{README.md,run.sh,results/walls.txt}`; `research/jv_cover_crossover_20261101/run.sh`; `research/jv_sieve_negrho_20261102/{results/inputs/ic_1511_sparse.json,review/validator/REPORT.md}`; `research/ic_cover_weakclass_20261023/{REPORT.md,PROTOCOL.md}`; `research/quartic_n4v2_20261020/README.md`; `research/ic_residual_dp_eq_20261101/{README.md,PROTOCOL.md}`; `src/cryptanalysis/jv_sieve.rs` (lines 41, 1620-1667, 2086, 2101-2129) and `src/cryptanalysis/jv_cover.rs` (lines 2703-2830); `problems/jv-cover-subbase-restricted-test.md`, `problems/quartic-binary-ic-wide-scalars.md`.
- **Literature (web, standard search; abstracts and catalogue entries only unless stated):**
  - Joux, Vitse, *Elliptic curve discrete logarithm problem over small degree extension fields*, ePrint [2010/157](https://eprint.iacr.org/2010/157) (J. Cryptology 2013), and *Cover and decomposition index calculus on elliptic curves made practical*, Eurocrypt 2012, [DOI 10.1007/978-3-642-29011-4_3](https://link.springer.com/content/pdf/10.1007/978-3-642-29011-4_3.pdf) (151-bit degree-6 extension record).
  - Diem, Thome, *Index calculus in class groups of non-hyperelliptic curves of genus three*, J. Cryptology 21(4), 2008 (plane quartic, heuristic `O~(q)`); Diem, ANTS-VII 2006; Diem, *On the discrete logarithm problem for plane curves*, [JTNB 2012](https://www.numdam.org/articles/10.5802/jtnb.815/).
  - Vitse, Wallet, *Improved sieving on algebraic curves*, LATINCRYPT 2015, ePrint [2015/211](https://eprint.iacr.org/2015/211) (sieving for non-hyperelliptic relation search, about 3x on Diem-Kochinke).
  - Laine, Lauter, *Time-memory trade-offs for index calculus in genus 3*, J. Math. Cryptol. 9(2), 2015, ePrint [2014/346](https://eprint.iacr.org/2014/346). **Only the abstract was retrievable** (the PDF returned 403 or binary); I did not read its mechanism and do not rely on it.
  - Gaudry, Thériault, Thomé, Diem double large prime variation (cited in the treatise as `gtthd2007`; not re-read).
  - A web search for 2024-2025 extension-field index-calculus work returned only surveys and a prime-field Edwards note; no paper that applies a Hilbert-function early stop inside a Joux-Vitse descent was found (a negative search, not a proof of novelty).

## What the evidence on disk says about where lane C can move

1. The **only** validated below-rho measurement is the Joux-Vitse sieve at `p = 1511`: `0.00647 [0.00545, 0.00799]` of matched negation rho in group additions (U1), `0.0156` in textbook F_p multiplications (U2), order holding across the band, validated as *accounting* (`research/jv_sieve_negrho_20261102/VALIDATION.json`, `VAL-20261103-c31c76`). Its F4, descent and linear-algebra additions are bounded by a proxy, not emitted (validator REPORT.md: proxy `1.56e9` per run; "layer A stays below parity unless the unemitted additions exceed k = 69 per unemitted multiplication").
2. The **Nagao arm with the factor-base line search** is below negation rho in U1 (`0.791, 0.659, 0.548` at `p = 1301, 1451, 1601`) and above in U2 (`1.900, 1.585, 1.318`); modelled U2 crossover `p* ~ 1799 [1732, 1872]` (`research/jv_cover_fbline_20261102/README.md`). No validation file.
3. **Found by reading the run scripts (new, checkable):** both the crossover study (`research/jv_cover_crossover_20261101/run.sh`: `--exp dlp --rho-runs 0 ...`) and the line-search study (`research/jv_cover_fbline_20261102/run.sh`: `COMMON=(--exp dlp --rho-runs 0 --rho-ref 1.3 --check-every 64)`) run the Nagao arm with **no `--stop`**, i.e. full F4. The validated Hilbert early stop (`--stop hilbert:16`, `S_A/S_B = 1.848 / 1.875 / 1.868 / 1.849` at `p = 53 / 101 / 251 / 503`, `benchmarks/20261029-jv-cover-hilbert-stop/`, `research/ic_cover_weakclass_20261023/VALIDATION.json`) was measured only at `p <= 503` and, per its PROTOCOL.md row L4, was "reachable from `--exp ccov`, not from `--exp dlp`" before that study. In `src/cryptanalysis/jv_sieve.rs` the sieve route's descent builds its F4 options with `opts_for(24, 600.0, stop)` (line 2086; `opts_for_mode`/`StopMode::Hilbert` is not imported at line 41). So neither of the two below-rho arms has ever been run with the one validated F4 saving the programme has for this cover.
4. The prime-field generic track and the isogeny-transport lanes are at a constant-factor floor (see "Considered and not proposed"); no new prime-field lead survived the check, and I say so rather than pad.

---

## Lead 1 (ranked first)

```yaml
id: LEAD-C-jv-nagao-hilbert-at-scale
title: Hilbert early stop plus line search on the Joux-Vitse Nagao arm at p = 1301-1601, one paired experiment
obstruction_attacked: >
  Open problem 11 (treatise prog_openproblems.tex): the Nagao arm's distance to matched
  negation rho in textbook F_p multiplications (U2) is 1.900, 1.585, 1.318 at p = 1301, 1451, 1601
  even after the line search, because a decomposition test costs about 9.5e5 F_p multiplications
  (jv_cover_fbline README, "Next steps" item 3). The U2 order does not hold across the band.
claim: >
  On the 9 instances of research/jv_cover_fbline_20261102 (p in {1301,1451,1601}, seeds 1-3), running the
  line-search arm with `--stop hilbert:16` returns bit-identical residuals, relations, relation digest and
  logarithms and lowers counted S by a factor in [0.50, 0.60] (paired geometric mean) at every prime and at
  w = 0, 0.1 and 1. Re-scored against the existing pooled 126-run negation-rho reference with the unchanged
  `negrho_rescore` tool, the U2 ratio at w = 0 falls to about 1.03 / 0.86 / 0.71 and the U1 ratio to about
  0.43 / 0.36 / 0.30 at p = 1301 / 1451 / 1601.
mechanism: >
  The Nagao test is six quadrics in six F_p unknowns (Bezout 2^6 = 64); the Hilbert rule drops every pending
  F4 pair that the complete-intersection Hilbert function proves reduces to zero and returns the same reduced
  basis (ic_cover_weakclass REPORT: 25 of 25 cells equal relations, digest equal; positive control 20/20 planted).
  The system shape does not depend on p, so the 1.85x measured at p <= 503 should transfer. The line search
  changes which instances are tried, not the F4 system, so the two savings should multiply. The Hilbert stop's
  repaired form leaves the emitted native counter at 0.983-0.990 of baseline (REPORT), so the saving is not the
  B0 relabelling (B0 raised unpriced F4-driver work 8.3-8.5x).
prediction:
  hilbert_over_plain_S_paired: "0.54 [0.50, 0.60]  (derived: 1/1.848 = 0.541, 1/1.875 = 0.533, 1/1.868 = 0.535, 1/1.849 = 0.541 at p <= 503)"
  U2_w0_line_plus_hilbert: {p1301: "1.03 [0.93, 1.14]", p1451: "0.86 [0.78, 0.95]", p1601: "0.71 [0.64, 0.80]"}   # derived: 1.900*0.54, 1.585*0.54, 1.318*0.54; intervals assume the line study's relative widths
  U1_line_plus_hilbert: {p1301: 0.43, p1451: 0.36, p1601: 0.30}   # derived: 0.791*0.54, 0.659*0.54, 0.548*0.54
  modelled_U2_crossover: "p* ~ 1342 (derived: 1799 / 1.85^(1/2.10), slope -2.10 from the fbline README); inside the already-measured range"
  band: "Hilbert/plain ratio stays in [0.50, 0.62] at w = 0.1 and w = 1 (leaderboard shows the same 1.84-1.89x at all three weights at p <= 503); U2 order below 1 at all three weights predicted at p = 1451 and 1601"
discriminating_test:
  local_stage_1_counts_only:
    what: >
      Rebuild `examples/jv_cover` (existing Rust) and run arm B = `--exp dlp --stop hilbert:16` with
      `JV_FB_LINE=65536` on the 9 instances; arm A = the frozen line-search records already on disk
      (results/ from jv_cover_fbline_20261102); counts are deterministic so no rho run is needed for S_B/S_A.
    size: "walls.txt: line-search runs took 436-625 s each single-threaded; expected 250-350 s with the stop. 4 runs in parallel, 3 jobs of about 7 min each; each job under 20 min on 4 vCPU / 15 GB."
    controls:
      - "C1: JV_FB_LINE unset and no --stop reproduces the p=1301 seed-3 record exactly (as in the fbline study)."
      - "C2 A/A: arm A re-run twice gives identical counts (deterministic)."
      - "C3: relation_digest, residuals, relations, decompositions, logarithm identical between A and B on 9 of 9."
      - "C4 positive: `--exp ccov --constructed 20` at p = 101 recovers 20 of 20 planted decompositions with the stop on."
      - "C5 fallback: report `hilbert_applied` and `hilbert_fallback` per instance; fallbacks must be below 10 percent of systems."
  local_stage_2_rescore: "Run the unchanged Rust `negrho_rescore` (research/jv_cover_negrho_rescore_20261101/fit) on the new records against the frozen pooled 126-run reference; confirm l and c_add_e equal the reference records of the same (p, seed)."
  isolab_job_if_stage_1_passes: >
    One isolab-eks task card (AGENTS.md 9a, frozen by experiment-director): the 9 instances for arms plain, line,
    line+Hilbert with F4-driver pair selection, matrix assembly and table lookups emitted as native counters,
    plus 16 fresh negation-rho runs per prime for a homogeneity check against the pooled reference
    (about 16 x 3 x 600-1500 s = 8-20 CPU-hours at p = 1301-1601), replay certificate and validator.
falsification: >
  Killed if any of: (F1) any digest, relation or logarithm mismatch between A and B; (F2) the pooled paired
  S_B/S_A has its upper 95% bound above 0.70 (gain below 1.43x) at any prime, which would mean the p <= 503
  saving does not transfer; (F3) hilbert_fallback exceeds 25 percent of systems; (F4) the emitted native
  counter (f4_native_ops) rises by more than 1.10x, which would make this a B0-type relabelling.
  Not killed, but reported as a miss: U2 at p = 1601, w = 0 with upper bound >= 1 despite F1-F4 passing (the
  composition then does not flip U2 and the distance remaining is stated).
row_moved: >
  Nagao line-search arm rows of research/jv_cover_fbline_20261102 (unvalidated, outside tables A-C; leaderboard
  Table D is withheld). Predicted: U2 distance remaining 1.318x -> 0.71x at p = 1601 (a 1.85x engineering
  improvement of that arm), and U2 below 1 at 1451 and 1601. Class: engineering (counted S falls, native counter flat).
cost_band:
  unit: "S = (counted F_p mults + w * unpriced native ops) / (c_add_E * sqrt(l)), w in {0, 0.1, 1}; U1 group additions, U2 textbook F_p mults"
  arms:
    plain_nagao: "reference record on disk; unpriced: f4_native_ops (lower bound: F_p additions, pair selection, memory traffic not emitted)"
    line: "same; plus the search's own 22,772 mults charged (fbline C2)"
    line_plus_hilbert: "same counters plus hilbert_check_muls (charged by the existing code)"
    rho_negation_reference: "pooled 126 runs (homogeneous, perm p 0.31); unpriced counters at p = 1511 (validator): canon_cmp 1.44e9, hash 5.75e9, dp_check 1.44e9, scalar_ops 4.33e9, table_ops 2.2e4, cycle_cmp 1.14e10, fp_adds 6.49e10 per run; per-prime counts come from the pooled records"
  ranking_rule: "rank two arms only where the order holds at w = 0, 0.1 and 1; otherwise 'unresolved'"
controls_summary: "see local_stage_1; reference unchanged; no rho run is re-tuned"
risks:
  - "The Hilbert gain was measured at p <= 503 only; fallbacks or degenerate leading forms could be more frequent at p ~ 1500."
  - "The line search changes the instance (G, Q moved to the new line); the stop interacts only through F4, but the combination has not been run. Run stage 1 before any claim."
  - "The rho reference is pooled from a different experiment (the same limitation the line-search study carried); the validation gate needs a matched same-binary reference, hence the isolab job."
  - "U2 prices the cover at harness multiplication counts and rho at tuned counts (fbline README, Deviations); a tuned cover F4 would narrow U2 further, a minimal F_{q^3} product would make rho about 1.34x cheaper in multiplications (red team RT-20261103-fd05ef). Neither is changed here."
lean_obligation: >
  The correctness argument the code relies on, `HilbertStop.drop_sound`: for polynomials f_1..f_6 in F_p[x_1..x_6]
  whose grevlex leading forms are a regular sequence of quadrics, if a partial Gröbner basis G of I=<f_i> has,
  in degree d, a quotient dimension equal to the coefficient of t^d in (1+t)^6, then no unprocessed S-pair of degree d
  produces a new leading monomial. No Lean statement exists on disk (a search of *.lean for "Hilbert" finds only
  unrelated files). Until it is proved the empirical gate is digest equality plus the existing Hilbert-versus-oracle unit test;
  the claim does not depend on the theorem being formalised.
closest_baseline:
  published: "Joux-Vitse 2012 cover-and-decomposition with F4 trace replay (ePrint 2010/157 and Eurocrypt 2012); this repository's trace replay was measured a regression on the Nagao arm (research/ic_cover_leads, L3)."
  in_repo: "research/ic_cover_weakclass_20261023 (Hilbert stop, p <= 503, validated) and research/jv_cover_fbline_20261102 (line search, p >= 1301, unvalidated). Each exists; their composition at p >= 1301 does not."
  nearest_sweep_lead: "LEAD-TRF-jv-nagao-genus2-cover-decomp (RESULT.md: a toy count of the smooth fraction, inconclusive on an A/A control). It tests a counting identity, not the F4 cost; no overlap in mechanism."
sources: [see "Sources checked"; research/ic_cover_weakclass_20261023/REPORT.md; research/jv_cover_fbline_20261102/README.md, run.sh, results/walls.txt]
implementation_language: "Rust only (existing examples/jv_cover and negrho_rescore); no Python."
```

## Lead 2

```yaml
id: LEAD-C-jv-sieve-hilbert-descent
title: Wire the Hilbert early stop into the Joux-Vitse sieve route's descent F4
obstruction_attacked: >
  Open problem 11, question 1 and the phase split in research/jv_cover_next_leads_20261102/LEADS.md:
  at p = 1511 the sieve route's descent is 42 percent of multiplications (1.44e9 of 3.43e9 on seed 17;
  descent_f4_muls 1.049e9 plus descent_lin_muls 0.380e9), at about 7.1e5 F4 multiplications per test and
  an average of about 720 tests per success.
claim: >
  With `opts_for(24, 600.0, stop)` at `src/cryptanalysis/jv_sieve.rs:2086` replaced by a `StopMode::Hilbert(16)`
  path, the sieve route at p = 1511 (V2 instances, seeds 17-32, `JV_LIN=sparse`) returns the same relation digest,
  descent digest and logarithms, lowers descent_f4_muls by a factor of 0.54 [0.50, 0.60], and lowers total counted
  multiplications by a paired geometric-mean factor of 0.87 [0.82, 0.93].
mechanism: >
  Same F4 system as the Nagao arm (the descent calls the shared `nagao_decompose`, jv_cover.rs:2743), so the
  same Hilbert-function pair elimination applies. The stop acts on the F4 share only; the linear stage
  (descent_lin_muls, about 11 percent of seed 17) and the sieve are untouched, which bounds the gain.
prediction:
  desk_check_first: >
    Before any code change, compute f_s = descent_f4_muls / total_muls for all 16 records in
    research/jv_sieve_negrho_20261102/results/inputs/ic_1511_sparse.json and the predicted ratio 1 - f_s * 0.46.
    From the four records I read (seeds 17-20): 0.306, 0.052, 0.192, 0.491 (mean 0.26; ratio-of-sums 0.316),
    i.e. predicted ratio 0.88 (mean of ratios) to 0.855 (ratio of sums). This is arithmetic on committed numbers,
    not a measurement.
  paired_S_ratio_p1511: "0.87 [0.82, 0.93]"
  S_plus_ratio: "0.90 [0.85, 0.95] (F4 additions fall with F4 multiplications)"
  effect_on_validated_row: "U1 sieve/rho 0.00647 -> about 0.0056 (derived); the row stays below rho, 155x -> about 178x"
discriminating_test:
  local: >
    Patch (Rust) to pass `StopMode` into `run_cover_sieve_dlp_on` and honour `--stop hilbert:16`; run seeds 17-32 at
    p = 1511 for arms plain (V2 options, `--stop 64`) and Hilbert. The V2 records show the whole sieve solve is
    about 3.4e9 multiplications (about 10 s: LEADS.md L1 row), so 32 runs fit in one job of a few minutes on 4 vCPU.
    Repeat at p = 1009 and 1823. Controls: plain arm reproduces V2 bit for bit (C1 of the fbline study); A/A copy;
    digest equality; hilbert_fallback count; `trace=false` as in the V2 records (descent_replayed = 0).
  isolab: "None needed for the first answer. A claim to the board needs the validation gate (replay certificate, validator, matched rho = the existing pooled 142 runs plus homogeneity), as for VAL-20261103-c31c76."
falsification: >
  Killed if any of: digest or logarithm mismatch; descent_f4_muls ratio above 0.70 in the pooled paired test;
  total ratio above 0.95; hilbert_fallback above 25 percent of descent systems (the descent family then differs
  from the cover family); or the emitted native counter rises by more than 1.10x.
row_moved: >
  The validated plain sieve route at p = 1511 (VAL-20261103-c31c76) and the H/U line-search sieve arms
  (research/jv_sieve_line_vs_rho_20261103). Predicted improvement 1.15x in S at p = 1511 (engineering). The
  gain shrinks where descent is a smaller share of S.
cost_band:
  arms: "plain sieve (V2); sieve + Hilbert; matched negation rho pooled 142 runs (validator REPORT: 7 unpriced counters listed under Lead 1, p = 1511); layer A (unemitted F4/descent/LA charged as native) and layer B (emitted only), both reported"
  unpriced_cover_counters: "sieve_adds 5.61e9, sieve_lookups 2.94e8, enum_adds 4.72e9 per run (validator); F4/descent/LA additions have no counter (proxy 1.56e9); Lead 3 emits them"
  weights: "w = 0, 0.1, 1; order claimed only where it holds at all three"
risks:
  - "Heavy-tailed descent: tests per success were 736, 96, 256, 2112 on seeds 17-20, so any per-seed S ratio has large variance; use all 16 paired seeds and report the per-seed table."
  - "Trace replay and the Hilbert stop have not been combined (replay is off in V2); do not enable both in this lead."
  - "A 12-13 percent S gain on a row already 155x below rho changes no ranking; its value is the ratchet and the carry-over to Lead 1."
lean_obligation: "Same as Lead 1 (`HilbertStop.drop_sound`)."
closest_baseline:
  published: "Joux-Vitse 2012 sieve and F4 descent; Vitse-Wallet 2015 improved sieving (relation search only, not the descent test)."
  in_repo: "research/jv_sieve_fbline_20261102 (line search on the sieve, 0.623/0.560/0.600 of plain) and research/jv_sieve_adds_20261103 (adds cut, S+ x0.621). Neither touches the descent F4. Problem record problems/jv-cover-subbase-restricted-test.md concerns large primes, not this."
  nearest_sweep_lead: "LEAD-TRF-jv-sieve-fp6-weak-class (toy decomposition-rate check, falsified only at p = 7 as a finite-size effect). No overlap in mechanism."
implementation_language: "Rust only."
```

## Lead 3

```yaml
id: LEAD-C-jv-sieve-emit-price
title: Emit the sieve route's F4, descent and linear-algebra additions, so the one validated below-rho row becomes a priced result
obstruction_attacked: >
  The validator classes VAL-20261103-c31c76 as accounting because "F4/descent/LA additions have no counter; the
  layer-A proxy is 1.56e9" per run, memory is not reported, the replay is sampled (1 of 16 new rho runs, 2 of 16
  cover rows) and the run was local (REPORT.md "Limitations"). This is open problem 11, question 1; it is an
  implementation gap, not a mathematical obstruction.
claim: >
  Counting every F_p addition executed by the sieve route's F4 row operations, linear stage and descent (the
  counters `descent_f4_muls`, `descent_lin_muls`, `descent_weil_muls`, `descent_post_muls` already exist for
  multiplications, jv_sieve.rs:1620-1646) gives an emitted count A_emit per run with A_emit / proxy in [0.80, 1.05]
  on seeds 17-24 at p = 1511, so that layer A recomputed from emitted counts equals the validated layer A
  (U1: 0.00647 / 0.1071 / 0.1214 at w = 0 / 0.1 / 1) within 15 percent, and the order against the pooled 142-run rho
  holds at all three weights in both units.
mechanism: >
  F4 row reduction is a multiply-add per touched entry, so additions are about one per multiplication
  (descent_f4_muls + descent_lin_muls + LA mults on seed 17: 1.049e9 + 0.380e9 + 0.301e9 = 1.73e9 mults,
  against the proxy 1.56e9 averaged over seeds 17-24; derived, one seed vs an 8-seed mean). The proxy is
  therefore close to k = 1 additions per multiplication, far below the k = 69 that would reverse the order.
prediction:
  A_emit_over_proxy: "0.92 [0.80, 1.05]"
  layer_A_U1_emitted: {w0: "0.0065", w0_1: "0.098 [0.090, 0.112]", w1: "0.112 [0.102, 0.128]"}
  layer_A_U2_emitted: {w0_1: "0.107 [0.097, 0.122]", w1: "0.113 [0.103, 0.128]"}
  memory: "peak RSS reported for both arms; no prediction beyond 'below 4 GB' (V2 sparse linear algebra at p = 1511)"
discriminating_test:
  local: >
    Rust patch adding an add counter next to the existing multiplication counter in `f4_fp` row kernels and in the
    sparse/FGLM linear stage, summed into new report fields; peak RSS via getrusage. Re-run the 16 V2 cover seeds
    (about 10 s each) at p = 1511 with the old and new binary; require old fields bit-identical (C1). Rescore with the
    validator's own Rust check (research/jv_sieve_negrho_20261102/review/validator/check/src/main.rs) against the
    frozen pooled rho records; no new rho run is needed for this stage. Under 5 minutes of runtime.
  isolab_job: >
    One task card (isolab-eks): the same cover side with emitted counters on the isolated lab, a full (not sampled)
    replay of all 16 cover rows, and the memory counters; rho arm reused from the frozen records unless the validator
    asks for a fresh homogeneity draw. This closes validator limitations 1 to 4 except the rho-side sampling.
falsification: >
  Killed (as a statement about the proxy) if A_emit / proxy exceeds 1.20 on the pooled seeds: the proxy then was not
  an upper bound and the validated layer A is understated, which would be an erratum to VAL-20261103-c31c76. The
  lead's ordering claim is killed only if an emitted layer-A cell reaches 1 in either unit at any w, which would
  need A_emit above 69 per multiplication (the validator's threshold); this is not expected.
row_moved: >
  The validated sieve row at p = 1511: from "accounting, bounded" to "priced, emitted" (changes the validator's
  class only if the gate re-runs; the ratio itself is predicted unchanged to within 15 percent). No S improvement
  is claimed. Class: accounting correction; it is a priced result in the sense of the brief, not an improvement.
cost_band:
  arms: "sieve route (layer A = emitted F4/descent/LA charged at w; layer B = old emitted-only); pooled negation rho (142 runs); unit U1 and U2"
  counters_listed_per_arm: "cover: sieve_adds, sieve_lookups, enum_adds (emitted), F4 adds, lin adds, descent adds (new); rho: canon_cmp, hash, dp_check, scalar_ops, table_ops, cycle_cmp, fp_adds (validator list). Byte counters: none on either arm (the band prices operations, not bytes; NON_OPERATION_COUNTERS)."
  weights: "w = 0, 0.1, 1 as in the validated row"
risks:
  - "Only labelling: if the new counters equal the proxy, the lead adds a priced statement but no new number; it is still the explicit gap the validator listed."
  - "Rho remains priced at the schoolbook harness (U1) about 2.4x above the tuned price (red team); the like-for-like cell is U2 (0.0156 at w = 0)."
  - "The weak class only; says nothing about prime-order curves."
lean_obligation: "None (counting)."
closest_baseline:
  in_repo: "research/jv_sieve_negrho_20261102 (the validated row) and its validator report; research/jv_sieve_adds_20261103 (cut the adds, S+ x0.621; counters for enumeration and sieve adds exist)."
  nearest_sweep_lead: "LEAD-RHO-native-step-cost / LEAD-ICB-koblitz-setup-phase-cost are accounting leads on other arms; none covers this cover."
implementation_language: "Rust only."
```

## Lead 4

```yaml
id: LEAD-C-quartic-n4v2-k26
title: Plane-quartic double-large-prime index calculus (N4v2 DL) past the 64-bit group-order wall, k = 23-26
obstruction_attacked: >
  Open problem 9: the exponent 0.983 [0.975, 0.993] and "0.005x rho near q = 2^21" are L0, have no validation
  file, stop at q = 4194271 because "k > 22 needs 128-bit ell arithmetic in the builder and solver"
  (quartic_n4v2 README, Deviation 8), and the top sizes lean on a post hoc rho supplement. Implementation gap,
  not a mathematical obstruction.
claim: >
  With 128-bit group-order and scalar arithmetic in the builder and solver, N4v2 DL solves and verifies instances at
  q = largest prime below 2^k, q = 7 mod 12, k = 23, 24, 25, 26 (4 | #E_0(F_{q^3}), ell about 2^(3k-2)), with a fitted
  cost exponent in q over k = 17..26 of 0.97-1.05 in M-equivalents at w = 0, and DL / negation-rho at k = 26 of
  about 0.0009 [0.0006, 0.0014].
mechanism: >
  Diem's analysis gives O~(q) for plane-quartic index calculus with double large primes (Diem 2006, Diem-Thome 2008);
  rho on the cubic-extension group costs q^1.5 (measured 1.543 [1.494, 1.583]). The O(N) memory design of the S-family
  removed the earlier memory cliff (B0 needs 4.7 GB at k = 21). The extension tests whether the exponent persists
  past the u64 order and the lmul/mul step the README reports at ell ~ 2^50 (1.55 -> 2.37).
prediction:
  exponent_DL_k17_to_k26: "1.00 [0.95, 1.06] in M-eq at w = 0 (the k = 17..22 fit was 0.988)"
  ratio_to_rho_neg_k26: "0.00085 [0.0006, 0.0014]  (derived: 0.004 at k = 22 times 16^(0.983-1.543) = 0.212)"
  wall_DL_k26: "about 190-400 s (derived: 12.3 CPU-s at k = 22 times 16^0.983 = 187 s, up to 2x for wider arithmetic)"
  rho_reference: "calibrated model plus measured negation-rho runs at k <= 23 only (rho at k = 22 took 616 CPU-s per run, so k = 24 is about 5000 s and k = 26 about 12 h per run; those are isolab runs, not local)"
discriminating_test:
  local: >
    Step 1 (implementation, Rust): widen `ell`, scalars, packed keys and the Lanczos modulus to u128 in the N4v2 builder and
    solver (cf. problems/quartic-binary-ic-wide-scalars.md for the analogous binary gap). Step 2 (control): rerun k = 12..22
    and require records identical to the committed N4v2 records (the README's replay property). Step 3: DL at k = 23 and 24
    (about 25-50 s of IC each at the extrapolated scaling), seeds 1-3, every logarithm verified. Each job under 20 min on
    4 vCPU / 15 GB if the O(N) memory holds; read peak RSS first at k = 23.
  isolab_job: >
    k = 25, 26 DL with 3 seeds each, plus negation-rho runs at k = 23 and 24 (2-12 runs, as the README did for k = 19..22) on
    an isolated host with a replay certificate; this is also the validation the treatise lists as open under problem 9.
falsification: >
  Killed if the DL exponent over k = 22..26 has its lower 95% bound above 1.10, or DL / rho-neg at k = 26 has its upper
  bound above 0.01, or any unverified logarithm. A memory failure at k = 25 is an implementation limit, reported as such
  (a resource limit is not a refutation of the exponent).
row_moved: >
  N4v2 DL / DL1 row (L0, no validation file): top size from q = 2^22 to 2^26, exponent re-fitted over four more sizes,
  and the first replayable isolated row if the isolab job is run. Class: scale-up and validation of an existing result,
  not a new improvement; the brief's "priced result where only a local one exists".
cost_band:
  arms: "DL, DL1 (ratchet baseline), B0 not runnable past k = 21 (4.7 GB); negation rho (calibrated), plain rho"
  unpriced_counters: "rho-neg charges about three unpriced operations per step (canonicalisation, distinguished-point test, cycle check) against one for plain rho (README); IC: mod-ell inversions charged 60 lmul each, uncharged Pic relation checks excluded from phase times (README Deviations 2, 4); list them with counts for every arm"
  weights: "w = 0, 0.1, 1; README: rho-neg is the more expensive reference at w = 1, cheaper at w = 0 and 0.1"
risks:
  - "Pure implementation cost; the experiment is minutes once the arithmetic is wide."
  - "The rho calibration rule failed at k = 22 in the frozen runs; use the same rule and report exclusions."
  - "Family is narrow: GHS-weak E_0/F_{q^3} with 4 | #E_0, never prime order."
lean_obligation: "None for the measurement. Diem's O~(q) is a heuristic analysis; no Lean statement is claimed."
closest_baseline:
  published: "Diem-Thome 2008 (O~(q) heuristic, experiments claim faster than rho at small sizes); Laine-Lauter 2015 time-memory variant (abstract only read); Vitse-Wallet 2015 sieving for plane quartics (about 3x in relation search; not applied here)."
  in_repo: "research/quartic_n4_20261019, research/quartic_n4v2_20261020, research/plane_quartic_program_20261018/CLOSING.md."
  nearest_sweep_lead: "LEAD-TRF-odd-char-descent-smith-quartic (toy). It concerns reaching quartics through Smith's isogeny, not the scale of the direct route."
implementation_language: "Rust only (the repository's existing N4v2 tool)."
```

---

## Ranking

1. **Lead 1** `LEAD-C-jv-nagao-hilbert-at-scale`. The largest predicted measured improvement (1.85x in S on the Nagao arm that is the programme's closest-to-rho cover arm outside the sieve), the cheapest decisive test (counts only, deterministic, 9 runs of about 5 min, no new rho runs), a mechanism already validated at smaller `p`, and a prediction that would flip the U2 order at `p = 1451, 1601` (distance remaining `1.318 -> ~0.71`). It also supplies the carry-over to Lead 2.
2. **Lead 3** (priced emission): near-certain to confirm, which is its value; it is the explicit gap the validator and problem 11 name, and it is a precondition for admitting any sieve gain (Lead 2) to the board.
3. **Lead 2**: smaller (about 13 percent) on a row already `155x` below rho, but a pure engineering ratchet with a desk check first.
4. **Lead 4**: a scale-up and validation of an existing result, the only lead that needs real implementation work.

Leads 1 and 2 share a mechanism but make independent claims on different rows with separate falsifiers; they are not bundled. Lead 2's descent saving is only the F4 share of `S` (about 26-32 percent), so it is the weaker of the two.

## Considered and not proposed (with the reason)

- **Prime-field generic track (residual collectors, k-sum tables, pair tables).** At the generic floor: the validated `res1eq` is `1.54-1.64x` rho at `w = 0` on 36-37 bits (Table `tab:pf-residual-cells`), the derived collision floor for a collision-based method with `F` unknown base logarithms is `1.5x` (`F = 1`) and `1.875x` (`F = 2`), and the treatise itself notes the producer-reported `1.32` (p36) and `1.39` (p42) sit below the `F = 1` floor with no source stating the number of unknown base logarithms (prime_ext.tex, "What the collectors are"; one scale-study baseline run was censored at `10^8` trials). That is a **reconciliation** test (re-measure the p36-p44 ladder uncensored against matched rho-neg, local, minutes), not an improvement; it fits the sweep's rho-calibration family (`LEAD-RHO-matched-rho-calibration`) and I did not write it up as a lead.
- **Promote the k = 5 sum table (unqualified, `0.45` of the incumbent at 24 bits, `6.3x` rho-neg) to a Table B row.** An execution gap (second-host replay plus `VALIDATION.json`), nearest lead `LEAD-ICP-generic-table-decomposition`; no new mechanism.
- **Best-of-K isomorphic-model selection by a log-free score (isogeny-transport/coordinate effect, `tau ~ 4.7%` across models at 24 bits, `iso-volcano-v1`).** Derived arithmetic kills it: scoring a model costs about `X` Legendre symbols (`X ~ 200`, about `4e3` multiplications), K = 64 costs about `2.6e5` multiplications, against a solve of `2^15.5` GAE at fp24 and an expected best-of-64 yield gain of `2.4 sigma ~ 11%` on a relation phase that is 72% of cost; break-even at best. Nearest sweep lead `LEAD-STR-class-representative-ic-selection` (a 1.2% exceedance of a 20-sample maximum).
- **Isogeny walk to a weak prime-field curve.** Closed by Tate's theorem on prime fields (transfer_ext.tex `sec:iso-tate`); `LEAD-TRF-prime-field-isogeny-to-weak`.
- **Descent: early-abort before F4, or large-prime variation inside the six-point test.** Rejected on the arithmetic: the success probability `1/720` is the chance that the six roots all lie in `F_p` after the F4 solve has produced the solution, so no filter is available before the expensive step; and LEADS.md L7 records that a large prime is impossible in a six-point test. I did not find a cheaper necessary condition and did not invent one.
- **Plane quartic via Smith's isogeny, and the genus-3 hyperelliptic ladder past `q = 2^22`.** Both are already named open items (problem 9; problem 8, which also needs more than the 4.5 GB cap at `q = 2^23`); neither has a new mechanism to add here. `LEAD-TRF-odd-char-descent-smith-quartic` is the nearest sweep lead.
- **Extension fields `k = 4` (odd `p`, `V_4` line, E24-E42).** The solve is 99 percent of `S` and E39/E40 already took the pivot and matrix-entry levers; E21/E35 closed trace replay. I found no untested lever I could ground in a file.

## Process notes (Preserve scope and report evidence)

- Nothing was executed; every "local test" size is quoted from `results/walls.txt`, README timings or the validator report. The coordinator should have the experiment director confirm the build and timings before freezing a card.
- Derived numbers are marked "derived" with the arithmetic shown; they are predictions' inputs, not measurements.
- Sources for Lead 1 and 2's saving (`S_A/S_B` 1.848-1.875) are producer-reported with a passing validation gate for the Nagao arm at `p <= 503`; no `pcc` claim id was looked up, so they carry `[pcc:unverified]`.
- If Lead 1 stage 1 fails (F2 or F3), the obstruction to record is "Hilbert-stop saving does not transfer to `p >= 1301`"; I did not find a `problems/` record for the Nagao arm's U2 distance (the nearest are `problems/jv-cover-subbase-restricted-test.md` and `problems/release-suite-jv-nagao-wall-budget.md`), so the coordinator should file or extend one in that case.
