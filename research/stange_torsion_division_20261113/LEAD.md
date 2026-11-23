# LEAD: Stange — division polynomials for arbitrary isogenies

Date recorded: 2026-11-13. Status: **lead recorded, not run.** Every claim
below is `[pcc:unverified]` unless it names a pcc claim id; none does.

## Citation (verified by web search, 2026-11-13)

Katherine E. Stange, *Division polynomials for arbitrary isogenies*,
arXiv:2503.15428 (math.NT, March 2025, later revisions incl. v3 reported dated 2026-04-17 [pcc:unverified]); also IACR ePrint
2025/521. https://arxiv.org/abs/2503.15428 , https://eprint.iacr.org/2025/521

Abstract-level content (from the arXiv/ePrint listings; full text not read in
this session) `[pcc:unverified]`:

- extends the Mazur–Tate / Satoh isogeny division polynomials to isogenies
  whose kernels do **not** sum to the identity (the "biased" / arbitrary case
  previously excluded);
- proves analogues of the classical properties: chain rule, relation to the
  x-coordinate, recurrence relations, links to classical elliptic functions;
- odd degree: agrees with the classical division polynomial; even degree:
  multiplicities at 2-torsion can differ (torsion-aware normalisation; Sage's
  `division_polynomial` exposes the variant choice);
- extends the construction to products of curves, analogously to elliptic nets.

## Owner's assessment (recorded as given)

Closes a **certification gap**; does **not** solve arbitrary-target lifting
or dependency discovery.

## What it closes `[pcc:unverified]`

- A canonical, normalised division-polynomial object for *every* separable
  isogeny, including those with kernel sum ≠ O. Our prior work implicitly
  assumed the multiplication-by-r / classical psi_r setting
  (H109 uses `psi_r` and `D psi_r` for [r]; the EDS/net notes use
  `psi_n` of [n]). For isogeny-walk evidence (volcano walks, Vélu chains)
  a verifier can now check a division-polynomial certificate for the actual
  isogeny, with the even-degree / 2-torsion multiplicity ambiguity fixed by
  convention instead of left implicit.
- Chain rule gives composable certificates along isogeny chains; recurrences
  give a checkable evaluation procedure.

## What remains open `[pcc:unverified]`

- **Arbitrary-target lifting:** nothing in the construction maps a target
  Q = [k]P to a lift or relation carrying k. Evaluating any of these
  polynomials at Q is still a GGM-simulable zero/non-zero or value oracle
  unless a new nonlinear use is found (cf. LEAD-EXO result below).
- **Dependency discovery:** no mechanism to find relations among targets /
  factor-base points; the index-calculus relation phase is untouched.
- No cost claim, no complexity theorem, no change to any leaderboard row.
  This is not an improvement over the IC baseline under the success criterion.
- Whether the even-degree variant gives *any* 2-power-character signal beyond
  H109's frozen family is untested.

## Relation to prior lead files

- `research/torsion_division_derivative_20261031/` (H109): tested D psi_r at
  targets for a constant-size 2-power character parity formula; 0/28 exact
  formulas, scoped null. Classical [r] only; Stange's arbitrary-isogeny
  polynomials are outside its frozen family, so H109 neither covers nor
  refutes this lead. Overlap: same torsion-local division-polynomial mechanism
  family.
- `research/lead_sweep_20261110/LEAD-EXO-elliptic-net-eds/`: zero-test net/EDS
  locator is linear-cost (slope gap 0.46–0.68 vs BSGS), `[pcc:unverified]`
  single toy run. Stange's product-of-curves extension is the elliptic-net
  analogue and inherits the same obstruction for zero-test use.
- `research/notes/cm-isogeny/RESEARCH_EDS_RESIDUE.md`: Lauter–Stange
  equivalence (EDS problems ⟺ ECDLP) and the "information-rich yet
  algorithmically inert" verdict on residue bits; the rank-2 net was built
  without Stange's seeds. The new paper does not alter that equivalence.
- `research/notes/ecdlp-general/` (STATE_OF_THE_ART, NZMS_AMS_AUSTMS_2024_IDEAS,
  TORSION_AUXILIARY_INPUTS): mention Stange/elliptic nets; none cites
  arXiv:2503.15428 in the lines read here (repo-wide grep for "2503.15428"
  timed out; absence not established).

## Sources checked

- research-kb: **not queried** — `scripts/research-kb` fallback failed
  ("no executable found; set RESEARCH_KB_BIN"); MCP server not available in
  this session. Novelty / "not tried" is therefore **not established**.
- Local grep: `grep -ril stange research/` → the four paths above.
- Web: arXiv:2503.15428 abs/html listing; IACR ePrint 2025/521 listing.

## Next step (if pursued)

A certification-only task: implement the arbitrary-isogeny division
polynomial as a verifier for existing Vélu/volcano isogeny evidence, with
odd-degree agreement against classical psi as positive control. Any ECDLP
use needs its own frozen protocol (PROPOSAL.md, separate author).
