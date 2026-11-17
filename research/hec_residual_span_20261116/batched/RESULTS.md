# Batched inversion: exact stream, explicit arithmetic tradeoff

Once-onlyexecution at pushed71060297990 exited0. All774helpercases matchscalarinversion andtheexactcostformula (n0:0inversions/0multiplications;n>0:1inverse+3(n−1)multiplications). All144solvesverifyoriginalsource/target, repeatedvectors/outcometuplesmatch,andREDCidentitiespass. Batched/scalarPencil firstdeterminingtuplesmatchexactlyall18;Folded/Expandedtuplesmatchall18. Source-normalizationflagsunchanged. No favorableanchor/parameter selection orbudgetchange.

|Genus|Savedbasefieldinversecalls perworkload vs scalarPencil|Folded/Batch basefieldinversecalls|Folded/Batch basefieldREDC|
|---|---|---|---|
|2|32–68|1.281553–2.575000|1.193960–1.712046|
|3|239–277|3.535714–17.698925|2.181886–7.821305|
|4|245–431|3.897059–13.042553|2.382984–5.999307|

Ranges are sixseparateworkloadspergenus, notpooledmeans. Ratiosarespecifiedprimitivecounts, notwalltime orweightedcompletecost. For everyworkload, postrunRustanalysis verifies exactdelta: addedFp::mul=3*savedinv and addedREDC=6*savedinv. Subgroupfieldvectorsareidenticalbetweenbatched/scalarPencil. Thisisolatesarithmeticchangefromrelation/rankselection. FullvectorsretainEuclideaniterationcountsandotherwork; nestedcolumns mustnotbesummed.

Batchingtradesinversionsformultiplicationsandextraarrays, preservingpoint/sign/bucketprocessingorder. UnderanexplicitmodelwithconstantcostIperinverseandMperfieldmultiply, thatlocalreplacementbreaksevenatI>3M; thisisan algebraicconditionalcostmodel, notacalibratedhardwareclaim. ActualinverseEuclideaniterationcounts varywithinputs, andmemory/allocation/bookkeepingcostsremainuncounted. No timingmeasuredoroldfailedwallqualificationrepaired.

Someg3/g4inversioncomponentratios exceed10, but basefieldREDC ratios andremainingworkdo not jointlyestablish10xcompletegain; g2remainswellshort. Classicalbatchinversionisnotnovelpriority. Publicsourceinfo/sharedsetup charged equallyacrosscontrols; onlyPencilhasthespecificevaluationdenominatorlist. OriginalscalarPencil retainedasexactstreamcontrol.

RawCONTROL/FIXTURES/OUTCOMES/RUN/EXIT plusOUTPUT.sha256verified;SUMMARY.tsv andSUMMARY.stderr arepostrunRustanalysisfromsummarize.rs. All774helperformularesultsretained inCONTROL.stderr. GitcustodywhileS3unavailable,no upload/no retry/nogaterelaxation. Broad10xgenus2+objective remainsunestablished.
