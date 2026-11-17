# Pre-registration: Legendre-type features inside ml-ic-review P2 (SYNTHESIS s5 residue 6)
Written 2026-11-02 BEFORE any feature was computed or any model fitted. File sha256 recorded in run_log.txt.

## P2 as found
P2 = experiments/sumset-structure-audit (worktree ml-cryptanalysis-worktrees/p2-sumset-audit, branch
research/p2-sumset-audit-20261031, commit a9d67bd). Read-only here. Binary fields GF(2^n), n in {17,19,23}.
Closed with verdict "no lead" (results.json): its distinguisher arm A (cell-matched, bits only) is at chance.
Holdout C is not used (P2 data are toy-field sumsets; no ic-dataset rows are read).

## Deviation, stated up front
A Legendre symbol (v/p) needs odd p. In GF(2^n) every element is a square (order-2 character is trivial since 2^n-1 is odd),
and for n=17,19 (2^n-1 prime) there is no nontrivial multiplicative character at all. P2 has no prime-field data.
So the literal feature does not exist on P2. The tested analogue is the additive character of power maps:
  f_{e,i}(x) = Tr(z^i * x^e),  i = 0..n-1 (z = field generator, per bit position), e in {-1, 3}   (x^-1 with 0 -> 0)
i.e. 2n extra 0/1 columns (centred like the bit columns). e=1 is the existing bit columns (linear). Tr(sqrt(b)/x) is determined by
Tr(x) inside L, so e=-1 adds information only through the other i. This is the closest char-2 analogue of "per-bit quadratic character";
a negative result here does NOT test the prime-field Legendre feature. That test needs a prime-field P-track dataset (none exists).

## Design
Data: the 66 arm-A files armA_{n}_{curve}_{family}_l{l}_m{m}.npz in /Volumes/SSD990-2/ml-ic-20261031-data/p2 (xtr,ytr,xte,yte;
cell-matched, V_a only; V_b test sets were not saved so V_b replication is NOT available).
Baseline = P2's own models and code path (scripts/sumset_stage2.py fit/feats/auc_score/boot_ci imported unchanged), models LR, LR2, MLP,
same torch seeds (fit seed 1; permutation seeds 100+k), same splits (the npz), same rng seed. Treatment = identical, extra 2n columns appended.
Metric: test AUC (P2 primary metric), bootstrap 95% CI (200 resamples, same resample indices for both arms = paired).
Permutation null as in P2: LR 100, LR2 20, MLP 20; q99 = max(null max, mean+2.33 sd).

## Decision rule (fixed now)
lift_c = AUC_feat / AUC_base per (config, model). Pooled lift = mean over configs of lift_c for a model; CI = bootstrap over configs (2000)
combined with the per-config paired-bootstrap noise (resampling test rows then configs).
KEEP the feature iff for some model: pooled lift >= 1.30 AND its CI lower bound >= 1.30 (AUC ratio of 1.3 from chance means AUC ~0.65).
Otherwise: record as NULL. Because P2's baseline is at chance by construction (AUC ~0.5), a ratio of 1.3 is a deliberately demanding floor;
secondary, non-binding report: count of configs where AUC_feat > q99 of its own permutation null, versus 1% expected by chance, and the pooled
excess (AUC-0.5). A secondary exceedance cannot flip NULL to KEEP; it is a pointer for a later protocol.
Planted positive control: reuse P2's planted leak (bit1 XOR bit5 = label on 10% of samples) and a new planted character leak
(label forced to equal f_{-1,3}(x) on 10% of samples via rejection sampling of rows); the feature arm must detect the character leak
(AUC > q99) or the null is reported as "power not demonstrated".
Compute: one process, nice 19, 1 thread, expected < 0.5 CPU-h; hard cap 2 CPU-h.
