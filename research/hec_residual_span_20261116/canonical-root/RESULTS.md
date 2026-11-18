# Canonical root replacement passes; full setup gains scoped

Pushed freeze477fdad9193, single invocation exits0. All frozen hashes pass.
175eligible prime/order pairs with q<=1000produce exact oldcanonicalroot.
100counted alternating calls repeat exactly. Every full(FB,z,character)tuple
matches unchanged oldsetup on all8curves. No rootorientation or curvechoice
changes. Full raw counts and SUMMARY.tsv retained.

Standalone root-discovery REDC old/new2.269-225.350across17declaredpairs.
Exampleq4001,n5:9014to40. Deterministic powermap needs1-10trials on this
panel; no expected/worstcaseconstant trialclaim for arbitrary q.

|p|n|c|old fullsetup REDC|new fullsetup REDC|ratio|
|---|---|---|---|---|---|
|101|5|2|682|338|2.017751|
|251|5|18|960|776|1.237113|
|601|5|7|1818|1514|1.200793|
|113|7|3|610|440|1.386364|
|421|7|2|1476|1102|1.339383|
|19|9|2|326|300|1.086667|
|37|9|2|404|342|1.181287|
|73|9|2|470|468|1.004274|

Fullhelper reuses already-computed primitivegeneratorpower, avoiding both old
rootsearch and redundant power-map trialwork. This is standard group algebra/
engineering, not novel mathematical IC. Genus4rows qualify setup equality,
not subgroup strength/DLP/absolute simplicity. Order4gate qualifies a root
primitive, not a whole Gaussian solver. No CPU/wall/fullsolve10xresult follows.

Next: prospective pipelinecopy must give both control and candidate this
optimization and preserve knownfield/subgroup operation vectors/outputs.
Earlier frozen sources/importhashes remain untouched. Do not compare new
candidate against deliberately unoptimized oldrootcontrol as a full gain.
