# Controlled compiled-map cohort: stability fails,10xrejected

All20cells pass both original-class checks on one frozen cohort. Source0911e248f439/checkpoint20261116T035102Z precedes execution under record20261116T0351Z-hec-tensor631. An initial launcher named a nonexistent0350record and exited before starting the collector; the actual0351record then launched once. No cell rerun/replacement. Raw inputs/outputs/statuses/phases archived in content-addressed S3.

| Label | Median component s | Observed min–max s | Max/min | <=1.25 |
|---|---:|---:|---:|---|
|Baseline A|1.478166542|1.448523250–2.065583333|1.425993|FAIL|
|Baseline B|1.462773166|1.451185833–1.473323334|1.015255|pass|
|Compiled A|1.290368459|1.275596250–1.407780416|1.103625|pass|
|Compiled B|1.287459000|1.275094542–1.303754042|1.022476|pass|

Baseline A/Bmedianratio1.010523419733and compiled A/B1.002259845945both pass<=1.10. However baseline A's first cell breaks the frozen spread threshold; it is retained. Overall stability qualification fails. Primary medianBaselineA/medianCompiledA=1.145538339604, below10. No qualified stable speedup/regression claim. No first-cell deletion, threshold adjustment or pooled Bcontrols. Repeated fixed pair, shared ARM host; extrema are not confidence intervals.

## Diagnostic medians, not additive medians

For compiled A, generatorD1=272.802125ms, generatorB=267.481125ms, solverD1=118.346667ms, solverB=118.669750ms, verificationD1=24.763791ms, verificationB=23.317458ms, reconstruction=448.754834ms. These disjoint child stages include logging/spawn/wait; their medians need not sum to the median total. Inside reconstruction, modelconstruction=107.776208ms,inputD1=167.495500ms,inputB=166.037250ms. Inclusive aggregate groupadd=331.973289ms,groupneg=12.729247ms,sectioncoords=4.323505ms are nested diagnostics and must not be added to enclosing phases. Full baseline and per-cell phase records retained in SUMMARY-TIMING/raw archive.

Posthoc Rust bounds over all20cells: with the measured remainder fixed, zeroing recorded section scope costs gives maximum total speed factor1.003492375198. Zeroing even the sum of inclusive groupadd/groupneg/section costs gives a generous maximum1.886930471206; group overlap is double-counted, making this bound more generous. These are counterfactual accounting bounds on this recorded workload, not preregistered success gates or generic lower bounds. Scope instrumentation overhead outside those recorded durations remains in the stipulated unchanged remainder.

## Decision

Do not promote the observed1.146ratio to a qualified speedup. Retain exact arithmetic equivalence, but reject10xfor this mechanism/workload. The phase evidence rejects further section-only microtuning as a plausible10xroute with other costs unchanged. A next construction must change repeated section/equation setup and inversion itself, or justify a different admitted regime; simply optimizing recorded group arithmetic cannot close the whole gap under this fixed-remainder model.

Common source/quotient/kernel/calibration/publiccoordinates and final conicdescent remain outside the inverse-component boundary. Tensor compilation/dual maps are charged. Earlier1.621xand noisy0.930cohorts remain immutable/separate. No novelty, globalmap, fullcold/genus2/4or mathematical10xclaim.

Initial postprocessing reducer also matched unrelated TRACE_PASSlines and stopped before producing statistics. It was corrected to parse only TRACE_PHASE/TRACE_AGGREGATE; empty SUMMARY-INITIAL retained. Scientific cells were not rerun. Standard benchrec and S3 custody handle measurements; all scientific reduction is Rust.
