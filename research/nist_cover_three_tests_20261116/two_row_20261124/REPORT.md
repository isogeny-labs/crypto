---
title: "P-192 tangent relations: two rows cannot close the frozen batch"
date: "2026-11-24"
geometry: margin=0.75in
fontsize: 10pt
---

The frozen-data audit finds no necessary norm identity for cancellation
between two distinct rows from the selected split-tangent family. All 30906
queries agree under separate GMP and OpenSSL arithmetic. Under the recorded
two-point residual trace identities and the written support classification,
any extra relation supported on the original base requires at least three
selected tangent rows. This is a finite producer-unverified derivation,
not a signed formal proof. [pcc:unverified]

## The hypothesis and its necessary tests

Keep the existing P-192 kernel3 plane quartic and degree6 map f. Let A contain
O and both signs of each original projected base point. For tangent i, the
residual trace is $T_i=f(R_i)+f(S_i)=\kappa-2f(P_i)$. Anchor and reference
images already lie on old columns. The preceding pair audit excludes
$T_i\in A+A$; this worker recomputes that premise for the selected subset.
No new quartic points or residual roots are reconstructed. [pcc:unverified]

H-I asks whether two tangent rows could eliminate every outside-base column.
It includes arbitrary nonzero coefficients modulo the P-192 prime subgroup
order, equality after elliptic projection, negative images and identity images.
Each nonzero residual new part has one of three forms, up to sign:

| Type | New-column part | Residual trace |
|---|---|---|
| A | X | X+q, with q in A |
| B | 2X | 2X |
| C | X+sY, with distinct columns and s=+1 or -1 | X+sY |

Two parts can cancel only when proportional. Different support sizes cannot
cancel. Two C parts, or two B parts, require $T_i\pm T_j=O$. Two A parts
require $T_i\pm T_j\in A+A$. Ordered A/B parts require
$2T_i\pm T_j\in 2A$. Because O belongs to A+A, the first query family also
covers B/B and C/C. The one-row premise excludes a canceled new part within
one row, including two opposite residual images. This is a written exhaustive
support argument, conditional on the recorded two-point geometry.
[pcc:unverified]

## What actually ran

The [protocol](PROTOCOL.md) and native source were published before calls.
The old baseline and trace inputs have exact SHA256 pins. The worker rederives
the reference and all recorded traces, recreates the complete signed pair
table, and checks each query with both arithmetic implementations. Grouped
and independently sorted joins retain every witness. Complete deterministic
stream fingerprints and the exact selected indices are recorded, rather than
duplicating every coordinate in the result. [pcc:unverified]

| Fixed domain | Queries | Witnesses |
|---|---:|---:|
| One-row premise, selected traces versus A+A | 102 | 0 |
| Unordered distinct traces, both signs, versus A+A | 10302 | 0 |
| Ordered distinct traces, doubled first trace, both signs, versus 2A | 20604 | 0 |

The selected subset has 102 rows; A has 451 symbols and its complete unordered
pair table has 101926 records. Eleven native controls pass, including positive
pair/double retrieval, negative and identity symbols, an unequal-coefficient
cancellation, changed witness, missing stream record and off-curve input.
The create-new output collision and unknown CLI controls reject; collision
checks preserve both existing result files. Result and all controls are
producer-unverified, with no signed mathematical verdict. [pcc:unverified]

![Necessary tests and the remaining mechanisms. Counts are fixed finite query counts, not timing or success-rate estimates. No uncertainty interval applies to this exhaustive finite audit.](mechanism.pdf){width=95%}

\newpage

## Consequence and remaining hypotheses

H-I's finite exclusion is supported: neither necessary query family has a
witness. Any combination adding old-base rank therefore needs at least three
selected tangent rows. Adding original baseline rows cannot help cancel new
columns, since those rows already have zero new part. This does not show that
larger combinations exist, or that the complete batch has zero gain.
[pcc:unverified]

Two concrete successor hypotheses distinguish possible mechanisms:

* **H-J: longer shared-image cycles.** Individual residual images coincide
  across at least three selected rows and their signed coefficients permit
  cancellation. For a three-edge cycle with two distinct new columns in each
  row, a necessary test is $\pm T_i\pm T_j\pm T_k=O$. A cycle in an unsigned
  drawing alone is insufficient; the coefficient kernel must be checked.
* **H-K: paths ending on old columns.** Some rows have one residual image in
  A and one new image, linked by rows with two new images. A three-row path
  requires $\pm T_i\pm T_j\pm T_k\in A+A$. Actual projected memberships and
  signs, rather than only trace sums, determine whether the path exists.

Both are proposals, not observations. The already prepared admitted collector
reconstructs individual residuals, checks target principal classes and maps
their support. Its complete left-kernel witnesses and separate rank checker
can test these mechanisms without selecting favorable rows. Any witness with
only one or two nonzero added-row coefficients would contradict this finite
audit or its recorded premises and must be investigated, not counted as yield.
No added search window or new production run is substituted here.

## Evidence, accounting and limits

Record PCC-20261124-df73b8 pins the source, protocol, result, manifests and
successful build/worker diagnostic costs. Record PCC-20261124-82e3a6 binds
the additional accounting receipt and failed-build/control diagnostic costs.
Evidence is in
[qualification-v2](evidence/qualification-v2/result.json). Separate arithmetic
backends share producer-written enumeration and case analysis; they do not
supply an independent signed verdict. The scalar record
IDC1hca5ea189578c2651 evaluates $b(aX+q)-a(bX+r)=bq-ar$ at 32 points over
$2^{61}-1$ with changed-constant rejection. It checks bookkeeping only.
Its reused issuer's source field still names the parent report; exact new
issuer input is pinned through this audit's source and claim. [pcc:unverified]

| Attempt/stage | Exit | Wall diagnostic, seconds |
|---|---:|---:|
| Initial driver, missing evidence parent | 1 | Not recorded |
| First strict build, ambiguous SSL type | 1 | 1.557 |
| Corrected strict build | 0 | 6.105 |
| Frozen-data audit and scalar record | 0 | 1.737 |
| Existing-output collision control | 1 expected | 0.005 |
| Unknown CLI control | 1 expected | 0.004 |

All raw stdout/stderr and user/system diagnostics are retained, including
failed source/build snapshots. Worker cap 120 seconds/1GiB; strict build cap
300 seconds/1GiB. These local process clocks are development diagnostics, not
admitted benchmark or end-to-end construction costs. Metadata preparation,
publication and rendering have no complete process clock. No speed comparison
or aggregate cost claim is made. No new curve point, residual root or actual
collector call occurred. Original map-synthesis/setup accounting remains open.
[pcc:unverified]

Sources checked: exact parent reports and frozen native arithmetic; official
[Magma index-calculus documentation](https://docs.magma-maths.org/v2.30-1/AlgebraicGeometry/AlgebraicCurves/linear-equivalence.html#index-calculus).
KB unavailable: no loaded tool and repository wrapper has no executable.
The support classification and pair joins use elementary group and linear
algebra; no novelty claim. Smith's population estimate for random genus3
hyperelliptic Jacobians is not a probability assigned to this elliptic-cover
family. This audit concerns a fixed constructed cover and fixed relation batch.

Prior reports, protocols and canonical performance figures remain byte-equal;
only this study's diagnostic diagram is added. The original requested maps,
exceptional representations and target checks stay covered by their retained
components, with independent signing still open. New collection remains
subject to the existing admission rule in
[isolab-eks](https://github.com/isogeny-labs/workspace/blob/main/.agents/skills/isolab-eks/SKILL.md):
“Stop new launches when the budget is exhausted or its remaining balance is
unknown.” Existing role denial prevents observing a balance; no denied probe,
paid launch, IAM change or signer bypass was attempted.
