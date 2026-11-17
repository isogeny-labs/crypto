# Fresh genus2 incidence oracle: no tenfold fixed-core concentration case

Frozen e6655cd3dd1 once-only producer exits0. Six fresh target/source workloads on the original3primeorder genus2models,16anchors each, retain5785squarefree split degree3residuals. Every residual reconstructs its exact polynomial and satisfies anchor+residual=0 in the original Jacobian. All nonfixedpointorbits have size5, so pointfractionrho=b/r is exact for an orbit-unioncore. No scalar solve or performance result is claimed. [pcc:unverified]

|p|source multiple|folded columns|retained rows|max upper bound over all fixed core sizes|
|---|---:|---:|---:|---:|
|101|1|7|106|3.500000000|
|101|17|7|112|3.437500000|
|251|1|27|759|3.450592885|
|251|17|27|750|3.492000000|
|601|1|68|2004|3.664670659|
|601|17|68|2054|3.376825706|

SUMMARY.tsv reports everyb=1..r:204cuts total, not a favorable chosen subset. For each workload, sum oforbit rootdegrees equals3M. Sorting degrees and summing thetopb gives an incidence upperbound for ANYfixedb-orbitset, even one chosen with free complete knowledge of all retainedtargetrows. If I_top is thatsum, acceptedrowcountA<=min(M,I_top), so the acceptedfullrow-per-scanratio cannot exceed min(r/b,r*I_top/(b*M)). Repeated incidences from one row can only make this optimistic bound looser; it is not claimed attainable.

Thus no fixed orbit-unioncore on any of these complete16anchorpanels can supply10xacceptedrowyield per scan through concentration alone, even before selection,setup,quadraticfactoring,LPelimination andverification costs. This is a deterministic finite-data bound, not a random-root extrapolation. The target construction scalar stays outside the probeAPI and is not used for selection. No paid pilot cost is made free in a proposed production algorithm: the freeoracle is deliberately optimistic to falsify a premise, not a performance arm.

Scope: the samecoreisfixed across its16anchors. The result doesnotbound per-anchorchangingcores, otheranchorpanels/curves/fields, a differentroot/rowfamily, weighted useful-rank per retainedrow, improvedmatrixcost or fullruntime. It also doesnotprove genericimpossibility. Any of those alternatives mustprovide a newconstruction/costcase rather than attribute10xto fixedcoreconcentration thatthispanelcontradicts. Genus3/4andbroadmathematical10xgenus2+remainopen; genus2bottleneckscreen isnot objectivecompletion.

RawROOTS.tsv retains orbitindices andsignedoriginalpoints for each row, allowing futureaudits without producer rerun. CONTROL.stderr retains originalpublicsource/targetfixtures, qualification scalars, rowtotals andgroupgate status. Originalrun/exit/source/native/import/binaryhashes andfrozenRustsummary retainedinGitduringS3outage. No producerretry,anchor/targetselection,budgetextension,S3polling orcloudspend.
