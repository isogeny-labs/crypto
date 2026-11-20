# Protocol: known-log generator-multiple columns on the small Koblitz rows (2026-11-18)

Frozen before any measured data (hashes in `REGISTRATION.sha256`, commit named in `registered-at.txt`). Class: **engineering
lever on the validated generator-orbit IC arm**. Every number is `[pcc:unverified]` until a validator signs it.

## 0. What was seen before freezing (disclosed)

A diagnostic pilot of the *incumbent* arm only (`gen-c1` n29, `gen-c3`/`gen-c4` n23, `gen-c8` n37, 4-6 targets, 1 round,
binary sha256 `b2275ffaa4774a661e7f2948c6cf1b54829649f81d6540fe2accd3f4cf6e2251`, scratchpad, not kept as evidence) gave the
phase split used to size the lever (mean GAE at w = 0):

| curve | arm | factor base | oracle set-up (pair table) | relations | linear algebra | verify | total |
|:--|:--|--:|--:|--:|--:|--:|--:|
| n29 (log2 r 15.4) | gen-c1 | 1 | 32 | 17 | 0.04 | 19 | 70 |
| n23 (22.0) | gen-c4 | 169 (15 scalar mults) | 398 | 690 | 0.3 | 33 | 1289 |
| n37 (27.8) | gen-c8 | 720 (29 scalar mults) | 2538 | 9489 | 0.4 | 40 | 12787 |

No candidate-arm data was produced before this freeze. Seeds below are new (not the pilot's, not the reference study's).

## 1. Lever (one parameter, default off)

`compact-orbit-scan:...,generator_orbit=1,generator_multiples=k`: columns `0..k` are the signed Frobenius orbits of
`[1]G, [2]G, ..., [k]G` (one doubling and `k-2` additions, charged to the factor-base phase), each with a known logarithm
`h*j`; the remaining `columns - k` columns come from the raw-x scan as before. Absent or `k = 1` is the incumbent base
byte-for-byte (unit test `generator_multiples_seed_known_log_columns_and_default_is_unchanged`; control arm `-m1` below).

Why it is cheaper (cost chain): a scanned column costs a cofactor projection, an order check and an eigenvalue check (3+
scalar multiplications) and needs its own relation; a known-log column costs one group operation and needs none. At
`k = columns` the relation search stops at the first relation (it pins the target), so it does `~1/c` of the incumbent's
relation trials, and the pair table can be sized for one relation (smaller `c`). Expected (model from the pilot
above, not a measurement): n23 kl-c3 ~ 0.4-0.5x gen-c4, n37 kl-c7 ~ 0.25-0.3x gen-c8, n29 no gain (gen-c1 is already
the all-known `k = 1` base).

What it is (named so nobody mistakes it): with every column of known logarithm, the pipeline is a Frobenius- and
negation-folded baby-step giant-step (pair sums of known multiples as baby steps, `Q + [t]G` giant steps), which is what
the gen-orbit README already says of `c = 1`. Its asymptotic cost is `~2 sqrt(r / 2n)` group operations against
`~sqrt(pi r / 4n)` for a folded rho: this lever cannot pass a matched folded rho asymptotically (about 1.6x above), and it
is no evidence for a smaller exponent. It is a T0-scope engineering gain on the named IC baseline.

## 2. Arms (one ecbench session per curve, same binary, 48 public targets, 3 rounds, 1 warm-up, alternating order)

| arm | method | role |
|:--|:--|:--|
| rho-strong | rho.signed_frobenius_strong | leaderboard's current matched reference |
| rho-v5 | rho.signed_frobenius_v5 | candidate reference (owner decision pending); reported beside strong |
| gen-c<c0>, gen-c<c0>-aa | `compact-orbit-scan:columns=c0,raw_x_cap=1e8,generator_orbit=1` + `mitm-frobenius-layout-counted:m=2`, `targets=stride-generator` | incumbent (validated, research/ecbench_generator_orbit_20261022); A/A |
| gen-c<c0>-m1 | the same with `generator_multiples=1` | default-off identity control: must equal gen-c<c0> record for record |
| kl-c<k> | `columns=k, generator_multiples=k` (all known-log), same oracle and targets | candidate |

| curve (a, n) | slug, log2 r | c0 | primary candidate | secondary | target seed |
|:--|:--|--:|:--|:--|--:|
| (1, 23) | icv1-f2m23-tm5197-1f85e9e1, 22.00 | 4 (validator's pre-declared test) | kl-c3 | kl-c4; gen-c3 | 20261118401 |
| (1, 29) | icv1-f2m29-tm40309-30c52b96, 15.37 | 1 | none (lever inert: gen-c1 is k = 1) | kl-c2 | 20261118402 |
| (0, 37) | icv1-f2m37-tm534059-32aad96b, 27.78 | 8 | kl-c7 | kl-c6, kl-c8 | 20261118403 |

Specs: `make_specs.py` -> `specs/n{23,29,37}.json`; run: `run.sh` (cpu lock, `--cpus none --allow-busy`, L0) with `ECBENCH` pinned to a copy of the binary built from commit 1b8e9e5cde (sha256 `c95535f432ed170235211a289d31d7bd8a3cfb318351b5b8dbe08768d3d48330`; the shared `target/` is rebuilt by another lane; the tree also held that lane's uncommitted test-only edit in `ic_boundary.rs` `ksum_wide_index_tests`, which no binary path reads). Analysis: `analyze.py`.

## 3. Pricing, statistics, decision

`ecbench verify --replay-all` per session; `ecbench band --ranks` at w = 0, 0.1, 1 GAE per unpriced native operation
(table work priced, the default); `ecbench compare --weight w --resamples 4000 --seed 20261002`, paired by target,
cluster bootstrap 95%.

- **P1 (primary, n23 and n37):** kl-c<k*> / gen-c<c0>. Improvement at a curve iff the 95% upper end is below 1 at all three
  weights. Multiplicity (two primary tests): a curve whose 95% upper end lies in [0.90, 1) at any weight is reported
  "not resolved under multiplicity" instead of an improvement.
- **P2 (gap):** kl-c<k*> / rho-strong and kl-c<k*> / rho-v5 at the three weights, with the incumbent's same ratios
  before; reported as distance remaining, never as a gate.
- **Controls:** gen-c<c0>-m1 equals gen-c<c0> on every record's total GAE (else the lever is not default-off and the
  session is void); A/A gen/gen-aa ratio contains 1; every solve verified.
- **Secondary (descriptive):** other kl-c<k> columns (sensitivity of k), gen-c3 at n23, n29 kl-c2 / gen-c1 (predicted >= 1).
  Phase split per arm (factor base, oracle set-up, relations, verify).

## 4. Deviations declared in advance

- No EKS/Iso Lab and no S3 in this container (AWS token rejected; checkpoint token expired): local L0 run, raw
  sessions kept in the scratchpad with their SHA-256 in `sessions.sha256`; compact per-record tables in git.
- CPU budget: <= 3 CPU-hours total for this study.
- `rho.signed_frobenius_v5` leaves its per-step jump-index selection untallied (koblitz_rho_reference_20261115 §1);
  the v5 distance is therefore slightly optimistic for rho at w > 0.

## 5. Non-claims

No below-rho claim, no exponent claim, no change to the leaderboard's rho reference. The lever is a generic-group
(BSGS-type) change to the IC arm's base and is reported as such.

## Addendum A1: spread known-log seeds (written 2026-11-18T06:27:06Z, after the frozen sessions and before any A1 data)

Seen before writing this addendum: all frozen-session results (n23 primary kl-c3/gen-c4 1.11 [0.87, 1.42] at w = 0, null;
n37 primary kl-c7/gen-c8 0.49 [0.39, 0.62] at w = 0, an improvement at all three weights; n29 kl-c2 1.70x, worse) and the
diagnosis: the progression seeds [1]G..[k]G give fewer distinct pair sums (n23 c4: 260 table entries against 356 for the
scanned base) and their sums cluster along the stride-G target walk (1051 trials for one relation against 282 per relation
on the incumbent). Lever A1: `multiple_doublings=d` makes the known seeds `[2^(d(j-1))]G` (d doublings each, no addition).
A sizing pilot (12 targets, 1 round, seeds 20261118499/599, binary below, scratchpad, not evidence) gave mean GAE: n23 gen-c4
1505, kl-c4 1568, sp-c4-d2 1461, sp-c4-d3 876, sp-c4-d5 932, sp-c3-d3 1598, sp-c5-d3 986; n37 gen-c8 10288, kl-c8 6278,
sp-c8-d3 5118, sp-c7-d3 5683, sp-c6-d3 9898; n29 gen-c1 110, sp-c2-d3 158 (so n29 is not in A1: the lever stays inert there).

Binary: `klc-ecbench-a1`, built from the working tree that commit (next) records, sha256
`58cfff9049e0399045e8b0b0d723b8b2faac379a848c0ed4ffd4e1ca87007f26`. Specs `a1/make_specs.py` -> `a1/specs/n{23,37}.json`
(48 new public targets, seeds 20261118411 / 20261118413, 3 rounds, 1 warm-up); run `a1/run.sh`; analysis `a1/analyze.py`.

Arms per curve (c0 = 4 at n23, 8 at n37): rho-strong, rho-v5, gen-c<c0> (validated incumbent), gen-c<c0>-aa, kl-c<c0>
(A0 progression, the off-state of `multiple_doublings`), **sp-c<c0>-d5 (primary)**, sp-c<c0>-d3, sp-c<c0+1>-d5.

- **A1-P1 (primary):** sp-c<c0>-d5 / gen-c<c0> at n23 and n37, the section 3 decision rule (95% upper end below 1 at
  w = 0, 0.1 and 1; an upper end in [0.90, 1) is "not resolved under multiplicity").
- **A1-P2 (ratchet):** sp-c<c0>-d5 / kl-c<c0>, same rule; the A0 kl arm is the off-state, so this isolates the spread seeds.
- **A1-P3 (gap):** sp arms / rho-strong and / rho-v5 at the three weights.
- Descriptive: d = 3, c0 + 1, table entries and trials per relation.
- Predicted from the pilot: n23 ~0.6x gen-c4, n37 ~0.5x gen-c8 (pilot: 12 targets, noisy). A1 is the thread's second attempt
  on these curves (A0 primary at n23 failed).
