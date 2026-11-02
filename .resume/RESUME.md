# Resume: crypto @ claude/isogeny-class-ic-yield-20261102

## Goal
User request (2026-11-02): measure index-calculus relation yield, solve time,
degree of regularity (Dreg), solving degree and related parameters across the
isogeny classes of ECC2K-130 and ECCp-131, and report per-curve differences.

## Lead / hypothesis
Within an isogeny class only transport cost, Aut and model presentation can
differ (research/isogeny_class_difficulty_20261008). Prediction: null (no
curve effect beyond A/A and presentation). Prior ECC2K-130 work:
cryptanalysis/experiments/ecc2k130-isogeny-class/, research/isogeny_conductor_gap_ic_20261010,
koblitz-isogeny ICC study, cryptanalysis/ecc2k130/research/volcano-m83/.
ECCp-131 never measured. A real curve effect larger than the presentation
control would revive the lead; a null closes it.

## Spec
research/isogeny_class_ic_yield_20261102/DESIGN.md (class structure, parts
P1-P3). Per-part PROTOCOL.md in p1_eccp131/, p2_prime_analogues/,
p3_ecc2k130/ (written by Sonnet builder agents; to be frozen before runs).
ECCp-131: p=0x48e1d43f293469e33194c43186b3abc0b, conductor 1, Elkies l<=150:
5,7,13,17,29,37,61,73,83,89,107,131,149 (x2), 67 (x1).
ECC2K-130: conductor 263*146505763881528721, ecdb curve id 22.

## State
- Class structure computed (PARI via cypari2 in a scratch venv; no gp/sage on host).
- Builders for P1/P2/P3 running (harness + smoke only). Nothing measured yet.

## Next steps
1. Collect builder outputs; review protocols; freeze them (commit).
2. experiment-director compiles task cards (route: isolab-eks unless director finds it unavailable).
3. executor-mechanical runs cards; director assesses; main session analyses.
4. Report + graphs + PDF (research-visuals), PR ready for review.

## Runs in flight
none

## Stop and success conditions
Budget: about one day of cluster CPU. Success = complete, controlled per-curve
measurements for P1-P3 with a verdict per metric (curve effect vs A/A and
presentation bands). Not an IC-baseline improvement claim either way.

## Open questions
- Is the isolab-eks route reachable from this cloud session (no kubectl; AWS user `claude` creds present)?
