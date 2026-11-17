# AUDIT L8-n33-control (auditor pass, 2026-11-02 ~10:45 PST)

Verdict: **valid, with minor wording and process corrections.** The code computes what PROTOCOL says, every README number matches RESULTS.json and raw, the re-runs I did all reproduced, and nothing is claimed beyond toy evidence. No security claim is made by the experiment or by this audit.

## 1. Does the code compute what PROTOCOL says? Yes.
- `src/run33.py`: arms A,C1,C2,D1,B,RM from N1 `lib.system(33,17,s)` (rng Random(1000n+100k+s), as stated); nsol via `bsols` (`sols` for RM); degree-4 `closure` only when nsol==0; stops an arm at 8 UNSAT; s<150. Seeds run in pairs with a barrier, so an arm could overshoot 8. It did not: RESULTS has unsat=8 for every arm, and raw has no duplicate (arm,s). It used a 4-worker pool, as allowed.
- `src/run27.py`: takes the first 8 UNSAT N1 n=27 records per arm (A,C1,D1) by seed, regenerates with `lib.system(27,14,s)`, runs the L1 `macclose` with N=28, d=4, and stores both results side by side. Matches P6.
- `src/analyze.py`: counts, Clopper-Pearson (0/8 gives 0.3123, correct), low4/dim4 histograms, P5 comparison vs L2 on common (arm,s), n=27 comparison vs N1 records. No logic errors found.
- `src/macclose.c` is byte-identical to L1's (cmp). The shipped `src/macclose` binary differs byte-wise from my rebuild (`gcc -O3`), as expected for a different toolchain stamp, and behaves identically (see 4).

## 2. Is validation real?
L8 adds no new engine. It relies on three layers, all checked:
- gf2close (N1) validated by N1/L1/L3 (n<=21 against Python and brute force). macclose (L1) validated on 297 cells, re-audited in L1 AUDIT. Both independent of each other. Only the generator is shared (disclosed in L1).
- P5 (L2's gf2close5 vs L8's gf2close) is a weaker check than it sounds: gf2close5 is the N1 engine with different pivot storage, so it is a regression check, not an independent one. L2 AUDIT (items 3,4) separately checked triangular vs N1 and an exhaustive 2^34 solution count. README says "agrees", not "independent"; fine.
- The only genuinely independent-engine data in L8 are the n=27 macclose replays. The README does not claim an independent engine at n=33. I added that check myself (below); L8 did not do it.
- Validation limit that README should state: nothing in L8's own validation exercises N=34 beyond the P5 overlap with L2 and my re-runs.

## 3. README vs RESULTS vs raw
- `python3 src/analyze.py` re-run: output **identical** to RESULTS.json (dict equality).
- Every table cell in README (draws 38/22/90/42/54/76, UNSAT 8, one-in-W_4 counts, low4, dim4, bounds, 24 common cells with 0 mismatches, n=27 8/8 with low4 3683, 0 mismatches vs N1) equals RESULTS.json.
- Identities check out: C(34,<=3)=6580, C(34,<=4)=52956=dim4(A), 33*97=3201, 33*35=1155, C(28,<=3)=3683, 31*91=2821.
- N1 comparisons: I recounted N1 raw. n=27: A 9/9, C1 8/8, C2 8/8, D1 8/8 reach 1, low4 3683 (B/RM 0/8, 783). n=29: C1 9/9, D1 7/8 (one D1 record at low4 2536). n=31: A 9/9, C1/C2/D1 0/8. All as README says.
- Prediction scoring (P1-P6 held) is correct. P5 and P6 and the A/C1/D1 parts of P1-P3 were known before the protocol, which PROTOCOL and README both say. P2 for C2 and P4 at n=33 are the only new ones, as README says.
- Timestamps: PROTOCOL.md mtime 09:00:32, run33.py and the run33.err creation 09:00:44, analyze.py 09:16, run27.py 10:17. So PROTOCOL precedes every run and was not edited afterwards. PROTOCOL's header says "~09:05 PST", which is later than its actual mtime and later than the R1 start; the order is still right, the stated time is wrong by about 5 min.

## 4. Re-runs (auditor; ~10 CPU-min in total, 4 threads)
- Rebuilt macclose from L8 source; **n=27 A s=1: (24158, 3683, 1)**, equal to the L8 record.
- **n=33 C1 s=0 with gf2close:** (45507, 3201, 0), equals the record.
- **n=33 C1 s=0 and A s=1 with the independent macclose (not done in L8):** C1 s=0 (45507, 3201, 0); A s=1 (52956, 6580, 1). Both equal the gf2close records. This is an independent-engine confirmation at N=34, on two draws only.
- Exhaustive 2^34 `sols` (a different mode from `bsols`): A s=4 -> 0, A s=3 -> 6, C2 s=8 -> 0, D1 s=16 -> 0. All equal the nsol recorded from bsols, so the UNSAT selection holds on those 4 draws. The other 44 selected draws were not re-counted.
- Not re-run: the other n=33 closures (about 40 CPU-s each by gf2close, 4-5 min by macclose), n=27 macclose for the other 23 seeds.

## 5. Problems found (none changes a headline)
1. **Wall time is wrong.** README: "wall about 2.3 h". File times give R1 from 09:00:44 to 10:29:01, i.e. about 1.5 h (88 min); the sum of per-job walls is 3.84 thread-hours, which is also not 2.3 h. Replace with about 1.5 h elapsed.
2. **Thread budget (undisclosed).** R2 (2 threads) ran 10:17-10:29 while R1 (up to 4 threads) was still running until 10:29:01. The raw records carry no timestamps; replaying the recorded job walls through the run33 pool logic (reproduces total elapsed within 8 s of actual) puts R1 at 1-4 busy workers in that window (mean about 1.9). So the 4-thread cap was probably exceeded briefly (peak 6), and my own audit re-runs added 4 more threads for several minutes. README claims neither; it should disclose the overlap.
3. **"They do not stall at n=29" is too strong.** README says the stand-ins do not stall "at n=29 (N1: C1 9/9, D1 7/8)". D1 at n=29 has one stalled record (low4 2536). Accurate: C1 does not stall at n<=29; D1 stalls on 1 of 8 at n=29.
4. **"n=2k-1 cannot be what separates stall from no-stall"** is correct (all odd n here have n=2k-1) and worth keeping. The hypothesis in L1 was really about k even/odd; the data also rule that out (k=14 no stall, 15 no stall for C1, 16 stall, 17 stall), but README does not say so. Also note that n=27..33 is only four sizes, so "stall onset between n=29 and n=31" is a description of four points, and the D1 behaviour at 29 shows the onset is not sharp.
5. **Selection** (also in L2 AUDIT C): closures run only on UNSAT draws, and A's nsol is even, so the UNSAT-conditioned sample differs by arm. For A, only 8/38 draws were UNSAT. README reports draws scanned, so this is visible, but it does not say that SAT draws can never refute, so "no A draw failed" is about UNSAT draws only.
6. The n=27 independent replay covers A,C1,D1 only on N1's seeds (8 each); n=27 has no stall to explain, so it is a replication of a null result. README says "match N1 record-for-record", which is fair.
7. The README's "Correction to LEADS2" is right on the facts (N1 raw has n=27), but LEADS2/L1 meant that no independent-engine or round-log n=27 run existed. L8 now supplies the former, not the latter.
8. `analyze.py` P5 `nsol_mismatch` is a quadratic scan; harmless. Raw logs are tiny (75 KB total), so no compression was needed; the limit of 200 MB is met.
9. Conductor: BRIEF section 0.6 says T-5736 holds this tree and agents must not run `conductor check`; the user-level CLAUDE.md asks for one before code changes. I followed the BRIEF and made no code changes, only this file.

## 6. Overclaim check
No security claim, no B*D^2 statement, no extrapolation beyond n=33, no claim of mechanism. A is correctly described as not the treatise's T33a/b instance. The 31% bound on 8 draws is stated. The sentence "the certified d_F>=5 onset does NOT appear on this generator at n=33" is supported only for 8 UNSAT A draws of this generator and is consistent with treatise Thm lfdexact (d_F=4 on random-subspace instances up to n=33); it says nothing about T33a/b, which are drawn inside ker Tr.

## Corrected outcome
At n=33 (k=17, N=34), degree-4 GF(2) closure: A refutes 8/8 UNSAT draws (low4 6580, full); C1, C2 stall 0/8 at 3201; D1 0/8 with low4 3201 x6, 3212, 3235; B and RM 0/8 at 1155. gf2close agrees with L2 on all 24 shared cells. The independent engine macclose agrees on the two n=33 draws I tried, and gives A,C1,D1 8/8 reaching 1 at n=27 with low4 3683, identical to N1. These are 8 outcomes per arm from a degree profile that fixes low4, not 8 independent confirmations. Fix items 5.1-5.3 in README (wall time, thread overlap, D1 at n=29).
