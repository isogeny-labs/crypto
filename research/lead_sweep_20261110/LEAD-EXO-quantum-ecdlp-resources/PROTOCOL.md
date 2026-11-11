# LEAD-EXO-quantum-ecdlp-resources
## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-24); family: exotic.
- Title: Quantum ECDLP: reversible secp256k1 point-addition circuits for Shor (resource reduction).
- Next-step excerpt (truncated in records): "Replace the per-step cswaps of the binary-GCD inverse by a safegcd/jump-GCD structure to narrow ..."
- Files read: research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (one line, l.199); research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (l.158-163). No further lead material found; the rest is from the mechanism.
## 2. Hypothesis
In a safegcd (Bernstein-Yang divsteps) modular inverse, the step that needs a conditional swap fires on a fraction of iterations well below 1, mean < 0.35 over random odd moduli of 12..22 bits, whereas the binary-GCD form spends one cswap on every step. So a divstep structure can replace the per-step cswap by a cheaper conditional negate-and-select on most steps. Divsteps also stay within the published bound floor((49n+57)/17) iterations.
## 3. Preregistered prediction and falsification criterion
- Statistic: mean over sizes of the per-input fraction (swaps in divsteps)/(divsteps iterations), N=20000 random inputs per size.
- Prediction: statistic < 0.35, zero bound violations, zero gcd errors, and |A - A/A| <= 0.01.
- Falsified if the statistic >= 0.35 or any iteration count exceeds the bound or any gcd is wrong (an implementation error would also show here). Inconclusive if only the A/A difference exceeds 0.01.
- Positive control: results of both algorithms checked against math.gcd; the bound check shows the instrument can detect violations. Null: binary GCD, where the cswap fraction is 1 by construction. A/A arm: same protocol, different seed (2 vs 1).
## 4. Method
- Pure stdlib Python. Moduli m = random n-bit odd, x uniform in [0,m). divsteps(f=m,g=x) with delta=1; count iterations until g=0 and the iterations with delta>0 and g odd (swap events). Binary GCD loop counted too (steps, cswap per step).
- Sizes n in {12,14,16,18,20,22}; seeds 1000*seed+n with seed in {1,2}; 2 arms x 6 sizes x 20000 inputs.
- Cost: well under 1 minute, under 50 MB, one core.
## 5. Classification
runnable_local. A stdlib toy decides the swap-frequency claim. The circuit-level (Toffoli/qubit) resource count needs a reversible-circuit compiler and is not attempted.
## 6. What a run cannot establish
- No Toffoli count, qubit count, depth or ancilla cost of a reversible point-addition circuit, and nothing about secp256k1-size inputs (n=256) beyond toy extrapolation.
- A low swap fraction does not show a reversible circuit is smaller: reversibility requires uncomputing and constant-time form, and iteration counts are higher (about 2.9n vs 2n).
- Not a speedup over Pollard rho or index calculus, and no ECDLP algorithm is advanced. Results are labelled [pcc:unverified].
