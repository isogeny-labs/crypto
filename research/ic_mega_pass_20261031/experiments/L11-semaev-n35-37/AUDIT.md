# AUDIT L11-semaev-n35-37 (auditor pass, 2026-11-02 17:36-17:52 PST)

Verdict: **partially valid. The measured fact (A at n=35 does not reach 1 in W_4) is real and independently reproduced, but README.md and RESULTS.json are stale and understate it. Several README statements are wrong against the raw file.** No security claim is made by the experiment or by this audit. I changed nothing except adding this file; the experiment's runner was still running and writing raw/n35_d4.jsonl while I audited (see 5.1).

## 1. Does the code compute what PROTOCOL says? Yes, with two deviations.
- `src/run.py`: n=35 -> k=18, N=36; n=37 -> k=19, N=38; `lib.system(n,k,s)` (N1, rng Random(1000n+100k+s)); nsol by `bsols`; d=4 `closure` only when nsol==0; arms A and C1; stops an arm at U=8 UNSAT; 3-worker pool. Matches R1.
- Deviation 1: arms stop only after a 3-seed batch finishes, so arms overshoot 8. At my 17:52 snapshot A had 9 UNSAT closures. PROTOCOL says "stop at 8". Harmless, but the sample is 9, not 8 (same as N1).
- Deviation 2 (thread cap): `run.py 35` and `run.py 37` each use 3 workers and were started 70 s apart (raw files and logs created 17:31:41 and 17:32:50). PROTOCOL says "at most 3 concurrent". Up to 6 processes ran together, and the n=37 file has 3 records and nothing after 17:33:05. My own re-runs added 2 more. The machine load average was 27 on 14 cores, so recorded `wall` is not meaningful; `cpu` (child CPU) is.
- The 45-CPU-min cap is not implemented in code; moot (closures take about 55-72 s CPU at n=35).
- `src/rnd3.py` was never run (no output). Read only: it calls L9 `macdump 3` with input "2k 4 neq", the format macdump's `scanf` reads, and takes `low_by_round`, `falls_r3 = low[3]-low[2]`. Looks right, untested.
- `src/SHA256` (created 17:31:33, 8 s before `run.py`) covers engines, lib.py and PROTOCOL.md only. All six hashes still verify. It does not cover `run.py` or `rnd3.py`, which did not exist yet. PROTOCOL.md mtime 17:31:33 precedes every raw file (earliest 17:31:41), and its hash is unchanged, so the pre-registration order holds.

## 2. Is validation real? Partly, and the load-bearing check was missing. I did it.
- The experiment adds no new validation. It leans on N1/L1/L8 validation of gf2close at n <= 33. The headline here is a *negative* closure result at n=35 (ncols = 66712 > 2^16, a size never run before in this lineage; A at n=33 had ncols 52956). A negative result on a new size, from one engine, is exactly where an engine bug would hide. L11 ran no cross-engine check and no positive control at n=35.
- I read gf2close.c for 16-bit or int overflow: all indices are `long`/`u64`; nothing size-specific. That is a read, not a proof.
- **Independent check (auditor):** regenerated `lib.system(35,18,2)['A']` and ran (a) `bsols` -> nsol 0; (b) N1 `gf2close` -> (dim 61425, low 3885, one 0, ncols 66712, nlow 7807, nprod 139860, cpu 72.2); (c) the independent L1 `macclose` (rebuilt from L1 source, `gcc -O3 -march=native`; 5 rounds, CPU 349 s) -> **(dim 61425, low 3885, one 0)**, bydeg [0,1,104,3780,57540]. Both engines agree exactly on A s=2. Its round-2 low rank is 3605 = n(3N-5), matching P4's round-2 part. This is one draw, one engine pair; the other A and C1 draws were not re-run.
- Not done by me: exhaustive 2^36 count (L8's auditor did 2^34; `bsols` was validated against it there, and I reproduced nsol 0 with `bsols` only).

## 3. README and RESULTS.json versus the raw file: they do not match.
At the README write time (mtime 17:35:22) the raw file already held the A s=6 record (a second UNSAT A closure, (3885, 61425, 0)) and C1 s=6 and s=8. Records are written in seed order, so later records existing implies these existed. Nevertheless:
- README: "1 UNSAT A draw (s=2)". Raw at that time: **2** A UNSAT, both one=0. RESULTS.json: A unsat_closed 1 (should be 2 at that time), C1 unsat_closed 1 (should be 3: s=0, 6, 8).
- Snapshot = the first 60 lines of raw/n35_d4.jsonl (file is append-only; `head -60 raw/n35_d4.jsonl | shasum -a 256` = bc3233a75a7e7ea8421008f8c2da1ad365a5c64adcbc134f298a10ec9cc44726; the file had 63 lines by 17:53): A 27 draws, **9 UNSAT, 9/9 with one=0, every one (low4 3885, dim4 61425)**; C1 33 draws, **7 UNSAT, 7/7 one=0, every one (3605, 54845)**. A UNSAT seeds: 2, 6, 13, 14, 16, 20, 23, 24, 25; C1 UNSAT seeds: 0, 6, 8, 12, 14, 18, 24. No duplicate (s, arm) in those 60 lines (checked). CPU 67-72 s (A), 55-63 s (C1).
- Arithmetic in README is right: 3885 = 3605 + 280 = n(3N-5) + 8n; C(36,<=3) = 7807; C(36,<=4) = 66712; 3605 = 35*103.
- README says "the n=35 runner died" and "session ended". **False as written:** raw/n35_d4.jsonl kept growing from 17:35:42 to at least 17:51:13 (about 35 new records after the README was written), so the runner was alive. The status "PARTIAL, interrupted" is wrong for n=35. For n=37 I cannot tell: the file stopped at 17:33:05 with 3 records (s=0 A, s=0 C1, s=1 A, all SAT), no UNSAT closure and no error log (run37.log empty). It may be dead or sitting in a slow C1 s=1 closure under heavy load. Process listing is blocked in this sandbox (`ps`, `pgrep`, `top` denied).
- `run35.log` and `run37.log` are empty; nothing there to check.

## 4. Predictions scored (against raw, 17:52 snapshot)
- P1 (A reaches 1 on >=7/8 UNSAT at n=35): **falsified**, not "in doubt". 7/8 allows at most one failure; A has 9/9 failures. The PROTOCOL falsification clause (any A UNSAT draw with one=0) fired at the first draw.
- P2 (n=37): untested (0 closures).
- P3 (C1 stalls at low4 = 3605, 8/8): holds on 7/7 so far at n=35; n=37 untested (4033 not observed).
- P4 (A round-3 falls 4n; C1 0): not run by L11 (rnd3 unrun). Not new in any case: L9 already measured n=35 A s0 round-3 falls = 140 = 4n and plateau 3605, so those two parts were not independent predictions.
- P5 (CPU < 45 min): moot for n=37 (no closure). The projection "n=33 ~44 s" is outdated; n=35 costs about 65 s.
- Not an independent prediction: P1's n=35 half was made after L9 had shown the A mechanism and L8 had shown A full at n=33; but the direction the data went (A stalls) is opposite to P1, so the pre-registration did its job.

## 5. Problems and caveats
1. **Moving target.** The runner was still appending while I audited; numbers above are the 17:52 snapshot (copy kept outside the experiment dir). The final raw file will differ in counts; the pattern (A all (3885, 61425, 0), C1 all (3605, 54845, 0)) is already constant across 9 and 7 draws.
2. Identical values across draws are not independent confirmations. low4 is fixed by a degree profile; 9 equal numbers show a deterministic plateau for this generator, not 9 independent tests of a mechanism (same caveat as L8).
3. Selection: closures only on UNSAT draws; A's nsol is always even (a<->b swap), so the UNSAT-conditioned sample differs by arm. "No A draw reached 1" is about UNSAT draws only.
4. At n=33 A has low4 6580 (= C(34,<=3)) together with one=1: once 1 is in W_4 the whole low block is spanned, so "full at 33, near-plateau at 35" is the same one=1 to one=0 flip, not a second observation. The macclose agreement above argues the flip is not an engine artefact; it does not explain it. L9's "8n = 4n falls + 4n zero syzygies" bookkeeping is consistent with 3885 - 3605 = 8n, but L11 did not test it (rnd3 unrun) and I did not re-derive it.
5. "Onset of d_F >= 5 for arm A in this closure's sense" (PROTOCOL falsification wording) is a statement about this closure definition (round-based W_4) on this generator's arm A, which is not the treatise's instance family (random subsets of ker Tr; T33a/b). It is not a measurement of the treatise's last fall degree. 1 not in W_4 does not give d_F = 5; whether 1 is in W_5 at n=35 was not run (about 66k -> 300k+ columns, several GB).
6. Process: BRIEF section 0.6 says do not run `conductor check`; the user-level CLAUDE.md asks for one before code changes. I made no code changes (only this file) and followed the BRIEF. 
7. Provenance of the generator beyond this directory (MP-comm_alg-1 `arms`, `ffd_semaev_core` under /Volumes/SSD990-2) was not re-audited.
8. Auditor CPU: about 72 + 5 + 349 s, about 7 CPU-min, 2 threads.

## 6. Overclaim check
No security claim, no extrapolation to n=131, no mechanism claim, no d_F claim. The README *under*-claims (1 draw instead of 2+, "in doubt" instead of falsified) and mis-states run status. The word "first" ("first A draw that does not reach 1") is correct relative to L8/L9/N1 records (A reached 1 on all UNSAT draws at n<=33 in L8, N1 n<=31).

## Corrected outcome
At n=35 (k=18, N=36), arm A (Semaev on a random subspace, N1 generator), degree-4 round-based GF(2) closure: **A does not reach 1 in W_4 on 9/9 UNSAT draws** (snapshot 17:52), each with low4 3885 = n(3N-5)+8n and dim4 61425 < 66712; C1 stalls 7/7 at low4 3605 = n(3N-5), dim4 54845. Independent macclose reproduces A s=2 exactly. P1 is falsified; P3 holds so far; P2, P4, P5 untested. The n=35 runner was alive when README said it died; n=37 produced no closures and its status is unknown. Fix README and RESULTS.json after the runner ends (regenerate from raw), state the thread overlap, and label results as "A leaves the d<=4 regime on this generator between n=33 and n=35" with the caveats in 5.2, 5.5 above. Whether 1 is in W_5 at n=35 is open.
