# Fast-Kummer coordinate gate on the frozen public cohort

On 21 checked Scholten genus-two models, the published generic fast-Kummer
projection and differential formulas returned matching nonzero projective
outputs for 84 doubles and 126 additions. The affine divisor transfer passed
336 sampled group-law checks. A 4-by-4 projective action was derived and
checked for each of 105 rational branch-point translations.

The separate exceptional-model panel contains 1,176 cases: 1,008 matching
nonzero outputs, 168 zero outputs, and no wrong nonzero outputs. The full
source-verified cohort contains 1,260 public targets and 8,820 labeled
cases: 8,813 matching nonzero xADD outputs, seven zero outputs, and no
wrong nonzero outputs. All source scalar, label and affine-map checks passed.
The seven zero outputs occur in four target rows at 16 and 20 bits; each
is recorded with input Kummer coordinates and its valid expected Mumford
class in `results/coordinate-exceptions.jsonl`.

The oracle check using K(T) and K(T+G) selected a unique signed class in
all 8,820 cases. This is a sign-disambiguation invariant, not an implemented
fast Recover or a DLP collector. Candidate operation cost, rho cost and
cost ratio remain null. The PDF and graph plot the recorded case counts by
size; the raw JSONL and data dictionary preserve every exception for review.

Primary equations: [Chung, Costello, and Smith §§7.1–7.4](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf).
Rational two-torsion and fast-Kummer model context:
[Gaudry, *Fast genus 2 arithmetic based on Theta functions*](https://eprint.iacr.org/2005/314/).
