# Skeptic verification: LEAD-A-closure-dimension-laws (A4)

Date 2026-11-12. Independent re-run by a skeptic subagent. Every statement is `[pcc:unverified]`. Nothing in this
directory other than this file was edited; re-runs used binaries rebuilt in the skeptic's scratchpad from the
`.c` files here.

## 1. Re-runs (exact commands)

```
gcc -O3 -march=native -o $S/fc  fallclose.c   -lm4ri -lm     # sha256 f8d33898...b52f, identical to ../A_fall_effective_variable/fallclose.c (checked)
gcc -O3 -march=native -o $S/fcq fallclose_q.c -lm4ri -lm
# L3: instance selection redone independently (first two seeds with `emin count` = 0), original fallclose, D=3, FULL=1
for n in 23..33 odd: emit_instance.py n s (n+1)/2 | emin count ; ... | $S/fc 3 8192 1
# L3 positive control n=15: emit_instance.py 15 {1,5} 8 | $S/fc 3 8192 1
# L3 negative control: emit_instance_T.py 23 {2,35} 12 | $S/fc 3 8192 1
# L4 positive control: emit_instance.py 35 6 18 | $S/fcq 4 4096 1
# L4 new size: emit_instance.py 41 2 21 | $S/fcq 4 8192 1
```

## 2. Results

| check | reproduced? | value |
|:--|:--|:--|
| Seed selection n=23..33 (first two unsat) | yes, same seeds (4,8 / 5,6 / 3,6 / 3,4 / 2,3 / 1,5) | |
| L3 W_3 dims, 12 cells, original fallclose | yes, all 12 | [0,1,3n,n(5n+1)/2]: 1334, 1575, 1836, 2117, 2418, 2739 |
| Formula arithmetic (3n, n(5n+1)/2, 3n(n+2)) | yes | 1334...2739; 5289 (n=41), 5805 (n=43) |
| L3 positive n=15 vs lfd_lower_bound out/n15.txt | yes | [0,1,45,570] |
| L3 negative T23 s2, s35 | yes | [0,0,44,1078] |
| L4 positive R35 s6, fallclose_q | yes | [0,1,105,3885,61425], codim 5287, one_in 0, 116 s, 0.45 GB |
| L4 R35 s6 vs A3 agent's pos_R35 record | yes (A_fall_effective_variable/out/cells.jsonl line 9) | identical |
| L4 new size R41 s2 (sols=0), fallclose_q batch 8192 | yes | [0,1,123,5289,99999], codim 24315, one_in 0, stopped_early 0, 590 s, 1.29 GB |
| n=41/43 seed selection | yes: scan.jsonl R41 s1,s2 sols=0; R43 s1=2, s2=4, s3=0, s4=4, s5=0 | |
| fallclose_q validation file | out/validate_q.txt: "checked 54 mismatch 0" | |
| No committed source edited by this lead | yes; dirty tracked files in the checkout (examples/jv_cover.rs, f4_fp.rs, gaudry_quartic_bin.rs) belong to other lanes, not listed by this lead | |

## 3. Preregistration and falsifier check

- PREREG.md copies the lane file's A4 claim, predictions, falsifier and controls unchanged (compared with
  lane_A_falldegree.md lines 318-352). No threshold moved. Verdict "held" follows: no dimension deviated.
- Exploratory additions (even n, n=35..43 at D=3, T-type fit) are labelled post-hoc and are not used for the verdict.

## 4. Caveats found (none refutes the verdict)

1. **Tool substitution at the decisive cells.** PREREG.md freezes the tool as `fallclose.c`; every L4 cell at
   n=37..43 (including both new sizes 41 and 43) was run with the agent's own `fallclose_q.c`. Bit-for-bit equivalence
   was shown only up to n=35 (54 small configs + R35 s6). The original tool was not run at n=41/43 (it would need
   about 1.9 GB of dense RREF and likely more than 20 min). RESULT.md discloses the tool but does not call it a
   protocol deviation; it is one, though a well-mitigated one.
2. **The "unsatisfiable" qualifier in L3 is vacuous at D=3.** Satisfiable R23 seeds 1, 2, 3 (2, 2, 4 solutions) give the
   same [0,1,69,1334] and one_in=0. Not a refutation (L3 only claims unsat instances), but the A/A "zero spread" and
   the match are a property of the generator family, not of unsatisfiability.
3. **D=4 negative control is thinner than the A/A rule.** One T-type instance per n (T35 s2, T37 s2), and none at the
   new sizes 41/43. At D=3, T-type had 2 instances per n.
4. **Positive control for L4 is the lfd_small R35 s6 instance, not the treatise's fit instance** (data-bin-fd-instances.json
   is not on disk). The total 61,425 matches the lane's logged number, so the tool sees the same structure.
5. Infrastructure (EKS/S3/benchrec/replay certificates, research-kb) unavailable, as the agent states.

## 5. Verdict

The key numbers reproduce exactly: L3 at all 12 preregistered cells with the original frozen tool, the L4 positive control
(61,425 total), and the L4 new-size cell n=41 s2 (B<=2 = 123, B<=3 = 5289 = 3n(n+2)). The "held" verdict follows the
preregistered falsifier with no threshold moved. I agree with the verdict `[pcc:unverified]`. The caveats in section 4 should
be carried into any summary: chiefly, the n=41/43 cells come from the agent's own `fallclose_q` and not from the frozen
`fallclose.c`, and that equivalence is shown only up to n=35. n=43 was not re-run here (about 17 min and 1.7 GB, at the limit).
