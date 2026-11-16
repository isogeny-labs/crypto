#set page(paper: "a4", margin: 17mm)
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)

= K13 rootfinding: independent L0 arithmetic audit

*Scope.* Post-hoc audit of archived ecbench session `ECBS1hc322a24b6610`
(source commit `8f4cc824c87a`, PR #4407). The raw JSONL object is
453,126 bytes, SHA-256 `dc16c97ae21a250c...`. This report adds no
measurement. Existing claim `PCC-20261116-3b7488` remains `not_covered`;
these are descriptive L0 values. [pcc:unverified]

== Independently checked

The Python auditor selected the least sparse irreducible degree-13 polynomial,
`0x201b`, and used its own binary-field and curve addition to check the
generator, subgroup order, both targets and all 48 recovered scalars.
It pinned the curve, factor-base identities, binary digest, session, target
seed and all four exact measured target-round pairs. A one-step wrong scalar
and a zero clock were rejected. The raw machine-readable output is
`RESULTS.json`; source is `audit.py`. [pcc:unverified]

== Paired complete-process records

The plotted quantity is summed batch-m3 wall divided by summed reference
wall over four exact pairs. The F4/F5/F6 ratios were below one in each
archived pair. The batch A/A pair ratios range from 0.785 to 1.316,
so this session cannot qualify a timing claim. [pcc:unverified]

#image("ratios.svg", width: 100%)

== Remaining measurement

The source process clocks have no independent execution attestation, and
macOS cannot earn ecbench L2 here. The frozen EKS L2 same-target panel and
its A/A gate remain the required timing test. Its resource gate and the
separate branch-specific resume checkpoint still block that work. The
audit does not change the algorithm's asymptotic cost or the rho comparison.
[pcc:unverified]
