# F631 quotient geometries: one source sign, one opposite sign

Source freeze a70cc5d9dd88, pre-run checkpoint20261116T014404Z. Both fixed kernel cases executed once in separate processes. All statuses0/stderr empty; no rerun, omitted kernel or post-outcome coefficient tuning. Fresh F631kernels and54normalizationweights are inputs, not any F127HC/QF or map data.

| Kernel | Gamma00 rank | Global quadric rank / rows | Quartic smooth | Counted F631points | Sign |
|---|---:|---|---|---:|---|
|0|7|36 / 64|yes|597|source|
|1|7|36 / 64|yes|667|opposite|

The source has N1=597 and nonzero trace; the opposite count is2*(631+1)−597=667. This is a source-sign admission control. It is not an independent full Weil polynomial, Jacobian cardinality or transfer-map verification. The opposite-sign kernel remains recorded and is NOT admitted for the current source-transfer path.

## Exact construction

Principal functions are rebuilt from the actual kernel input. Fresh primitive cube root and three-character trace basis supply the eight descended sections. The degree-four local jet calculation checks exact curve residuals, base-field descent and vanishing of odd coefficients. Unique rank7Gamma00relation gives the plane quartic; smoothness and64source-multiple section samples give rank36globalquadric exclusion. Jet-ring controls pass.

The rational2torsion chart is t.u=[436,451,7,1], t.v=0, independently admitted in the prior local study. Projection at x0 initially has order4, so one explicit doubling is required to obtain order2. The old assumption that every nonzero ell projection already has order2 would fail here; projection_depth1is recorded for both cases.

Kernel0 HC=[156,615,212,277,71,173,170,1], QF=[1,627,254,511,40,66,318,33,29,463,86,201,389,519,310].

Kernel1 HC=[35,630,606,77,268,402,363,1], QF=[1,550,515,146,323,263,170,121,519,1,564,451,2,341,432].

The full coefficient files and raw controls sit in run/kernel0/ and run/kernel1/, with adjacent stdout/stderr/status. These are newly constructed models; no expected quartic coefficients were supplied.

## Scope and next verification

This establishes smooth candidate target geometry and its point-trace sign, beyond merely torsion or normalization. It does not establish the actual source-input images, calibrated source-to-target correspondence, full target order, complete target Weil polynomial or DLP recovery. Kernel0 is selected for further work by the preregistered source-sign criterion; retaining kernel1 prevents a hidden positive-only report.

Next work must verify the actual transfer on kernel0, including target subgroup/order checks, calibration and both original public input images, then complete destination IC and source/destination scalar verification. Construction and all controls remain chargeable in a future frozen comparison. No timing, exponent, novelty or tenfold gain is claimed. Genus2/4 and the full mathematical objective remain open; the earlier failed charged cold-cost comparison is unchanged.
