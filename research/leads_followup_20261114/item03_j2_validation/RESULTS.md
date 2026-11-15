# RESULTS: independent validation of J2 (c_D = 1.60 vs the frozen 2.00, N4v2 DL1)

Validator, 2026-11-14 23:38 UTC to 2026-11-15 00:45 UTC. Protocol: `PROTOCOL.md` (written before data, amendment at its end).
Every number is `[pcc:unverified]`; the validator files are **unsigned** (no keyring signature in this session).
Verdict: `insufficient_evidence` for the improvement claim; the measurement itself reproduces. Report: `VALIDATOR_REPORT.md`,
gate files `replay-certificate.json`, `validator-report.json`, `VALIDATION.json` (the gate script lists the open reasons).

## 0. Setup, versions, hashes

| item | value |
|---|---|
| validated commit | `44d35033fa06b9796c6a4bf7696056d7a6e595a6` (`git -C /home/user/crypto rev-parse HEAD`, branch claude/cool-davinci-szm3om); producer paths clean in git (`git status --porcelain` empty for them) |
| other repos | ml-cryptanalysis, cryptanalysis: not used in this item |
| frozen manifest | `FROZEN.sha256` (440 files plus a header line, sha256 `5813c744...` of the manifest itself, written before data) |
| host | Linux 6.18.44 x86_64, Intel Xeon @ 2.10 GHz, 4 vCPU shared with other agents (L0, unisolated); at most 2 threads, `RLIMIT_AS` 6,000,000 KiB per solver run, `nice 5` |
| toolchain | rustc 1.97.0 (2d8144b78 2026-07-07); gcc 13.3.0; Python 3.13.18, numpy 2.5.3 (regression only) |
| producer binary | `j2/v2.bin` sha256 `183810cf9e1f9ed8ecaedf0295d3b8080528418bb208a71bcabbc3eb03d79bb1` (= BUILD.txt = `.artifacts.json`; store copy fetched, same hash) |
| N4v2 raw units | `units.tar.gz` sha256 `aac9a25ce50ac5d23895f0dba8f907fb223aaac32a2a41391bc590e3c6bcca42` (store GET into the scratchpad, equals manifest; 382,105 bytes) |
| J2 raw tarball | `j2/raw-units.tar.gz` sha256 `4a60e8b9e8eb8488e01d111120be6b20de92ed9b9e7eb59022ac995962b252d2` (= bench record; 238 members, all equal the committed files; the store holds it too) |
| validator code | `tools/` (sha256 prefixes in the table of section 9) |

Measured (a solver counter, a recorded time, a hash) versus derived (a price, a ratio, a regression) is marked in each section.

## 1. V0 provenance (pass)

Evidence `evidence/v0_v8.json`. The commit, the clean tree, the binary hash chain, the raw tarball against the bench record and [pcc:unverified]
against the 238 committed files, the N4v2 units tarball against its manifest and `instances.jsonl` against `instances.sha256`
all agree. The N4v2 sources match the addendum's source hashes except `v2.rs` and `summ2.rs` (README deviation 7: `homcheck`
and summarizer changes after the grid); V3 below shows the D-family counters unchanged.
The bench record says `"uploaded": false` for the raw tarball (2026-11-12); the object is in the store now (probe GET, hash verified).

## 2. V1 rebuild (pass)

`cd research/quartic_n4v2_20261020 && rustc --edition 2021 -O v2.rs -o $SCRATCH/build/v2_A` gives
`183810cf9e1f9ed8ecaedf0295d3b8080528418bb208a71bcabbc3eb03d79bb1`: the producer's binary, bit for bit.
`rustc ... research/quartic_n4v2_20261020/v2.rs` from the repository root gives `2fbc75eb...`, and with the absolute path
`a093b418...`: the binary embeds the source path, so only the source-directory invocation reproduces the hash.
The binary therefore *is* the producer's; the "replay with the producer's `v2.bin`" fallback of the protocol was not needed.

## 3. V2 replay: 84 of 84 units identical (pass)

`tools/replay_worker.py` ran the producer's own command, `v2 ic <q> <seed> D 3.0 <c> 4 4`, for all 54 grid units and all 30
held-out units (the six mandated ones first: q=2097091 seed 4 at c=1.60 and 2.00, seed 5 at 1.75; q=4194271 seed 4 at 1.60 and
2.00, seed 5 at 1.75), 84 runs, 84 return codes 0, 27.5 minutes of one thread (23:47:14Z to 00:14:43Z). `tools/replay_compare.py` compared every record
field by field (`tools/compare.py`: ignores only keys ending `wall`/`cpu`, `load1`, `search_s`, `s.controls_*`, and r_inv/r_lmul of the
`ratios` record). Result (`evidence/replay_compare.json`): **84/84 identical, 27,612 field comparisons** (counters, `s.*` statistics,
k, k_found, ok, attempt, c, ell, order, gae_mul, gae_inv). The producer's own A/A repeat equals its first run and equals the validator
replay of the same unit. Negative controls of the comparator (`evidence/tool_controls.txt`): a counter +1, a k_found +1, a ladd -1, a
dropped record are all detected; a timing-only change is ignored. The 4 units produced before the binary vanished and the 26 reruns
(`failed-binary-missing/` holds 26 empty outputs with "No such file or directory") replay identically, so the rerun binary was the same.

## 4. V3 control the producer's protocol required: c = 2.00 reproduces the committed N4v2 records (pass)

`tools/v3_compare.py`, `evidence/v3_compare.json`: for q in 65,479, 262,111, 1,048,507, 4,194,271 and seeds 1-3, the D, DL1 and DL
records of J2's `ic_<q>_<seed>_c2.00` equal the N4v2 grid raw units (`ic_<q>_<seed>_SD`, rustc 1.93.1, Apple M4 Pro, 2026-10-25)
in 293 fields each (12 of 12); ell and order equal `instances.jsonl`. RESULTS.md reports only the A/A repeat for this control.

## 5. V4 certificates re-verified with independent code (pass)

`tools/certcheck.c` (own xorshift64*, F_q, F_q^3 = F_q[x]/(x^3 - c), Weierstrass group law, Hasse-interval baby-step giant-step,
deterministic Miller-Rabin; the producer's `ghs.rs` read only as the written construction). For all 22 instances (q in 65,479,
262,111, 1,048,507, 2,097,091, 4,194,271; grid seeds 1-3 and held-out seeds 4-8) it re-derives E0, finds `order = 4 ell` with
`ell` prime, a point G with `ell G = O`, the planted k from the seed, and checks `k_found G = k G` on E0 for every ok record, plus two
negative-control logs per instance (k_found + 1, k_found xor 2^20) that must be rejected. **22/22 pass**; the recorded ell and order
are reproduced; the tries until the match equal the committed N4v2 `tries` for the 12 instances of `instances.jsonl` that J2 uses (seeds 1-3 at four sizes); the planted k equals the
record's k; j(E0) is not in F_q and not 0 or 1728, so negation is the only automorphism. Controls (`evidence/tool_controls.txt`): a wrong
ell, order or planted k is rejected. Scope: this certifies the instance, the group and the recovered log of each unit (`k_found` is
computed from the descent logs, not copied from k: `solver2.rs` `downstream`); it does not re-derive the plane-quartic model or the relation
sets (the solver's own sampled checks, `relations_bad` and `holdout_bad`, are 0 in all 252 arm records but are solver-side).

## T7 independent certificates (own C code), 22 instances

| q | seed | tries until match (committed N4v2 tries) | ell | order = 4 ell | k planted (from seed) | recorded k_found checked | j in F_q | pass |
|---|---|---|---|---|---|---|---|---|
| 65,479 | 1 | 2 (2) | 70185295384741 | 280741181538964 | 29244505750785 | 1/1 real + 2 negative controls rejected | false | PASS |
| 65,479 | 2 | 125 (125) | 70185290997949 | 280741163991796 | 61099871129250 | 1/1 real + 2 negative controls rejected | false | PASS |
| 65,479 | 3 | 131 (131) | 70185299665991 | 280741198663964 | 43733770945000 | 1/1 real + 2 negative controls rejected | false | PASS |
| 262,111 | 1 | 13 (13) | 4501899029752451 | 18007596119009804 | 1398150763278345 | 1/1 real + 2 negative controls rejected | false | PASS |
| 262,111 | 2 | 26 (26) | 4501899037171043 | 18007596148684172 | 398044013317672 | 1/1 real + 2 negative controls rejected | false | PASS |
| 262,111 | 3 | 159 (159) | 4501899087666733 | 18007596350666932 | 1454878235966216 | 1/1 real + 2 negative controls rejected | false | PASS |
| 1,048,507 | 1 | 52 (52) | 288173479903168577 | 1152693919612674308 | 196688052282590435 | 1/1 real + 2 negative controls rejected | false | PASS |
| 1,048,507 | 2 | 36 (36) | 288173480515744919 | 1152693922062979676 | 19678715313697470 | 1/1 real + 2 negative controls rejected | false | PASS |
| 1,048,507 | 3 | 67 (67) | 288173480364596809 | 1152693921458387236 | 128437213181964192 | 1/1 real + 2 negative controls rejected | false | PASS |
| 2,097,091 | 4 | 8 (n/a, seed not in N4v2 list) | 2305641804149486501 | 9222567216597946004 | 1483201808580115546 | 1/1 real + 2 negative controls rejected | false | PASS |
| 2,097,091 | 5 | 22 (n/a, seed not in N4v2 list) | 2305641804999265121 | 9222567219997060484 | 144186910555077430 | 1/1 real + 2 negative controls rejected | false | PASS |
| 2,097,091 | 6 | 76 (n/a, seed not in N4v2 list) | 2305641804312339217 | 9222567217249356868 | 475117378766834864 | 1/1 real + 2 negative controls rejected | false | PASS |
| 2,097,091 | 7 | 19 (n/a, seed not in N4v2 list) | 2305641803271239761 | 9222567213084959044 | 2300285555431123261 | 1/1 real + 2 negative controls rejected | false | PASS |
| 2,097,091 | 8 | 234 (n/a, seed not in N4v2 list) | 2305641805645570837 | 9222567222582283348 | 1370537200301052163 | 1/1 real + 2 negative controls rejected | false | PASS |
| 4,194,271 | 1 | 235 (235) | 18446308672764918653 | 73785234691059674612 | 5881832086521072675 | 1/1 real + 2 negative controls rejected | false | PASS |
| 4,194,271 | 2 | 48 (48) | 18446308671982004833 | 73785234687928019332 | 10989533593747831194 | 1/1 real + 2 negative controls rejected | false | PASS |
| 4,194,271 | 3 | 46 (46) | 18446308672317154117 | 73785234689268616468 | 3436403882125682732 | 1/1 real + 2 negative controls rejected | false | PASS |
| 4,194,271 | 4 | 46 (n/a, seed not in N4v2 list) | 18446308669096152463 | 73785234676384609852 | 17427032051851184066 | 1/1 real + 2 negative controls rejected | false | PASS |
| 4,194,271 | 5 | 265 (n/a, seed not in N4v2 list) | 18446308669144905401 | 73785234676579621604 | 11369057431138369548 | 1/1 real + 2 negative controls rejected | false | PASS |
| 4,194,271 | 6 | 70 (n/a, seed not in N4v2 list) | 18446308669282100851 | 73785234677128403404 | 9560562580629128557 | 1/1 real + 2 negative controls rejected | false | PASS |
| 4,194,271 | 7 | 29 (n/a, seed not in N4v2 list) | 18446308669181047661 | 73785234676724190644 | 10574231355505449283 | 1/1 real + 2 negative controls rejected | false | PASS |
| 4,194,271 | 8 | 137 (n/a, seed not in N4v2 list) | 18446308670389649893 | 73785234681558599572 | 15480408942030619923 | 1/1 real + 2 negative controls rejected | false | PASS |

## 6. V5 recompute from raw counters (pass), and V7 success counts

Own code, `tools/recompute.py`. Pricing: M-eq per record = sum over the 7 phases of `mul + r_inv inv + r_lmul lmul`, every attempt of the arm
summed, r_inv and r_lmul the medians of the three `v2 calib` records of the size (J2's own: q=2097091 from `heldout/`, q=4194271 from
`units/`), unpriced hash + cmp at w GAE with GAE = gae_mul + r_inv gae_inv. J2 calibration medians: r_inv 23.00 / 17.20 / 21.93 /
24.23 / 25.32 and r_lmul 1.210 / 0.832 / 0.979 / 1.010 / 1.027 at q = 65,479 / 262,111 / 1,048,507 / 2,097,091 / 4,194,271.
Fresh `v2 calib` here (`evidence/calib_recheck/`): r_inv 23.60 and 24.83, r_lmul 1.0106 and 1.0275 at q = 2,097,091 and 4,194,271, within 3% of the
recorded medians. Cross-check of the pricing code against committed numbers: with the N4v2 calibration my DL1, D and DL means at every size
equal `summary_supp.json` to at most 3.5e-7 relative (print precision of the committed file).

## T1 first-attempt success and paired DL1 cost ratio to c = 2.00 (M-eq, recomputed, w = 0; min-max over seeds)

| k (q) | set | seeds | c | first-attempt success | attempts per seed | ratio w=0 | ratio w=0.1 | ratio w=1 |
|---|---|---|---|---|---|---|---|---|
| 16 (65,479) | grid | 1-3 | 1.40 | 0/3 | 2,2,2 | 1.223-1.461 | 1.198-1.378 | 1.144-1.202 |
| 16 (65,479) | grid | 1-3 | 1.50 | 0/3 | 2,2,2 | 1.267-1.353 | 1.268-1.332 | 1.270-1.289 |
| 16 (65,479) | grid | 1-3 | 1.60 | 3/3 | 1,1,1 | 0.869-1.119 | 0.815-1.003 | 0.702-0.760 |
| 16 (65,479) | grid | 1-3 | 1.75 | 3/3 | 1,1,1 | 0.907-1.003 | 0.874-0.947 | 0.806-0.828 |
| 16 (65,479) | grid | 1-3 | 2.00 | 3/3 | 1,1,1 | 1.000-1.000 | 1.000-1.000 | 1.000-1.000 |
| 18 (262,111) | grid | 1-3 | 1.40 | 0/3 | 2,2,2 | 1.128-1.138 | 1.124-1.131 | 1.115-1.117 |
| 18 (262,111) | grid | 1-3 | 1.50 | 0/3 | 2,2,2 | 1.213-1.260 | 1.227-1.263 | 1.257-1.268 |
| 18 (262,111) | grid | 1-3 | 1.60 | 3/3 | 1,1,1 | 0.720-0.798 | 0.699-0.758 | 0.656-0.675 |
| 18 (262,111) | grid | 1-3 | 1.75 | 3/3 | 1,1,1 | 0.790-0.850 | 0.784-0.829 | 0.774-0.787 |
| 18 (262,111) | grid | 1-3 | 2.00 | 3/3 | 1,1,1 | 1.000-1.000 | 1.000-1.000 | 1.000-1.000 |
| 20 (1,048,507) | grid | 1-3 | 1.40 | 0/3 | 2,2,2 | 1.091-1.107 | 1.095-1.107 | 1.104-1.107 |
| 20 (1,048,507) | grid | 1-3 | 1.50 | 0/3 | 2,2,2 | 1.229-1.232 | 1.240-1.242 | 1.261-1.264 |
| 20 (1,048,507) | grid | 1-3 | 1.60 | 3/3 | 1,1,1 | 0.673-0.687 | 0.663-0.673 | 0.641-0.644 |
| 20 (1,048,507) | grid | 1-3 | 1.75 | 3/3 | 1,1,1 | 0.775-0.780 | 0.773-0.777 | 0.769-0.770 |
| 20 (1,048,507) | grid | 1-3 | 2.00 | 3/3 | 1,1,1 | 1.000-1.000 | 1.000-1.000 | 1.000-1.000 |
| 21 (2,097,091) | held-out | 4-8 | 1.60 | 5/5 | 1,1,1,1,1 | 0.660-0.685 | 0.652-0.671 | 0.636-0.641 |
| 21 (2,097,091) | held-out | 4-8 | 1.75 | 5/5 | 1,1,1,1,1 | 0.771-0.775 | 0.770-0.773 | 0.767-0.768 |
| 21 (2,097,091) | held-out | 4-8 | 2.00 | 5/5 | 1,1,1,1,1 | 1.000-1.000 | 1.000-1.000 | 1.000-1.000 |
| 22 (4,194,271) | grid | 1-3 | 1.60 | 3/3 | 1,1,1 | 0.653-0.658 | 0.647-0.650 | 0.633-0.634 |
| 22 (4,194,271) | grid | 1-3 | 1.75 | 3/3 | 1,1,1 | 0.771-0.772 | 0.770-0.770 | 0.767-0.767 |
| 22 (4,194,271) | grid | 1-3 | 2.00 | 3/3 | 1,1,1 | 1.000-1.000 | 1.000-1.000 | 1.000-1.000 |
| 22 (4,194,271) | held-out | 4-8 | 1.60 | 5/5 | 1,1,1,1,1 | 0.655-0.666 | 0.648-0.656 | 0.633-0.636 |
| 22 (4,194,271) | held-out | 4-8 | 1.75 | 5/5 | 1,1,1,1,1 | 0.771-0.773 | 0.770-0.771 | 0.767-0.767 |
| 22 (4,194,271) | held-out | 4-8 | 2.00 | 5/5 | 1,1,1,1,1 | 1.000-1.000 | 1.000-1.000 | 1.000-1.000 |

Comparison with the producer's printed numbers (`validator-report.json` `headline`, 38 entries): every cell of the RESULTS J2 table, the
held-out ranges (0.655-0.685 at c=1.60, 0.771-0.775 at c=1.75), the bench-record spreads, the success counts (the 36 first-attempt successes the grid table implies,
held-out 10/10 and 10/10) and `j2/summary.json` (18 cells, 3 decimals) are reproduced to their printed rounding. Three statements are not supported as
worded (findings F2, F3, F4): the threshold flip "at every k" (no c <= 1.50 data at k = 21, 22), "Seeds agree within 1% at k >= 20" (the c=1.60 ratio
spreads 0.8-3.8%; only the c=2.00 cost agrees within 1%), and "N4v2 DL1 about 0.005x matched negation rho" (the README's 0.005 is DL; DL1/rho-neg
is 0.0028 in `summary_supp.json`). The c_50 model range is reproduced (fixed point 1.486 to 1.557 against the printed 1.49 to 1.57).
Exact one-sided 95% lower bound (Clopper-Pearson, all successes) on first-attempt success at c=1.60: 0.873 over the 22 units, 0.741 over the 10 held-out units
(derived). At k = 16, 18, 20 the flip sits between 1.50 (0/3) and 1.60 (3/3); at k = 21, 22 only c_50 <= 1.60 is observed.

## 7. V6 the w band

## T2 per-seed order c=1.60 < 1.75 < 2.00 (seeds where it holds / seeds), by pricing reading

| k | set | w=0 frozen | w=0.1 frozen | w=1 frozen | w=0 strict | w=0.1 strict | w=1 strict |
|---|---|---|---|---|---|---|---|
| 16 | grid | 1/3 | 1/3 | 3/3 | 1/3 | 0/3 | 0/3 |
| 18 | grid | 3/3 | 3/3 | 3/3 | 3/3 | 0/3 | 0/3 |
| 20 | grid | 3/3 | 3/3 | 3/3 | 3/3 | 0/3 | 0/3 |
| 21 | held-out | 5/5 | 5/5 | 5/5 | 5/5 | 0/5 | 0/5 |
| 22 | grid | 3/3 | 3/3 | 3/3 | 3/3 | 0/3 | 0/3 |
| 22 | held-out | 5/5 | 5/5 | 5/5 | 5/5 | 0/5 | 0/5 |

Frozen N4v2 rule (unpriced = hash + cmp): the claimed order holds per seed at every w for k = 18, 20, 21, 22 (grid and held-out) and fails at
k = 16 for w = 0 and 0.1, exactly as claimed; it is unchanged by calibration constants r_inv in 8-30 and r_lmul in 1.0-2.5 (c=1.60 ratio at
k = 21, 22 stays 0.654-0.691).
Strict reading (the records also emit `add` and `ladd`, F_q and mod-ell additions, counted but unpriced by the frozen unit; AGENTS.md 5a asks for every
unpriced counter to be listed and priced): the order **fails at w = 0.1 and 1 at every size**; the c=1.60 ratio becomes 1.25-1.34 and c=1.75 1.02-1.06
at k = 21, 22. Price of an addition at which the c=1.60 gain disappears (derived):

## T3 price at which an addition (F_q add and mod-ell add, M-eq each) erases the c = 1.60 gain; ratio c1.60/c2.00 at selected prices

| k | set | break-even M-eq per addition | ratio at 0.05 | at 0.25 | at 0.5 | at 1.0 | at 2.0 |
|---|---|---|---|---|---|---|---|
| 18 | grid | 0.442 | 0.797 | 0.924 | 1.017 | 1.116 | 1.199 |
| 20 | grid | 0.536 | 0.734 | 0.880 | 0.988 | 1.104 | 1.202 |
| 21 | held-out | 0.589 | 0.724 | 0.868 | 0.973 | 1.085 | 1.178 |
| 22 | grid | 0.575 | 0.710 | 0.863 | 0.976 | 1.094 | 1.194 |
| 22 | held-out | 0.580 | 0.712 | 0.864 | 0.975 | 1.091 | 1.189 |

## 8. V8 held-out discipline (pass)

Amendment commit `0056acedae` (2026-11-12T04:54:39Z) precedes the first held-out launch (04:54:56Z, bench record) and the first completion (04:55:51Z, `grid.log`);
its header says "05:00 UTC" (a label error). Seeds 4-8 are absent from N4v2's instance list (seeds 1-3 at k >= 17) and from the J2 grid. The amendment's
predictions are met: first-attempt success 5/5 at c = 1.60 and 1.75 at both sizes; c=1.60 ratios 0.660-0.685 (k=21) and 0.655-0.666 (k=22) inside [0.60, 0.72];
c=1.75 ratios 0.771-0.775 and 0.771-0.773 inside [0.72, 0.82]; order at every w under the frozen rule. The 26 failed-binary-missing outputs are empty and
no number uses them. The k = 22 half is a seed hold-out only (the grid already contained k = 22 seeds 1-3 when the amendment was written); k = 21 is a new size.

## 9. V9 matched rho reference

N4v2 raw rho records (from `units.tar.gz`, plus `supp-k22-rho.tar.gz` for the k = 22 supplement): q = 2,097,091, 6 runs per arm (seeds 1-3, 2 runs each),
steps/sqrt(ell) 1.3415 (plain) and 1.1309 (neg), both inside the frozen calibration bands; q = 4,194,271, the frozen 2 runs fail the rule (1.6405 and 1.4486,
as the README says), the post-hoc 6-run supplement passes (1.2028 and 1.0309). My recompute of every rho mean equals `summary_supp.json` to at most 2.8e-7.

**Replay of rho in the rebuilt binary** (`tools/rho_replay.py`, the joblist's own commands, `evidence/rho_replay/`): q=65,479 seed 1 (16 runs x plain, neg), q=262,111 seed 1
(6 runs x 2 arms), q=2,097,091 seed 1 run 0 plain (2.46e9 steps) and neg (1.56e9 steps), q=4,194,271 seed 1 run 0 neg (6.29e9 steps): 5 job files, all records identical to
the N4v2 raw records, 1,027 field comparisons (steps, adds, dps, lookahead, cycles, cycle_adds, every counter). Not replayed: the other k = 21 and k = 22 runs (about 25 minutes of
one thread each).

Mean M-eq per solve, J2 calibration, frozen rule (derived; both arms priced with the same r_inv, r_lmul, GAE):

## T5 IC (DL1) vs rho, mean M-eq per solve, J2 calibration, frozen rule (unpriced = hash + cmp), w = 0 / 0.1 / 1 GAE

| k | arm | w=0 | w=0.1 | w=1 |
|---|---|---|---|---|
| 21 | DL1 c=1.60 | 2.246e+08 | 2.966e+08 | 9.449e+08 |
| 21 | DL1 c=1.75 | 2.582e+08 | 3.460e+08 | 1.136e+09 |
| 21 | DL1 c=2.00 | 3.340e+08 | 4.485e+08 | 1.479e+09 |
| 21 | rho-plain | 1.634e+11 | 1.798e+11 | 3.268e+11 |
| 21 | rho-neg | 1.426e+11 | 1.845e+11 | 5.610e+11 |
| 22 | DL1 c=1.60 | 4.491e+08 | 5.946e+08 | 1.904e+09 |
| 22 | DL1 c=1.75 | 5.269e+08 | 7.045e+08 | 2.303e+09 |
| 22 | DL1 c=2.00 | 6.821e+08 | 9.141e+08 | 3.002e+09 |
| 22 | rho-plain | 4.201e+11 | 4.621e+11 | 8.401e+11 |
| 22 | rho-neg | 3.728e+11 | 4.821e+11 | 1.466e+12 |

| k | DL1 c | vs rho-plain w=0/0.1/1 | vs rho-neg w=0/0.1/1 | vs rho-plain strict | vs rho-neg strict |
|---|---|---|---|---|---|
| 21 | 1.60 | 0.0014 / 0.0017 / 0.0029 | 0.0016 / 0.0016 / 0.0017 | 0.0014 / 0.0059 / 0.0066 | 0.0016 / 0.0066 / 0.0073 |
| 21 | 1.75 | 0.0016 / 0.0019 / 0.0035 | 0.0018 / 0.0019 / 0.0020 | 0.0016 / 0.0048 / 0.0053 | 0.0018 / 0.0053 / 0.0059 |
| 21 | 2.00 | 0.0020 / 0.0025 / 0.0045 | 0.0023 / 0.0024 / 0.0026 | 0.0020 / 0.0046 / 0.0051 | 0.0023 / 0.0052 / 0.0056 |
| 22 | 1.60 | 0.0011 / 0.0013 / 0.0023 | 0.0012 / 0.0012 / 0.0013 | 0.0011 / 0.0048 / 0.0054 | 0.0012 / 0.0053 / 0.0059 |
| 22 | 1.75 | 0.0013 / 0.0015 / 0.0027 | 0.0014 / 0.0015 / 0.0016 | 0.0013 / 0.0039 / 0.0043 | 0.0014 / 0.0043 / 0.0047 |
| 22 | 2.00 | 0.0016 / 0.0020 / 0.0036 | 0.0018 / 0.0019 / 0.0020 | 0.0016 / 0.0038 / 0.0041 | 0.0018 / 0.0041 / 0.0045 |

Unpriced counters of every arm (AGENTS.md 5a: list them with counts):

## T4 unpriced counters per arm (mean per solve, every attempt, all phases)

| arm | q | c | mul | inv | lmul | hash | cmp | add (F_q) | ladd (mod ell) |
|---|---|---|---|---|---|---|---|---|---|
| DL1 held-out seeds 4-8 | 2,097,091 | 1.60 | 1.742e+08 | 5.940e+04 | 4.854e+07 | 8.978e+06 | 0.000e+00 | 5.058e+07 | 7.040e+08 |
| DL1 held-out seeds 4-8 | 2,097,091 | 1.75 | 2.027e+08 | 3.097e+04 | 5.417e+07 | 1.094e+07 | 0.000e+00 | 5.467e+07 | 5.469e+08 |
| DL1 held-out seeds 4-8 | 2,097,091 | 2.00 | 2.637e+08 | 3.391e+04 | 6.873e+07 | 1.428e+07 | 0.000e+00 | 7.042e+07 | 4.985e+08 |
| DL1 held-out seeds 4-8 | 4,194,271 | 1.60 | 3.504e+08 | 5.613e+04 | 9.468e+07 | 1.790e+07 | 0.000e+00 | 9.246e+07 | 1.478e+09 |
| DL1 held-out seeds 4-8 | 4,194,271 | 1.75 | 4.158e+08 | 4.434e+04 | 1.070e+08 | 2.184e+07 | 0.000e+00 | 1.072e+08 | 1.138e+09 |
| DL1 held-out seeds 4-8 | 4,194,271 | 2.00 | 5.405e+08 | 4.129e+04 | 1.369e+08 | 2.853e+07 | 0.000e+00 | 1.374e+08 | 1.031e+09 |
| rho-plain (N4v2 raw, 6 runs) | 2,097,091 | - | - | - | - | 1.178e+02 | 2.037e+09 | 1.080e+11 | 4.074e+09 |
| rho-neg (N4v2 raw, 6 runs) | 2,097,091 | - | - | - | - | 1.093e+02 | 5.214e+09 | 9.424e+10 | 3.439e+09 |
| rho-plain (N4v2 raw, 6 runs) | 4,194,271 | - | - | - | - | 1.473e+02 | 5.166e+09 | 2.738e+11 | 1.033e+10 |
| rho-neg (N4v2 raw, 6 runs) | 4,194,271 | - | - | - | - | 1.335e+02 | 1.344e+10 | 2.430e+11 | 8.868e+09 |

(rho mul/inv/lmul per run: see evidence/recompute.json `rho`; every rho counter listed in the N4v2 raw records is emitted.)

Reading. DL1 at c=1.60 is 0.0011-0.0023 (k = 22) and 0.0014-0.0029 (k = 21) of either rho arm at w in {0, 0.1, 1} (0.0011-0.0073 under the strict reading): the order IC < rho
holds across the band by two to three orders of magnitude, the c=1.60 arm is about 0.66x the c=2.00 arm of that ratio in the frozen unit. rho is single-trail with one inversion per step (batched
inversion could lower its per-step cost from 56 + r_inv to about 56 + 3 M-eq, a factor below 1.4, derived). Gate fields: kind `negation` (j(E0) not in F_q, not 0 or 1728 for all 22 instances);
`matches_protocol` **false** (the J2 protocol names no rho reference; N4v2's protocol names rho-neg); `strongest_available` true in the symmetry sense; `same_unit` true;
`same_binary_or_process` **false** (rho: N4v2 `v2_grid`, rustc 1.93.1, Apple M4 Pro; c-sweep: `j2/v2.bin`, rustc 1.97.0, Xeon; counters replay identically across the two, but the arms did not run in one binary or process).
The k = 22 reference is the post-hoc supplement.

## 10. V11 does the gain survive measured time? (post-hoc check, fail)

The same solves, measured (CPU seconds of the process, from the records' `<phase>.cpu`; L0 host, so a time is practicality evidence and not a speed claim; counts stand at any isolation level): [pcc:unverified]

## T6 measured CPU and wall of the same DL1 solves, ratio c / c=2.00 (per-seed median [min-max]); L0 host

| source | k | set | c | total CPU | total wall | LA-phase CPU | relations-phase CPU |
|---|---|---|---|---|---|---|---|
| producer_records | 18 | grid | 1.60 | 1.128 [1.075-1.203] | 1.132 [1.078-1.208] | 1.491 [1.386-1.493] | 0.658 [0.588-0.886] |
| producer_records | 18 | grid | 1.75 | 0.989 [0.985-1.036] | 0.988 [0.984-1.029] | - | - |
| producer_records | 20 | grid | 1.60 | 1.152 [1.132-1.163] | 1.133 [1.097-1.134] | 1.520 [1.498-1.528] | 0.724 [0.662-0.732] |
| producer_records | 20 | grid | 1.75 | 1.002 [0.967-1.012] | 0.989 [0.918-1.011] | - | - |
| producer_records | 21 | held-out | 1.60 | 1.051 [1.025-1.133] | 1.044 [1.031-1.127] | 1.406 [1.348-1.508] | 0.664 [0.584-0.704] |
| producer_records | 21 | held-out | 1.75 | 0.974 [0.949-0.995] | 0.960 [0.926-0.997] | - | - |
| producer_records | 22 | grid | 1.60 | 0.980 [0.953-0.989] | 0.987 [0.973-1.003] | 1.413 [1.401-1.416] | 0.621 [0.618-0.643] |
| producer_records | 22 | grid | 1.75 | 1.002 [0.943-1.006] | 1.235 [1.112-1.238] | - | - |
| producer_records | 22 | held-out | 1.60 | 1.022 [0.953-1.116] | 1.010 [0.890-1.193] | 1.476 [1.339-1.556] | 0.648 [0.621-0.748] |
| producer_records | 22 | held-out | 1.75 | 0.945 [0.914-0.987] | 0.923 [0.861-0.979] | - | - |
| validator_replay | 18 | grid | 1.60 | 1.134 [1.122-1.187] | 1.146 [1.119-1.182] | 1.475 [1.394-1.552] | 0.682 [0.656-0.750] |
| validator_replay | 18 | grid | 1.75 | 0.985 [0.970-1.021] | 0.979 [0.968-1.019] | - | - |
| validator_replay | 20 | grid | 1.60 | 1.123 [1.054-1.135] | 1.113 [1.052-1.166] | 1.474 [1.363-1.544] | 0.675 [0.669-0.689] |
| validator_replay | 20 | grid | 1.75 | 0.986 [0.907-1.031] | 0.977 [0.901-1.034] | - | - |
| validator_replay | 21 | held-out | 1.60 | 1.065 [1.056-1.098] | 1.114 [1.023-1.190] | 1.419 [1.413-1.492] | 0.658 [0.581-0.707] |
| validator_replay | 21 | held-out | 1.75 | 0.964 [0.944-0.972] | 0.969 [0.935-0.993] | - | - |
| validator_replay | 22 | grid | 1.60 | 1.011 [0.999-1.062] | 1.009 [0.992-1.061] | 1.420 [1.412-1.476] | 0.673 [0.644-0.684] |
| validator_replay | 22 | grid | 1.75 | 0.932 [0.889-0.934] | 0.932 [0.881-0.935] | - | - |
| validator_replay | 22 | held-out | 1.60 | 1.004 [0.967-1.044] | 1.006 [0.956-1.046] | 1.436 [1.353-1.491] | 0.662 [0.654-0.684] |
| validator_replay | 22 | held-out | 1.75 | 0.944 [0.912-0.960] | 0.943 [0.910-0.961] | - | - |

Producer records and the validator replay agree. Total CPU of DL1 at c=1.60 against c=2.00 is 0.95-1.13 per seed at k = 21, 22 held-out (medians 1.00-1.07): no gain. The relations phase is cheaper (0.58-0.75x at k >= 20) and the
linear-algebra phase is dearer (1.34-1.56x at k = 18-22). Counted mod-ell additions in LA rise 1.38-1.49x at c=1.60, because the relation graph is closer to the percolation
threshold and the reduced matrix is denser (nnz 1.91-2.03x of c=2.00; k = 22 seed 4: 260,289 against 132,855, mean cycle-row weight 68.0 against 27.7). The frozen unit counts those additions and does not price them.
In-situ least squares of the LA-phase CPU on its counters over 64 DL1 attempt records (derived): 7.96 ns per counted ladd and 1.53 ns per lmul, R^2 0.993; against the median `v2 calib` F_q multiplication of 7.4 ns that is
1.08 M-eq per ladd and 0.21 M-eq per lmul, while `r_lmul` prices an lmul at about 1.0. c=1.75 measures 0.91-0.99 CPU at k = 21, 22 (medians 0.94-0.97). Figure: `evidence/ratio_vs_addition_price.svg`
(drawn by `tools/make_figure.py`; no SVG rasteriser is installed here, so it was not rendered or inspected visually).

## 11. V10 reporting contract (gap list)

One table and one unit: yes (the ratio to c = 2.00 in M-eq), but the J2 table has no absolute cost, no rho column (the distance remaining), no phase split, no unpriced-counter list (add and ladd are emitted and not listed), no
correctness column (the text says every unit recovers the planted log, which V4 confirms). The class "engineering" is asserted; AGENTS.md defines it as "S fell", and V11 does not show that. m = 83 and ECC2K-130 disclosure: not applicable (prime-field quartic family; no binary-field claim).
The c = 2.00 control against the committed records is required by the J2 protocol and was not reported (V3 supplies it, identical).

## 12. Findings (claims the artifacts do not support, ordered)

- **F1 (decisive for classification).** The 0.66-0.78x gains exist only in the frozen N4v2 M-eq unit, which leaves counted F_q and mod-ell additions unpriced. At c=1.60 those additions in LA rise 1.38-1.49x and measured CPU does not fall; the order [pcc:unverified]
  c=1.60 < 2.00 reverses once an addition costs more than 0.44-0.59 M-eq (the in-situ fit is 1.08 M-eq per ladd) and at the 5a weights 0.1 and 1 GAE. AGENTS.md: a headline count that falls while work moves where the headline does not
  look is relabelling; a ratio improved by dropping a cost from the budget does not count.
- F2. "Threshold between 1.50 and 1.60 at every k": no c <= 1.50 data at k = 21, 22.
- F3. "N4v2 DL1 about 0.005x matched negation rho": that is DL; DL1 is 0.0028 (c = 2.00, w = 0, k = 21).
- F4. "Seeds agree within 1% at k >= 20": true for the c=2.00 cost (0.07-0.40%), false for c=1.60 (cost spread 0.8-4.0%, ratio spread 0.8-3.8%).
- F5. The J2 protocol names no rho; the rho arm is another binary and host; the k = 22 reference is a post-hoc supplement.
- F6. The required c = 2.00 control against the committed records is not reported (V3: identical).
- F7. The amendment header time (05:00 UTC) is later than its commit (04:54:39Z).

## 13. What was not done

- No L2-isolated or A/A-bracketed timing; the CPU evidence is L0 (the replay and the producer's records agree, which is a consistency check and not an isolation claim).
- No retuning of the SGE merge parameter gamma (4) at lower c; no run at c between 1.60 and 2.00 other than 1.75; no k = 21, 22 runs below c = 1.60.
- Not replayed: the remaining k = 21 and k = 22 rho runs, and the S, L0, L1, L, B0 arms of N4v2 (outside J2). The plane-quartic construction and the relation sets were not re-derived independently.
- No keyring signature; no pcc claim registration (pcc ledger untouched); no session-log entry, benchmark record, leaderboard or `benchmarks/README.md` row (dispatch scope).

## 14. Commands run (all from `/home/user/crypto` unless stated; `$SCRATCH` = the session scratchpad)

```
git rev-parse HEAD; git status --porcelain -- <producer paths>; (manifest) git ls-files <sets> | sha256sum -> FROZEN.sha256
cd research/quartic_n4v2_20261020 && rustc --edition 2021 -O v2.rs -o $SCRATCH/build/v2_A        # + repo-root and absolute-path forms
python3 -I tools/s3get.py setec-astronomy-artifacts store/sha256/aa/aac9a25c... $SCRATCH/n4v2raw/units.tar.gz us-east-2   # read-only; tar -xzf into the scratchpad
python3 -I tools/replay_worker.py $SCRATCH/build/v2 $SCRATCH/replay/rebuilt w1            # 84 x: v2 ic <q> <seed> D 3.0 <c> 4 4
python3 -I tools/replay_compare.py research/leads_chase_20261112/j2 $SCRATCH/replay/rebuilt evidence/replay_compare.json
python3 -I tools/v3_compare.py research/leads_chase_20261112/j2 $SCRATCH/n4v2raw/x/units research/quartic_n4v2_20261020/results evidence/v3_compare.json
gcc -O2 -o $SCRATCH/build/certcheck tools/certcheck.c -lm; python3 -I tools/run_certs.py $SCRATCH/build/certcheck evidence/certcheck.jsonl <J2 units> <J2 heldout>
python3 -I tools/recompute.py <J2 dir> $SCRATCH/n4v2raw/x/units $SCRATCH/n4v2supp/supp-k22-rho $SCRATCH/replay/rebuilt evidence/recompute.json
python3 -I tools/rho_replay.py $SCRATCH/build/v2 $SCRATCH/replay/rho <N4v2 raw> w1 rho_65479_1_all rho_262111_1_all rho_2097091_1_0_neg rho_2097091_1_0_plain   # and w2: rho_4194271_1_0_neg
$SCRATCH/build/v2 calib {2097091,4194271} {1,2,3}                                           # evidence/calib_recheck/
python3 -I tools/v0_v8_checks.py . ; python3 -I tools/controls.py ...; python3 -I tools/make_tables.py ...; python3 -I tools/make_figure.py .; python3 -I tools/build_reports.py .
python3 scripts/validation_gate.py evidence research/leads_followup_20261114/item03_j2_validation
```

Wall: replay 84 units 27.5 min (1 thread), rho replays 23 min (k = 22), certificates about 8 min, everything else seconds. Peak: 2 threads.

## 15. Validator tool hashes (sha256 prefixes)

- `build_reports.py` fab95103ae9af9b2
- `compare.py` dd93dfc3e56d69c8
- `controls.py` 144bcb95d36ec06d
- `make_figure.py` 1be882e19fcbd43c
- `make_tables.py` 677ec295c488c477
- `recompute.py` be19ea78997f8ce6
- `replay_compare.py` 388eb23e156d1290
- `replay_worker.py` ceb2c4c7acae3364
- `rho_replay.py` 86404954045768b5
- `run_certs.py` 1944cd2062ad1812
- `s3get.py` 222f16e9331c726d
- `v0_v8_checks.py` 7ac256d71c2ecc87
- `v3_compare.py` fb8c584c6103cba6
- `certcheck.c` 5568f495f36a88e7

## 16. Background runs: state at the end

All long runs were started with `nohup` (logs and status in the scratchpad, copied to `evidence/`); none is running now (`ps` shows no `replay_worker`, `rho_replay`, `run_certs`, `certcheck` or `v2` process).
Replay worker: PID 16843, finished 2026-11-15T00:14:43Z, 84 units, all return codes 0 (`evidence/replay/worker_w1.log`). Certificate driver: first run stopped by me at the order-parsing bug (section 5 / PROTOCOL
amendment 3a), second run finished `ALL PASS 22 instances` (`evidence/certcheck.stdout.txt`). Rho replay: first launch (PID 20125) mis-specified and killed, then two workers (w1: k = 16, 18, 21 plain and neg; w2: k = 22 neg) finished
2026-11-15T00:37:02Z and 00:41:06Z (`evidence/rho_replay/rho_compare.log`). The other PIDs were not recorded.
