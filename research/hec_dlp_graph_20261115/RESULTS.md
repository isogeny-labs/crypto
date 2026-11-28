# Real double-LP collection and row-weight calibration

Scientific source353a85552e4d; protocol3331b61aba. Before execution source was
checkpointed at20261115T113606Z-353a85552e4d. This is a count/forecast study,
not a timed solve comparison. New target indices14/15/16on six reused fixtures.

## Terminal gates
18targets x32768trials=589824trials. M1passes: every split divisor matches
its oriented place decomposition in the WHOLEJacobian; publica,bwalk labels
match at72snapshots; all returnedscalars group-check and separately match the
harness. Full and rooted weighted forests both recover18/18. No cap extension,
stopping-time benchmark, measured speedup or statistical-confidence assertion.

Rawdecompositions include distinct non-anchor support0/1/2and reject larger
support; multiplicities/odd-field weights remain. Forest emissions, empty rows,
unique normalized rows, coefficients, active core rank and first target prefix
are separate. Echelon continues until fullcolumnrank, then skips redundant
updates; exactrank remains H+1thereafter. Count workers are terminal.

## Registered M2forecast result: fails
Ratesa,b,cfit ONLYfirst2048trials. Native RK4integrates known GTTDrooted
equations fromzero,step.25. Of36holdouts(8192/32768per target),35pass all three
count tolerances andonefails. Keep the failure: g2p16381seed16,t32768.

|Quantity|Observed|Forecast|Registered tolerance|
|---|---:|---:|---|
|Known off-base nodes|3807|2902.622|passes|
|Raw rooted Full outputs|744|367.595|fails|
|Root-depth sum|13739|9639.259|passes|

Tolerance is observed+5>=forecast/2andobserved<=2forecast+5. It is a broad pilot
test, not a confidence interval. The row miss is close to its boundary, but
no tolerance adjustment or full-stream refit is made. The rooted model has
not passed the registered all-holdout criterion. Its geometry forecasts do
not establish independent rank or the full-forest output distribution.

## Final three-target medians

|g,p|Root outputs|Full outputs|Root/full mean row support|Full/root first target trial|
|---|---:|---:|---:|---:|
|2,1009|15721|16130|3.75 / 3.67|460 / 564|
|2,16381|673|8442|3.91 / 3.76|7231 / 19693|
|3,127|4638|4645|6.69 / 7.07|431 / 482|
|3,1009|2090|2407|8.95 / 11.23|4948 / 6783|
|4,31|1563|1563|5.93 / 5.93|369 / 369|
|4,127|979|986|8.08 / 9.54|1722 / 2211|

Weights use total nonzero entries / ALLemissions including empty, per target,
thenmedian. The initial reducer expressed root weights over nonempty rows;
this was standardized before publication. All root empty counts are0, so no
numeric value changes; original SUMMARY-MIXED-DENOMINATORS.jsonis retained.
No row-count/trial ratio is a runtime gain. Full-row nullity is at least the
rooted subsystem's at every snapshot, matching exact-subsystem monotonicity.
Genus2p16381targetdetermination occurs with some core columns still free;
full rank of every place log is not required to recover the target.

## Exact genus2boundary: justified next gate
For degree<=2Mumford decompositions every row contains at most two unknown
place logs plus two known public coefficientsa,b. Consequently ANYpartition
into small/off-base places leaves at most two off-base entries. Full weighted
forest elimination followed by exact core elimination preserves the projection
of the original equation system onto k, by the prior target-filter theorem.
Thus, for an IDENTICALadmitted raw stream, the first target-determination
prefix is independent of this partition. In particular H=0is algebraically
allowed in genus2and leaves no place-log core: only the target unknown.

This does NOTapply to the root-connected simplification: H0has no FPseed and
its rooted ODE stalls. The full weighted graph can pin a component via an
unbalanced cycle without that seed, then produce a pure public-label relation.
Both coefficient weights and retention of disconnected PPcomponents matter.
For rooted genus2cycles the PPedges carry no small-place contribution, so
the projected row has at most two endpoint-anchor logs plusa,b, independent
of path length. The observed means near4are consistent with this algebraic
bound; this run does not log the per-row maximum as a separate measurement.

This is a specialization of established two-LPelimination, not a novel
exponent or an achieved10xmechanism. Before timing, gate H0versusH>0on the
same stream and compare target-determination prefixes exactly. Genus3/4need
different off-base handling or genuine changes in relation supply; H0does not
automatically retain all degree3/4rows. Do not narrow the full objective.

## Reproduce / standing
build.shdiscovers the native rlib and records source/fixture/library/binary
hashes. ODE_GATEcovers48closedform checks andzero-FPcontrol. RAW.txt/stderr
retain all72SNAPand18TARGETrecords plus terminal outcome. summary.jqreduces
counts; render.rsdraws recorded medians; one-page TypstPDFis inspected.
No scientific Python, cloud worker, measured benchmark record/default change,
or leaderboard admission. Existing cold losses remain unchanged.

Primary framework: GTTDSections3.3,4.1--4.4,5,
https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf. Its source assumptions
are not a novelty or calibrated wall-time claim for this implementation. New
complete costs and independent replay/novelty assessment remain required.
