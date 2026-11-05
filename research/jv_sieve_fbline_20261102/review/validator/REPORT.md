<!-- Written by the main session from the validator's returned text, verbatim apart from this line: the validator's harness blocked it from writing this file. -->
# Validator report VAL-20261103-594245: factor-base line search on the JV sieve route

Verdict: passed. The receipts are admissible for the study's frozen scope, a paired
comparison of the line-search arms against the plain sieve route. One caveat touches a
claim: arm T's S+ and §5a band are lower bounds (see "Finding").
Validation gate: not admissible. No VALIDATION.json was written. The protocol names no
rho reference (list below).

- Snapshot: main aa1c89ab2a, merged via PR #3342. The study files are clean against HEAD.
- Scope: the weak class only (full rational 2-torsion, #E = 4l, genus-3 cover over
  F_{p^2}), at p in {1009, 1511, 1823}, seeds 17-32.
- Checker: check/src/main.rs, written in Rust from PROTOCOL.md and the record schema. It
  shares no code with fit/: it has its own splitmix64 bootstrap, type-7 quantiles and
  normal-maximum quadrature.
- Output: check_output.txt and recompute.json. Both rerun byte-identically from replay/.

## What was checked
- Presence: 21 JSON and 21 log files. Each arm file holds 16 records (seeds 17-32);
  each control file holds 1 (seed 17).
- Freeze hashes: sha256sum -c passes on FROZEN.sha (PROTOCOL.md, run.sh, rule.txt) and
  on FIT.sha.
- Freeze ordering:
  - FROZEN.sha (19:29:06Z) is clock-only.
  - The first git record is 73582ce306 (19:37:29Z). The frozen files are
    byte-identical to the final ones.
  - Records present at that commit: P_1009 (16), H_1009 (16), T_1009 (6).
  - The wall times are consistent with the run starting at the freeze.
  - The design was already in LEADS.md at 3764c93e98 (18:50:54Z).
- Revision and dirty tree: the jv_cover.rs and jv_sieve.rs blobs recorded at freeze are
  the committed ones. On main the only later change is a div_ceil edit, identical for
  odd p.
- C4 receipts: 201/201 runs solved and correct, 0 failed verifications, none exhausted,
  none timed out.
- Phase sums: exact integer identities on every run (relation, descent, total,
  la = 16 la_ops). total_plus = muls + sieve adds + sieve lookups + enum adds + search
  adds + search lookups.
- Pair identity: l, c_add_e (= 331) and seed equal on all 144 arm/P pairs.
- Search counters: exact on all 144 rows.
  - Horner: 15pn adds, pn lookups.
  - Table: 29q + 2pn adds, q + pn lookups, 38q table multiplications.
  - Multiplications above the counted floor are exactly 341 per extra Horner candidate
    (line_poly) plus 374 for the move of G and Q.
  - The exhaustive arm adds 17 per direction with a0 != 0.
- C1: P equals V2's sparse record in every field except wall_ms, 48/48, with equal key
  sets.
- C2h, C2t: d(total_muls) equals the search's multiplications exactly (Horner
  17,662 / 26,447 / 31,907; table 38,687,587 / 86,759,358 / 126,287,418).
  d(total_plus) = muls + adds + lookups. The route fields are unchanged.
- C3: table and Horner at n = 4096 choose the same line (511->554 idx 642,
  768->824 idx 3928, 924->997 idx 1306), with equal route multiplications.
- C5: b_plain = P's base and b_best = the arm's base on 144/144 pairs.
- Replays (main build d21002a3, seeds drawn from a nonce after publication):
  - H p=1511 seed 27, U seed 21, P seed 32 and T seed 18 are identical to the
    committed records in every field except wall_ms.
  - P seed 32 is also identical to V2's record.

## Code review
- The moved model is used everywhere. run_cover_sieve_dlp_on shadows spec with the
  moved spec before any context is built, so every later use sees it:
  - Ctx::new and Ctx::with_trace apply spec.line.
  - The factor base, the Jacobian, gj/qj, sieve_setup, sieve_batch (its own
    Ctx::new(spec)), the descent (Ctx::with_trace(&spec), nagao_decompose, verify_dec)
    and the final check on E all use it.
  - No generate_spec call remains in the route.
  - End to end, all 144 moved runs recover the planted logarithm. A mixed model would
    not.
- The chi-table charging matches the executed code. Per point, chi_table performs 7
  Horner steps (5 products and 4 additions each), a norm (3 products, 1 addition) and
  one lookup: the charged 38 multiplications, 29 additions and 1 lookup. table_count
  performs 2 additions and 1 lookup per point.
  - Uncounted, and declared: the index arithmetic x0*p + x1 of each lookup, the trivial
    offsets a'*u with a' in {1, i}, and loop control.
  - Over-counted: the plain line is walked twice in exhaustive mode.
- P is bit-identical. With JV_FB_LINE unset, fb_line_from_env returns the cloned spec
  and None, so both line terms are 0 and fb_line is not serialized. C1 and the
  main-build replay confirm it.

## Recomputation (point estimates exact; intervals use an independent bootstrap stream)
- H S pooled: claimed 0.623 [0.562, 0.689]; recomputed 0.6230 [0.5632, 0.6882].
- U S+ pooled: claimed 0.525 [0.491, 0.561]; recomputed 0.5249 [0.4914, 0.5602].
- T S / S+ pooled: claimed 0.560 / 0.800; recomputed 0.5604 [0.4994, 0.6320] /
  0.7997 [0.7505, 0.8529].
- H S per prime: claimed 0.495 / 0.644 / 0.759; recomputed 0.4949 / 0.6440 / 0.7587.
- Band pooled at w = 0.1 (H / T / U): claimed 0.528 / 0.876 / 0.497; recomputed
  0.5279 / 0.8757 / 0.4973.
- analysis.txt: all 60 point estimates within half a unit of the last printed decimal;
  interval endpoints within 0.0036.
- README absolute S and S+, phase shares, search shares and B-model ratios
  (0.930 / 0.973 / 0.962): all match.
- gamma_hat for H = 5.915. The producer's interval is an unstratified bootstrap, the
  validator's is by prime; both contain 5.
- rule.txt recomputed from V2's means: k(4096) = 3.626. w = 0: n_H = 4096 (J 0.6554),
  n_T = exhaustive (J 0.5642). w = 0.1 and 1: n_H = 8192, n_T = 32768. Every per-prime
  J equal to 4 decimals.

## Finding: arm T's S+ and band rest on uncounted index arithmetic
The protocol calls the index arithmetic x0*p + x1 (one integer multiplication and one
addition per lookup) negligible. That holds for U, but not for T, where the line-walk
lookups (p*L) are a third of T's native operations.
- T, S+: as frozen 0.800; priced at 2 operations per lookup 1.016 [0.956, 1.082]
  (1.447 at p = 1823).
- T, band at w = 0.1: as frozen 0.876; priced 1.164 [1.101, 1.231].
- U, S+: 0.525 -> 0.532. U, band at w = 0.1: 0.497 -> 0.507.
- H has no index arithmetic and is unaffected. T's S (0.560) counts multiplications
  only and is unaffected.
- Under §5a, T's band order against P is unresolved, not "pooled yes". This does not
  change the producer's reading that U is the arm to use.

## Gate admissibility: missing elements
1. No named rho reference. PROTOCOL.md says "No rho is run" and "there is no
   VALIDATION.json". The README's distances to rho use V2's R3 reference, which V2
   withheld because A4 failed.
2. No matched rho at p = 1009 or p = 1823. Study B measured only p = 1511, seeds 17-24.
3. No frozen arm-vs-rho comparison with the §5a band. LEADS.md's "Combined" multiplies
   two ratios computed on different seed sets; dividing directly, H S/rho U1 is 0.00454
   on seeds 17-24, against the product 0.0042.
Present, and reusable by a later round: replay-certificate.json (sampled) and
validator-report.json (status pass, 60 headline numbers).

## Limitations
- Uncharged on the moved arms: the moved model's line_poly in every Ctx (341
  multiplications each), estimated at most 2.7e-4 of total_muls.
- Memory is not reported. The chi table takes q bytes (1.0 / 2.3 / 3.3 MB).
- The producer binary (68f23f40) is unavailable; the main build replays its records.
- The run was local, not on isolab-eks. The counted metrics are host-independent.
- LEADS.md says only P and H existed at 73582ce306; T_1009 was in fact 6/16 done.
- Rounding slips in the prose: README "x1.91" for U S+ should be x1.90; README T
  p=1009 "0.488" should be 0.487; LEADS.md "T (x1.79)" should be x1.78.
