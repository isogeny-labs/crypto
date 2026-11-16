# L4: is the solving-degree regularity F4 bookkeeping? (frozen before data, 2026-11-15)

Question (red team R6/1b). The recorded `solving_degree_max` equals the highest productive F4 pair degree and
tracks the largest generator degree plus 0 to 6. Is the last productive step at degree D + 1 simply the pair of
the binomial y^D − 1 with a quadric (bookkeeping), and does the variable order change it?

Instrument. (a) `F4SUB_STEP_LOG=1 curve25519_family f4sub 20 sub 19 0 mont 2 sym 6 20263019 1800` (the k = 20,
m = 2, D = 19 cell; same seed as the record so the targets agree) with the per-step log {degree, pairs, rows,
all_rows, new_polys, ops}. (b) The same cells with form `sym-efirst` (e-variables first) at D = 19 and D = 57
(seed 20263057), m = 2. `ulimit -v 6000000`, RAYON_NUM_THREADS=1.

Predictions. P1: in every target of (a) the last step with new_polys > 0 has degree 20, and that step has at most
m·(number of chain generators) = 2 pairs... stated plainly: at most 4 pairs, consistent with binomial–quadric
pairs only. P2: every step of degree 21 or above (if any) has new_polys = 0. P3: with `sym-efirst`, SD differs from
the recorded value (20 at D = 19, 21 at D = 57) by at most 1 and mean field_ops by less than a factor 2.

Decision. P1 and P2 holding means SD is bookkeeping of the generator degree and must not be read as a hardness
measure (the corpus record for HEUR-FIRST-FALL-DEGREE says so). P3 failing means SD is order-dependent and the
exponent studies' SD column needs a variable-order disclosure.
