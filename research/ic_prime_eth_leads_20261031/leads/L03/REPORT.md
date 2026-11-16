# L03 report: sparse-parity (expander-code) hashed factor bases vs Bochum's affine hash

Date 2026-10-31. Workdir /private/tmp/claude-501/-Volumes-SSD990-2/05ea5a86-648b-4917-bb71-bf7fdae8b2e7/scratchpad/L03/ (copied to /Volumes/SSD990-2/ic-prime-eth-leads-20261031/leads/L03/).
Labels: [M] measured (file named), [D] derived (arithmetic shown), [P] plan.

## Verdict: constant_only (adopt random sparse parity rows of weight >= 5; weight 3 needs a repair)
1. **Structure beyond set size: none** [M]. For every family (Bochum-faithful affine, dense F2-affine, sparse weight 3/5/7, biregular-Tanner weight 3/5, window) Pr[X>0], E[X^2]/E[X]^2, Pr[X>=2] equal the size-matched i.i.d. random null to 1.000 +- 0.0003 at p ~ 2^14, 2^16, 2^18, 2^20, 2^22, m = 2, 3. The weighted slope of ln(ratio) per bit of b is |slope| <= 6e-5 per bit (all |z| < 2; table.txt "SLOPES"). Sparse/dense paired ratio within +-0.06% everywhere. No deficit growing with b.
2. **Weight 5 and 7 meet the 5% rule** [M]: raw Pr[X>0] of sparse / dense (unpaired, includes realised-size effects) is 0.97-1.01 at every b, both m, +-1-3% noise; weight 5 deviates at most -3.1% (reg5, b=16, m=3, +-1.7%), sp5 at most -1.5%, sp7 at most -3.0 +- 3.0% (b=20, 18 instances).
3. **Weight 3 with i.i.d. uniform RHS fails at m = 3** [M]: raw ratio to dense 0.905 / 0.880 / 0.902 / 0.980 / 0.941 at b = 14/16/18/20/22 (se 0.018, 0.026, 0.031, 0.043, 0.090). The cause is purely an **empty or heavy-tailed base**, not lost independence: any slot empty in 18% / 16% / 12% / 9% / 11% of m=3 instances (dense: 3% -> 0%; Bochum: <1%), because k ~ 2b/3 sparse rows over b+1 code bits are often linearly dependent with an inconsistent RHS. The empty fraction falls only slowly with b (about 18% -> 9-12%, roughly 1/b), so the deficit is a **constant, slowly decaying, repairable**, not growing. Repair [M]: choose the RHS as the parities of a planted curve point (rows consistent, columns sp3c/reg3c/sp5c): empty fraction exactly 0, paired ratio to null still 1.000, but base sizes become heavy-tailed (E[X] 1.27-1.8 vs 0.8 nominal at b=14, m=3) so Pr[X>0] per unit E[X] is 0.67-0.93 of dense. So weight 3 buys no yield; use weight >= 5 (or a rank-checked weight-3 system and accept bigger, more variable bases).
4. The structured biregular construction (reg3/reg5, configuration model) is **brittle**: reg3 m=3 empty fraction 0.51 / 0.03 / 0.02 / 0.33 / 0.00 at b = 14/16/18/20/22 (non-monotone; depends on stub-degree pattern vs b+1). Not better than uniform random rows; do not use without a rank/consistency check.
Skeptic-relevant limit: the Lemma-10 2/5 figure is not tested directly; measured Pr[X>0] = 0.55-0.65 at E[X] = 0.85-1.05 (about 1-exp(-E)), above 2/5 for all families.

## Instances and method [M]
Curves: L07's nine prime-order curves at p ~ 2^14, 2^18, 2^22 (curves.txt lines 1-3, 7-9, 13-15) plus six new ones I generated with L07's curves.py logic (seed changed) at p ~ 2^16, 2^20 (curves_new.txt, lines 4-6, 10-12): p=53857 (r=53653), 50671 (50753), 63409 (63541); p=899531 (r=899233), 847109 (848557), 1014037 (1015561). L07's dlog.c gives the full table k -> (x,y), k in Z_r (tab_*.bin, regenerated). So the requested 2^16 / 2^20 rungs are real, and 2^18 / 2^22 are L07's.
Target: X(R) = #ordered m-tuples of base points (slot-specific independent bases, as in Bochum) with point-index sum R, for **every** R in Z_r, by FFT convolution over the dlog index (exact after rounding; R = 0 excluded from statistics). Checks run on all 38160 instances [M]: sum_R X(R) = prod |B_i| (conservation) and a planted tuple's R has X >= 1: 38160/38160 both; an independent brute-force check of the FFT count (brute.py) matched 50/50 targets.
Encoding enc = 2x + (y & 1) < 2p (reduction.rs / encoding.rs hash_member: bits = [y0] ++ x-bits).
Families (exp.py, per-slot size n_i = r/2^{k_i}, sum k_i = round((m-1) log2 r), i.e. E[X] ~ 1 nominal; realised sizes recorded):
- bochum: s prime uniform in [2p,4p], alpha in [1,s), beta in [0,s), member iff (alpha*enc + beta) mod s < tau, tau = round(s n / r). Exactly the reduction.rs form.
- dense: k rows, each a uniform nonzero mask over the b+1 code bits, random RHS. sp3/sp5/sp7: k rows, each w uniform distinct bit positions, random RHS (k = round(log2(r/n))). reg3/reg5: biregular Tanner graph (configuration model, each column in ~kw/(b+1) rows). sp3c/sp5c/reg3c: same with consistent (planted) RHS. window: x in [o, o+round(n p/r)), random offset. random: i.i.d. uniform subset of size n (noise-floor comparator vs a second stream).
- Null for each arm and seed: i.i.d. uniform subsets of the arm's REALISED sizes (including size 0), so ratios isolate structure from size.
Seeds (each instance gives X for all R in Z_r): b=14: 3 curves x 334 = 1002; b=16: 3 x 100 = 300; b=18: 20+40 (curve 7), 60, 60 = 180; b=20: 3 x 30 = 90; b=22: 3 x 6 = 18. The requested 1000 seeds x 256 targets is met at b=14 (1002 seeds x all r targets) and not above 2^16 (fewer, per brief). se: delta-method over paired per-seed values.
Commands: `python3 curves_new.py` (curves.txt = L07's + new, sorted by b); `cc -O2 -o dlog dlog.c; ./dlog p a r gx gy tab_b_i.bin` per line; `CURVES=i python3 exp.py BITS SEEDS raw<b>.jsonl` (SEED0=20 for the extension of curve 7); `python3 analyze.py > table.txt`. A first run (superseded_v1/) had a null bug (size-0 arm got a size-1 null), which made weight 3 look like a spurious 1.5-7% structural deficit; fixed (m_random(n=0) is empty) and everything rerun.

## Results [M] (table.txt has every cell; raw*.jsonl are the per-seed rows)

Columns: P1 = mean Pr[X>0] over targets and seeds; ratio = P1 / size-matched random null (paired se); F2r = ratio of E[X^2]/E[X]^2 to null (the E[X^2]/E[X]^2 values themselves are 2.0-2.5, Poisson-like 1+1/E); empty = fraction of instances with an empty base; raw/dense = unpaired P1 ratio to dense with se.

### m = 2

| b | family | n | P1 | E[X] | ratio P1/null | F2r | empty | raw P1 / dense |
|---|---|---|---|---|---|---|---|---|
| 14 | bochum | 1002 | 0.5904 | 0.893 | 1.0001+-0.0002 | 1.0000+-0.0003 | 0.003 | 0.9990+-0.0044 |
| 14 | dense | 1002 | 0.5910 | 0.896 | 0.9998+-0.0002 | 1.0004+-0.0003 | 0.004 | - |
| 14 | sp3 | 1002 | 0.5763 | 0.902 | 1.0000+-0.0002 | 1.0002+-0.0003 | 0.047 | 0.9752+-0.0084 |
| 14 | sp5 | 1002 | 0.5887 | 0.894 | 1.0000+-0.0002 | 0.9998+-0.0003 | 0.009 | 0.9962+-0.0050 |
| 14 | sp7 | 1002 | 0.5896 | 0.895 | 1.0003+-0.0002 | 0.9994+-0.0003 | 0.006 | 0.9977+-0.0047 |
| 14 | reg3 | 1002 | 0.5914 | 0.897 | 1.0001+-0.0002 | 0.9999+-0.0003 | 0.005 | 1.0006+-0.0045 |
| 14 | reg5 | 1002 | 0.5844 | 0.892 | 0.9999+-0.0002 | 1.0001+-0.0003 | 0.020 | 0.9889+-0.0060 |
| 14 | window | 1002 | 0.5881 | 0.890 | 0.9984+-0.0006 | 1.0018+-0.0008 | 0.000 | - |
| 16 | bochum | 300 | 0.5742 | 0.856 | 0.9998+-0.0002 | 1.0002+-0.0003 | 0.000 | 1.0011+-0.0061 |
| 16 | dense | 300 | 0.5735 | 0.854 | 0.9998+-0.0002 | 1.0003+-0.0003 | 0.000 | - |
| 16 | sp3 | 300 | 0.5484 | 0.842 | 1.0001+-0.0002 | 0.9998+-0.0003 | 0.067 | 0.9562+-0.0168 |
| 16 | sp5 | 300 | 0.5696 | 0.848 | 0.9999+-0.0002 | 1.0001+-0.0003 | 0.007 | 0.9931+-0.0077 |
| 16 | sp7 | 300 | 0.5719 | 0.851 | 0.9999+-0.0002 | 1.0004+-0.0003 | 0.000 | 0.9971+-0.0063 |
| 16 | reg3 | 300 | 0.5730 | 0.855 | 0.9997+-0.0002 | 1.0004+-0.0003 | 0.000 | 0.9991+-0.0065 |
| 16 | reg5 | 300 | 0.5691 | 0.846 | 0.9997+-0.0002 | 1.0003+-0.0003 | 0.003 | 0.9922+-0.0070 |
| 16 | window | 300 | 0.5721 | 0.853 | 0.9997+-0.0003 | 1.0007+-0.0004 | 0.000 | - |
| 18 | bochum | 180 | 0.5750 | 0.858 | 1.0000+-0.0001 | 1.0000+-0.0002 | 0.000 | 1.0045+-0.0069 |
| 18 | dense | 180 | 0.5724 | 0.852 | 1.0000+-0.0001 | 1.0002+-0.0002 | 0.000 | - |
| 18 | sp3 | 180 | 0.5637 | 0.865 | 1.0001+-0.0001 | 1.0000+-0.0002 | 0.039 | 0.9847+-0.0178 |
| 18 | sp5 | 180 | 0.5749 | 0.857 | 1.0000+-0.0001 | 1.0000+-0.0002 | 0.000 | 1.0042+-0.0067 |
| 18 | sp7 | 180 | 0.5707 | 0.851 | 0.9999+-0.0001 | 1.0001+-0.0002 | 0.006 | 0.9971+-0.0088 |
| 18 | reg3 | 180 | 0.5750 | 0.858 | 0.9999+-0.0001 | 1.0000+-0.0002 | 0.000 | 1.0045+-0.0069 |
| 18 | reg5 | 180 | 0.5740 | 0.856 | 1.0000+-0.0001 | 1.0001+-0.0002 | 0.000 | 1.0028+-0.0071 |
| 18 | window | 180 | 0.5758 | 0.860 | 1.0002+-0.0002 | 0.9999+-0.0003 | 0.000 | - |
| 20 | bochum | 90 | 0.5863 | 0.884 | 1.0000+-0.0001 | 1.0000+-0.0001 | 0.000 | 1.0022+-0.0073 |
| 20 | dense | 90 | 0.5850 | 0.881 | 1.0001+-0.0001 | 0.9999+-0.0001 | 0.000 | - |
| 20 | sp3 | 90 | 0.5519 | 0.851 | 1.0001+-0.0001 | 0.9999+-0.0001 | 0.067 | 0.9434+-0.0294 |
| 20 | sp5 | 90 | 0.5899 | 0.900 | 1.0001+-0.0001 | 0.9999+-0.0001 | 0.000 | 1.0084+-0.0103 |
| 20 | sp7 | 90 | 0.5845 | 0.880 | 0.9999+-0.0001 | 1.0000+-0.0001 | 0.000 | 0.9991+-0.0075 |
| 20 | reg3 | 90 | 0.5849 | 0.884 | 1.0001+-0.0001 | 1.0000+-0.0001 | 0.000 | 0.9999+-0.0089 |
| 20 | reg5 | 90 | 0.5866 | 0.885 | 0.9999+-0.0001 | 1.0001+-0.0001 | 0.000 | 1.0026+-0.0077 |
| 20 | window | 90 | 0.5852 | 0.882 | 0.9999+-0.0001 | 1.0000+-0.0002 | 0.000 | - |
| 22 | bochum | 18 | 0.5989 | 0.919 | 1.0000+-0.0001 | 1.0000+-0.0001 | 0.000 | 1.0033+-0.0267 |
| 22 | dense | 18 | 0.5969 | 0.915 | 1.0000+-0.0001 | 0.9999+-0.0002 | 0.000 | - |
| 22 | sp3 | 18 | 0.6203 | 1.044 | 1.0000+-0.0001 | 1.0000+-0.0001 | 0.000 | 1.0391+-0.0421 |
| 22 | sp5 | 18 | 0.6032 | 0.931 | 1.0000+-0.0001 | 1.0000+-0.0001 | 0.000 | 1.0106+-0.0276 |
| 22 | sp7 | 18 | 0.5989 | 0.919 | 1.0000+-0.0001 | 1.0000+-0.0002 | 0.000 | 1.0033+-0.0272 |
| 22 | reg3 | 18 | 0.5969 | 0.914 | 1.0000+-0.0001 | 1.0001+-0.0001 | 0.000 | 1.0000+-0.0272 |
| 22 | reg5 | 18 | 0.5979 | 0.918 | 0.9999+-0.0001 | 1.0001+-0.0001 | 0.000 | 1.0016+-0.0278 |
| 22 | window | 18 | 0.5961 | 0.913 | 0.9999+-0.0001 | 1.0000+-0.0002 | 0.000 | - |

### m = 3

| b | family | n | P1 | E[X] | ratio P1/null | F2r | empty | raw P1 / dense |
|---|---|---|---|---|---|---|---|---|
| 14 | bochum | 1002 | 0.5339 | 0.792 | 1.0001+-0.0003 | 0.9999+-0.0006 | 0.008 | 1.0008+-0.0109 |
| 14 | dense | 1002 | 0.5335 | 0.806 | 0.9996+-0.0003 | 1.0006+-0.0006 | 0.028 | - |
| 14 | sp3 | 1002 | 0.4827 | 0.818 | 1.0002+-0.0003 | 1.0000+-0.0007 | 0.181 | 0.9047+-0.0175 |
| 14 | sp5 | 1002 | 0.5322 | 0.817 | 0.9999+-0.0003 | 0.9999+-0.0006 | 0.045 | 0.9976+-0.0128 |
| 14 | sp7 | 1002 | 0.5322 | 0.807 | 0.9996+-0.0003 | 1.0009+-0.0006 | 0.033 | 0.9976+-0.0121 |
| 14 | reg3 | 1002 | 0.3779 | 0.797 | 0.9991+-0.0003 | 1.0009+-0.0006 | 0.514 | 0.7083+-0.0242 |
| 14 | reg5 | 1002 | 0.5285 | 0.814 | 0.9998+-0.0003 | 1.0002+-0.0006 | 0.053 | 0.9906+-0.0131 |
| 14 | window | 1002 | 0.5252 | 0.789 | 0.9994+-0.0007 | 1.0019+-0.0014 | 0.000 | - |
| 16 | bochum | 300 | 0.6836 | 1.202 | 1.0000+-0.0003 | 1.0003+-0.0005 | 0.000 | 1.0179+-0.0135 |
| 16 | dense | 300 | 0.6716 | 1.176 | 1.0000+-0.0003 | 0.9998+-0.0004 | 0.013 | - |
| 16 | sp3 | 300 | 0.5909 | 1.139 | 1.0001+-0.0003 | 0.9997+-0.0005 | 0.163 | 0.8797+-0.0263 |
| 16 | sp5 | 300 | 0.6619 | 1.180 | 0.9995+-0.0003 | 1.0010+-0.0005 | 0.033 | 0.9855+-0.0173 |
| 16 | sp7 | 300 | 0.6703 | 1.183 | 1.0001+-0.0003 | 1.0000+-0.0005 | 0.017 | 0.9980+-0.0151 |
| 16 | reg3 | 300 | 0.6411 | 1.137 | 1.0001+-0.0003 | 0.9997+-0.0005 | 0.027 | 0.9546+-0.0173 |
| 16 | reg5 | 300 | 0.6507 | 1.134 | 1.0002+-0.0003 | 0.9999+-0.0005 | 0.033 | 0.9688+-0.0166 |
| 16 | window | 300 | 0.6672 | 1.165 | 0.9999+-0.0006 | 0.9998+-0.0013 | 0.000 | - |
| 18 | bochum | 180 | 0.5922 | 0.924 | 1.0001+-0.0002 | 0.9999+-0.0003 | 0.006 | 0.9944+-0.0189 |
| 18 | dense | 180 | 0.5955 | 0.942 | 0.9997+-0.0002 | 1.0006+-0.0003 | 0.017 | - |
| 18 | sp3 | 180 | 0.5369 | 0.888 | 0.9997+-0.0002 | 1.0005+-0.0004 | 0.122 | 0.9016+-0.0314 |
| 18 | sp5 | 180 | 0.5913 | 0.923 | 0.9999+-0.0002 | 1.0003+-0.0004 | 0.006 | 0.9930+-0.0189 |
| 18 | sp7 | 180 | 0.5975 | 0.933 | 1.0000+-0.0002 | 1.0000+-0.0003 | 0.000 | 1.0033+-0.0181 |
| 18 | reg3 | 180 | 0.5746 | 0.934 | 1.0000+-0.0002 | 0.9999+-0.0003 | 0.017 | 0.9648+-0.0235 |
| 18 | reg5 | 180 | 0.5808 | 0.923 | 1.0002+-0.0002 | 0.9999+-0.0003 | 0.033 | 0.9752+-0.0230 |
| 18 | window | 180 | 0.5892 | 0.920 | 1.0004+-0.0003 | 0.9991+-0.0006 | 0.000 | - |
| 20 | bochum | 90 | 0.6131 | 0.981 | 1.0001+-0.0001 | 1.0000+-0.0002 | 0.000 | 0.9968+-0.0253 |
| 20 | dense | 90 | 0.6150 | 0.993 | 1.0001+-0.0001 | 0.9998+-0.0002 | 0.011 | - |
| 20 | sp3 | 90 | 0.6028 | 1.080 | 0.9999+-0.0001 | 1.0000+-0.0002 | 0.089 | 0.9801+-0.0431 |
| 20 | sp5 | 90 | 0.6158 | 0.992 | 0.9997+-0.0001 | 1.0005+-0.0002 | 0.000 | 1.0013+-0.0256 |
| 20 | sp7 | 90 | 0.5963 | 0.959 | 1.0000+-0.0001 | 1.0000+-0.0003 | 0.022 | 0.9696+-0.0301 |
| 20 | reg3 | 90 | 0.5108 | 0.997 | 1.0000+-0.0001 | 0.9999+-0.0002 | 0.333 | 0.8305+-0.0650 |
| 20 | reg5 | 90 | 0.6144 | 0.994 | 1.0000+-0.0001 | 0.9999+-0.0002 | 0.011 | 0.9991+-0.0278 |
| 20 | window | 90 | 0.6139 | 0.987 | 0.9999+-0.0002 | 1.0000+-0.0004 | 0.000 | - |
| 22 | bochum | 18 | 0.6504 | 1.053 | 1.0002+-0.0001 | 0.9996+-0.0002 | 0.000 | 1.0006+-0.0164 |
| 22 | dense | 18 | 0.6500 | 1.055 | 1.0001+-0.0002 | 0.9999+-0.0003 | 0.000 | - |
| 22 | sp3 | 18 | 0.6116 | 1.109 | 0.9999+-0.0001 | 1.0002+-0.0002 | 0.111 | 0.9409+-0.0897 |
| 22 | sp5 | 18 | 0.6516 | 1.063 | 1.0002+-0.0002 | 0.9997+-0.0003 | 0.000 | 1.0026+-0.0216 |
| 22 | sp7 | 18 | 0.6455 | 1.042 | 1.0000+-0.0001 | 0.9999+-0.0002 | 0.000 | 0.9932+-0.0189 |
| 22 | reg3 | 18 | 0.6355 | 1.042 | 0.9999+-0.0001 | 1.0002+-0.0002 | 0.000 | 0.9777+-0.0342 |
| 22 | reg5 | 18 | 0.6491 | 1.055 | 1.0001+-0.0001 | 0.9997+-0.0002 | 0.000 | 0.9987+-0.0205 |
| 22 | window | 18 | 0.6316 | 1.012 | 0.9999+-0.0003 | 0.9999+-0.0004 | 0.000 | - |

## Membership constraint cost per slot [D] (cost.py, cost_table.txt)
Assumptions (derived, not measured; Bochum's own encoder gives edge counts through its Fourier modular_multiply gadget, which I did not run): arity m = 3, so k = ceil(2b/3) parity rows; word width W = bits(s) = b + 2 (s in [2p,4p]); a W-bit ripple adder = W majority (MAJ) nonlinear ports + 2W XOR; multiplication by the *known constants* alpha is shift-and-add, about W/2 - 1 adders (alpha has ~W/2 set bits). Bochum's `hash_member` = modular_multiply(enc, alpha, s) + modular_add(beta) + less_than(tau) (encoding.rs lines 1049-1083). Barrett-style `mod s` = 2 more constant multiplications. Totals: 3 constant multiplications of W/2-1 adders each, plus 3 W-bit carry chains (add beta, correction, compare tau): nonlinear ports nl = 3 (W/2-1) W + 3W + W, XOR2 = 6 (W/2-1) W + 6W + W.
Example b = 64: W = 66, (W/2-1) = 32, nl = 3*32*66 + 4*66 = 6336 + 264 = 6600; XOR2 = 6*32*66 + 7*66 = 12672 + 462 = 13134. Sparse w=5, b=64: k = 43 rows, each one XOR of 5 bits = 4 XOR2, so 43*4 = 172 XOR2, 0 nonlinear, 0 multiplications.
| b | Bochum affine (3 const mults) | dense F2-affine (rows x width, XOR2) | sparse w=3 | sparse w=5 | sparse w=7 | window x in [o,o+w) |
|---|---|---|---|---|---|---|
| 16 | nl 504, XOR2 990 | 11 x 8, 77 | 11 x 3, 22 | 11 x 5, 44 | 11 x 7, 66 | nl ~54 |
| 32 | nl 1768, XOR2 3502 | 22 x 16, 330 | 22 x 3, 44 | 22 x 5, 88 | 22 x 7, 132 | nl ~102 |
| 64 | nl 6600, XOR2 13134 | 43 x 32, 1333 | 43 x 3, 86 | 43 x 5, 172 | 43 x 7, 258 | nl ~198 |
| 128 | nl 25480, XOR2 50830 | 86 x 64, 5418 | 86 x 3, 172 | 86 x 5, 344 | 86 x 7, 516 | nl ~390 |
| 256 | nl 100104, XOR2 199950 | 171 x 128, 21717 | 171 x 3, 342 | 171 x 5, 684 | 171 x 7, 1026 | nl ~774 |
Reading: sparse parity costs 0 multiplications and 0 nonlinear ports; XOR2 count 2-5 per row-bit: (w-1)*ceil(2b/3), i.e. linear in b with constant (w-1)*2/3 (weight 5: 2.7 b; weight 7: 4 b), against about 3 b^2/2 nonlinear ports (b^2 scaling, 3 multiplications) for the affine hash. Dense F2-affine is also nonlinear-free but its rows have width ~b/2, so its XOR count is quadratic (b^2/3). Window costs ~3W comparator ports (linear) and no multiplication, but is negation-closed in x (L07). Savings per slot at b = 64: 6600 nonlinear ports -> 0 (w=5: 172 XOR2). Sparse rows are the only option that is linear in b with zero multiplications and zero nonlinear ports. A dense row of width b/2 cut into 3-XOR pieces is the same count, but it is exactly the dense-matrix elimination case; for an XOR-native solver the weight only sets the sparsity of the Gaussian elimination, and sparse is still better for CNF.

## Decision rule applied
Rule (LEADS L3): sparse hashes with Pr[X>0] within 5% of dense at row weight <= 7 -> adopt; falling with b -> record the weight growth required.
- Structure-conditional (paired to size-matched null): sparse/dense ratio 0.9995-1.0006 at all weights, b, m; slopes in b |<=6e-5|/bit, z<2: **adopt** (no weight growth required by structure).
- Realised-yield (unpaired): weights 5 and 7 within 3% of dense at every (b, m) (se 1-3%): **adopt, weight >= 5**. Weight 3 with random RHS: -9% to -12% at m = 3 (b = 14-18; -2 to -6 +- 4-9% at 20/22) and -2% to -6% at m = 2, caused by empty bases (4-18%), non-growing in b (slope of ln(raw/dense): +0.007 +- 0.006 per bit, i.e. closing slowly, not opening). No weight growth needed.
- => **constant_only**: sparse parity is a free substitute for the affine hash at the measured sizes, and removes b^2 nonlinear ports per slot. This does not move the exponent (it changes constants in the encoding; Lemma 10's 2-universality is not used by the measurement; the sparse family is only small-bias, so the expander/small-bias argument is still required for a proof at b >> 22).

## Caveats
- b <= 22 and m <= 3 only; per-slot sets of size r^{(m-1)/m}: 13 to 2000 points. At 128-256 bits the code-space/curve-point structure (bit bias of x < p, top bits) is not reproduced by toy curves; the empty-base mechanism (dependent sparse rows) is a code property and should persist at constant row density k/(b+1) = 2/3.
- Hash is one random draw per slot (independent), not the shared-base m! case (L07 covers that; window shared base loses by negation closure).
- b = 22 has only 18 instances, b = 20 90: raw (unpaired) ratios there have 3-9% se; the paired ratios (±0.0001-0.0002) are tight. The claim "no structure beyond size" is well measured; the claim about the realised-yield size effect is +-3% at best.
- The cost table is a derivation under a stated ripple-adder/shift-add model, not the Rust encoder's edge count, and does not include the (b-dependent) nonlinear cost of the EC point addition itself, which is unchanged between families. The SAT arm (b) of the lead (solver work with the three membership encodings) is NOT run: it rides on L1/L2, which have not produced an encoder; this report is the (a) arm plus the cost table.
- The x-window arm with independent slots does not show L07's negation deficit (ratio 0.998-1.000), as expected.

## Next step [P]
(1) Fold into atlas as constant_only: LEAD-ICE-decomposition-probability-audit gets the second-order numbers (paired ratio 1.000 +- 0.0003, five sizes). (2) In any SAT/tensor arm use uniform random sparse rows of weight 5 (k = ceil((m-1) b/m) rows) with RHS drawn as the parities of a random point only if a rank check shows the system inconsistent; weight 3 only with rank-checked rows. (3) When L1/L2 produce an XOR-native encoder, run arm (b): same instances, three membership encodings, kissat/CryptoMiniSat conflicts, expect the nonlinear-port column above.

## Skeptic re-run (about 1 minute)
`mkdir -p /tmp/sk && cd /tmp/sk && D=/Volumes/SSD990-2/ic-prime-eth-leads-20261031/leads/L03 && cp $D/curves.txt $D/tab_14_1.bin $D/exp.py $D/analyze.py . && CURVES=1 python3 exp.py 14 40 raw14.jsonl && python3 analyze.py | grep -E '^checks|^14 3 (dense|bochum|sp3|sp5|sp7) '`
Expect: checks all ok; paired "ratio P1 +-se" 1.00 +- 0.003 for every family; raw P1/dense noisy (+-8%) at 40 seeds. To see the weight-3 empty-base effect use the full b=14 set (`python3 exp.py 14 334 ...` per curve) and read the "empty" column (sp3 m=3: 0.18).

## Files
exp.py, analyze.py, cost.py, brute.py, diag.py, curves.txt, curves_new.py/.txt, dlog.c, raw14/16/18/20/22.jsonl (raw per-seed rows), table.txt (full analysis output incl. slopes and trends), cost_table.txt, log*.txt, superseded_v1/ (first run with the null bug, not used).
