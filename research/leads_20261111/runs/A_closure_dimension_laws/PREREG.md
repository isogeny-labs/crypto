# Frozen before any run (2026-11-12) -- LEAD-A-closure-dimension-laws

Copied unchanged from research/leads_20261111/lane_A_falldegree.md, Lead A4. No threshold is moved after data.

- L3: dim(W_3 cap B_{<=3}) = n(5n+1)/2 and dim(W_3 cap B_{<=2}) = 3n for every unsatisfiable random-V
  curve-z instance, k = ceil(n/2), odd n, 15 <= n <= 33.
  Predictions: n=23: 1334; 25: 1575; 27: 1836; 29: 2117; 31: 2418; 33: 2739.
- L4: dim(W_4 cap B_{<=3}) = 3n(n+2) and dim(W_4 cap B_{<=2}) = 3n for every stalled random-V instance with
  n >= 35 (n=41: 5289; n=43: 5805).
- Falsifier: any deviation of one dimension from the formulas at any tested n kills the corresponding law;
  L3 and L4 are independent.
- Controls: positive = reproduce logged values n=15..21 (L3; out/n15..n21.txt seeds) and n=35 (L4: 61,425 total);
  negative = T-type instances must NOT satisfy the formulas; A/A = two instances per n.

Instance rule (fixed now): R-type = lfd_small.make_instance via emit_instance.py, k = (n+1)/2; the first two seeds
s = 1, 2, ... with `emin count` = 0 (unsatisfiable). T-type = emit_instance_T.py, same k, first two unsatisfiable seeds.
For L4 an instance is "stalled" iff fallclose D=4 FULL=1 returns one_in = 0; instances with one_in = 1 are reported,
not counted against L4.
Tool: fallclose.c (copied byte-identical from ../A_fall_effective_variable, sha256 f8d33898...b52f), M4RI.
