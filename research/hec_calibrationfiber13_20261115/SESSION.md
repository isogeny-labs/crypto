# Calibration fiber handoff

## Outcome
V4passed once after d8d3ca3d599e/checkpoint224740: degree18fiber roots54/112,
sourcepoints54,56/112,65, two nonzero ell-calibrationpairs, exact124regular
controls and completezeroset. No generator-multiple scan or relative scalar.
V1failedwrongdegreebound; V2passedalgebra butfullsplitchord restrictionfailed;
V3partialchorddecoded butoldnegativecontrol cardinalityfailed. ALLretained.

## Decisions
Use any calibrationbase, not originalD1. Norm saturation30-12=18. Factor
chordquartic instead of scanningtargetpoints, admit2rationalhits. ErasedHessian
control is non-discriminating at2hits. Final oldMcheck is verificationonly.
Leanconditionaldegreebound checkedexit0, standardaxioms, explicitgeometrygap.

## Next
RefitMfromfirstnewpair; mapBOTH D1andB using fixedtranslations bynewknownbase;
inverse, publicadjacencysign, freshpointlogs/conicdescent; coldquotientinputs
mustreplacecopiedkernel/alpha. Fullcostcomparison/genus2/4stillopen.

## Publication
RESULTS/protocolamendments/raw/PDF/Leanproof. Preserve original dirtyresume.
Shared dailylog ownedelsewhere; thisstudyhandoffrecords currentwork.

Publication checks: V4 and Lean exit0; axiom list only standard three. The
one-page PDF rendered and visually inspected. Initial Typst build rejected an
unescaped wildcard; fixed in report prose before successful rendering. No
scientific execution repeated during reporting or formalization.
