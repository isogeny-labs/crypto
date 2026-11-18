# Pilot selection rule (written before any pilot data)

The pilot fixes the candidate's two parameters per curve: the column count `k` (spread seeds `[2^(5(j-1))]G`,
`multiple_doublings=5` kept from the A1 study) and the block half-width `m` of `walk_sib=m`. It runs one session per
curve on its own target seeds (n23 20261118471, n29 20261118472, n37 20261118473; never reused by the confirmatory
study), 48 public targets x 2 rounds, arms `ls` (the incumbent, lean + sib) and `c{k}-m{m}` for
k in {4,5,6} (n23), {1} (n29), {7,8,9} (n37) and m in {1,2,4,8} (`make_specs.py pilot`).

Rule, per curve, applied mechanically by `pilot_select.py`:

1. For each candidate arm, take `ecbench compare --a ls --b <arm> --weight w --resamples 4000 --seed 20261002` point
   ratios at w = 0, 0.1, 1 (paired by target over the pilot session).
2. Among arms whose three point ratios are all < 1, pick the one with the smallest mean of the three ratios.
   If no arm has all three below 1, pick the arm with the smallest maximum ratio over the three weights.
3. Ties (to 3 decimals): smaller k, then smaller m.

The rule is a function of the curve (n, a) alone once fixed; the confirmatory study uses fresh seeds. The pilot's
ratios are not evidence for the confirmatory claim and are reported as pilot only.
