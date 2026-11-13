# The special-fiber norm equations force the origin

2026-11-13. Source freeze `5e342cafb852fe15d8744969dbd6de04f3561e77`;
literal-issuer correction `aec2bb861`. All claims unsigned [pcc:unverified].

The explicit pole-scaled norm/membership family now has a Lean implication:
at parameter zero and Dickson depth4, vanishing of the four nonconstant
remainder coefficients forces `a1=a2=c0=c1=0`. The result holds over every
field, including extension fields. The family at parameter one recovers
the original symbolic target-completed norm quotient and Dickson master.
This establishes a precise zero-set fact for the explicit family; its use
as the actual reduced input's weighted initial system still needs a faithful
coefficient/filtration binding [pcc:unverified].

Central claim: `PCC-20261113-7b7707`. [Proof source](../lean/NormWeightedLeading.lean),
[protocol](../WEIGHTED_LEADING_PROTOCOL.md),
[claim map](../weighted-leading-evidence/claim-pins.json),
[editable diagram](special-fiber.dot), [SVG](special-fiber.svg), [PDF](report.pdf).

![Constructed family endpoints and the still-proposed initial-form bridge. Solid arrows correspond to Lean endpoint/implication results; dashed arrows are unresolved. All evidence is unsigned.](special-fiber.pdf){width=90%}

## Construction and readable proof

Use the retained coefficient order `(a1,a2,c0,c1)`, proposed pole weights
`(4,2,3,1)`, and X weight2. Put `D=X^3+a2*X^2+a1*X`, `B=c0+c1*X`.
For parameter e, curve `f_e=X^3+e^4*a*X+e^6*b`, target
`(q_e,z_e)=(e^2*r,e^3*z)`, and original curve equation
`z^2=r^3+a*r+b`, the scaled target lies on `f_e`.
Use the existing actual `completeTarget` operation:

```
A_e = D + (z_e*B(q_e)-D(q_e))
U_e = (A_e^2-f_e*B^2) div (X-q_e).
M_0 = X; M_(n+1) = M_n^2-2*e^(2^(n+2))
R_e,n = (M_n-terminal*e^(2^(n+1))) mod U_e.
```

The norm division is exact. At e=1 these are the original symbolic norm
quotient and Dickson recurrence; at e=0 the exact norm identity gives

```
U* = X^5 +(2*a2-c1^2)*X^4 +(a2^2+2*a1-2*c0*c1)*X^3
     +(2*a1*a2-c0^2)*X^2 +a1^2*X
R_0,4 = X^16 mod U*.
```

`U*` is monic of degree5 and has constant zero. The remainder has degree
below5; its constant is zero because both dividend and divisor vanish at0.
Thus its four nonconstant coefficient equations imply the whole remainder
is zero. Consequently `U*` divides `X^16`. The prime-power divisor theorem
and monicity give `U*=X^5`. Comparing coefficients, first `a1^2=0`, then
`c0^2=0`, then `a2^2=0`, then `c1^2=0`. A field has no nonzero nilpotents,
so all four coefficients vanish. No characteristic restriction is needed.
The actual remainder criterion also has a converse at every exponent n>=5
[pcc:unverified].

The zero tuple of this special fiber is not a recovered solution of the
original parameter-one problem. No theorem transports that tuple across
the family, assumes flatness, proves regularity or supplies an efficient
finder. A weighted homogeneous zero-set argument and a low weighted
last-fall bound are distinct obligations [pcc:unverified].

## Checks, failures and experiment boundary

The module has18 new statements. Build007 records standard foundations
`propext`, `Classical.choice`, `Quot.sound`. The whole-study gate reports
304 statements and0 additional trusted uses, exit0; import-inclusive fresh
replay also exits0. Validation claim `PCC-20261113-582ee7` binds the
[receipt](../weighted-leading-evidence/validation-001.json) with exact
source/artifact hashes and preserved exits.
The module uses standard `import all` access to existing definitions;
no earlier mathematical source, checker, dependency pin or gate changed
[pcc:unverified].

Literal certificate `IDC1h0e1e85cab8ff464b`, claim `PCC-20261113-027206`,
encodes only the literal special-fiber norm expansion displayed above.
The unchanged issuer/checker accepts32 statement-derived and16 extra points.
It is unsigned and does not certify the geometric implication, Lean execution,
initial forms or a solver. The first issuance rejected degree6: ordinary
syntactic degree is7, from `X^3*(c1*X)^2`. Its failed log/exit is retained
under `PCC-20261113-aea26a`; the correction changed no expression or
mathematical acceptance criterion [pcc:unverified].

The adapter's22 canonical tests and format/build pass in the saved receipts.
Strict clippy records four `chunks_exact_to_as_chunks` style failures in the
unchanged canonical SHA256 under Rust1.99.1. No suppression or canonical
source edit was made. Old source/evidence/claims remain intact
[pcc:unverified].

The preceding projection's saved benchrec diagnostic remains1680 historical
sparse-artifact issues across162 unchanged records. This component changes
no benchmark record, generator, progress or gate path; the diagnostic is
preserved rather than relabeled as a pass [pcc:unverified].

The fixed native fixture-to-leading-form experiment did **not** run.
Flow: planned1, attempted0, completed0, excluded0, admission-blocked1.
Checkpoint gate first required a resume; after supplying it, upload transport
was unavailable: no configured profile or checkpoint token. Both failures
remain. The tracked root resume belonged to another study and was restored;
this task's resume is saved under `weighted-leading-evidence/`.
No AWS/vault request, paid job, F4/msolve, branch search or performance
measurement occurred. Local Lean proof checking and literal algebra
certificate issuance are mathematical construction, separate from that
unlaunched experiment [pcc:unverified].

## Sources and next discriminating obligations

Sources checked (KB): unavailable as already recorded. The bounded direct
fallback and precise URLs are in
[prior-search-receipt.json](../weighted-leading-evidence/prior-search-receipt.json).
Read ml-cryptanalysis at `8020e131a1a3806531d695b6bc7586473802dada`:
formal README and actual Closure/VariableClosure Lean sources. Full iterated
closure permits multiplication of newly derived low-degree rows; static
Macaulay multiples alone do not establish its closure. Those existing
ordinary-degree results are not new weighted ECDLP bounds [pcc:unverified].

Read Faugere, Safey El Din, Verron, arXiv1412.7547v2, section3.2 equation11,
Theorem12 and section4. Weighted bounds have regularity/homogeneity/position
hypotheses and add input degrees; they do not automatically bound a solver
by its maximum input degree. The proposed weights are not reverse
chain-divisible. Neither theorem is applied to the concrete source here
[pcc:unverified].

1. Prove the exact pole-normalization/filtration map, or complete the frozen
   symbolic fixture binding once checkpoint upload admission is available.
   The predicted residual weights24,26,28,30,30 remain predictions.
2. Establish a weighted normal-form/iterated-closure certificate with finite
   budgets. A low degree at one fixed fixture is not an asymptotic bound.
3. Prove a growing-family weighted last-fall bound and include decomposition
   yield, finite-field extraction, supplied-label relation collection and
   all expected bit costs. No generic regularity assumption fills this gap
   [pcc:unverified].

The original W-jet/rank proposal remains a separate family without a faithful
public map. No jet/rank attack or non-generic lower bound follows here.
Existing global scoreboard, timeline and curve diagrams remain unchanged:
this component has no new measured speed or certified transport edge.
The study-local diagram explicitly separates constructed endpoints from
the proposed weighted-initial-form bridge [pcc:unverified].

## Exact proof map

Every row has the exact field and parameter hypotheses in the linked Lean
declaration; the binding hash and source commit are in claim-pins.json.
All entries are unsigned [pcc:unverified].

| Lean declaration (namespace NormWeightedLeading) | Unsigned claim |
|---|---|
| `monicDivisorPower` | `PCC-20261113-dc02f2` |
| `supportMonic` | `PCC-20261113-2bbd73` |
| `supportDegree` | `PCC-20261113-e3477f` |
| `supportPowerForcesZero` | `PCC-20261113-96dfec` |
| `supportDivisorForcesZero` | `PCC-20261113-2e2e93` |
| `supportRemainderForcesZero` | `PCC-20261113-c31867` |
| `supportRemainderIffZero` | `PCC-20261113-bbbc67` |
| `scaledCurveAtTarget` | `PCC-20261113-e562e9` |
| `leadingNormIdentity` | `PCC-20261113-fe03ff` |
| `scaledSupportZero` | `PCC-20261113-d4d4af` |
| `scaledSupportOne` | `PCC-20261113-677b73` |
| `scaledSupportExact` | `PCC-20261113-2868f3` |
| `scaledFiberRemainderForcesZero` | `PCC-20261113-33be81` |
| `fourCoefficientsForceZero` | `PCC-20261113-06284b` |
| `scaledMasterZero` | `PCC-20261113-e00654` |
| `scaledMasterOne` | `PCC-20261113-2bcb8a` |
| `scaledMembershipZero` | `PCC-20261113-45c790` |
| `membershipFourCoefficients` | `PCC-20261113-7b7707` |
