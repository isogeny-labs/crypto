# LEAD-TRF-second-torsion-direction
## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_obstructed (last reviewed 2026-10-24; source atlas_tally_20261101). Family: transfer.
- Title: Reach a second N-torsion direction: nonequivariant trace-zero transfer, Kani/Robert graph kernels, Richelot gluing of E with E'.
- next_step_excerpt: empty in leads.json (no next step recorded; lead is closed).
- Files read: research/lead_sweep_20261110/leads.json; docs/session-logs/2026-10-27.md (lines ~782-792, one-line closure "trace-zero transfer is closed"); research/ecdlp_subexp_desk_20261101/data/candidate_matches.json (title/score only). The atlas YAML is encrypted here; no derivation of the obstruction was available, so section 2 uses my own reading of the mechanism.
## 2. Hypothesis
Recorded obstruction (my reading, unverified): E[n] has Frobenius eigenvalues 1 and p mod n. A transfer built from Frobenius-equivariant maps (trace-zero, Weil restriction) sees only the eigenvalue-1 line, i.e. E(F_p)[n]. The "second n-torsion direction" is defined only over F_{p^k}, k = ord_n(p). For a random curve whose group order has a large prime factor n (the ECDLP regime), k is huge, so the second direction is never reachable at small extension degree.
Falsifiable claim: among random toy prime-field curves with largest prime factor n >= 100 of #E(F_p), the fraction with ord_n(p) <= 12 is <= 0.5%.
## 3. Preregistered prediction and falsification criterion
- Statistic: F = fraction of scored curves with ord_n(p) <= 12.
- Prediction: F <= 0.005 -> consistent_with_prediction (obstruction re-derived: second direction is unreachable for generic curves). F > 0.02 -> falsified (second directions are cheap generically; closure would be wrong). 0.005 < F <= 0.02, or any failed control -> inconclusive.
- Null / negative control: the random-curve population itself is the null (no designed embedding degree).
- Positive control: curves with prime n | p-1, n | #E (n in [11, 4 sqrt p)) must all score k = 1; detector must see them (otherwise inconclusive).
- A/A arm: scored curves split by index parity into two halves with identical procedure; |F0-F1| must be within 3 sigma (binomial) or the verdict is inconclusive.
## 4. Method
Pure Python. Primes p = nextprime(6000), nextprime(9000), nextprime(14000) (group orders ~2^12.5..2^14). Seed 20261110 (positive control seeded by p). 5500 random (a,b) per prime; #E by Legendre-table sum (O(p) each); n = largest prime factor of #E by trial division; k = multiplicative order of p mod n, capped at 64 (cap+1 means >64). Curves with n < 100 skipped. Cost estimate: ~16500 curves x ~10^4 ops, about 1-3 minutes, < 50 MB, one core, 400 s internal time guard.
## 5. Classification: runnable_local
A stdlib toy decides the recorded bound (generic embedding degree) cheaply; run.py checked in with `--smoke`.
## 6. What a run cannot establish
Nothing about Kani/Robert graph kernels, Richelot gluing, or nonequivariant maps themselves; only that the field-of-definition obstruction holds generically at toy size. Toy sizes (n >= 100) may not extrapolate to cryptographic n (though the heuristic gets stronger with n). Closure of the lead could still rest on a different obstruction than the one tested. Results are [pcc:unverified].
