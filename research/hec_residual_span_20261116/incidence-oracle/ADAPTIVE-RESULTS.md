# Free adaptive-anchor/core oracle: exact visit-model ceiling below10

Post-analysis frozen at965d0513a49 reads immutable ROOTS.tsv; no producerrerun or newdata. All3264anchor/orbit blocks are retained, including zero-rowanchors. Coveragecounts deduplicate multiple roots in the same orbit within each row. All6workloads and16anchors are checked. [pcc:unverified]

|p|source|exact fixed-core oracle row-rate gain|free anchor/core oracle row-rate ceiling|
|---|---:|---:|---:|
|101|1|3.235849057|6.339622642|
|101|17|3.062500000|6.000000000|
|251|1|3.450592885|5.691699605|
|251|17|3.348000000|5.184000000|
|601|1|3.596806387|5.429141717|
|601|17|3.310613437|5.296981500|

Reference density is allMretainedrows dividedby thepointvisits ofALL16fullanchors, not the completing solver's firstdeterminingprefix. Eachanchor/orbitblock has5canonicalpointvisits; bothsigns haveequalcommoncostfactor. If a choice processes blocks B, its emittedrowcount is atmostsum_B coverage. Therefore its rowcount per visit is atmost the largest individualblockcoverage/5. Dividing by the reference density M/(16*5*r) gives16*r*maxcoverage/M. This bound applies even to cores changing byanchor and free skipping. The largestblock attains this idealizedrowdensity, so it is not just a loose rootincidence count. A multi-block union can lose overlap, never exceed the maximumblockdensity.

For a fixedcore acrossALLanchors, aggregatecoverage gives the analogous exactoptimalratio r*max_orbitcoverage/M; any larger core's unioncoverage per orbit cannot exceed its bestsingleorbit. This acceptedrow-rateobjective is different from optimizing independenttargetrank. Choosing a single highcoverageblock does not establish target determination; setup,learningselection,failedcandidatefactoring,weightedLPelimination andoriginalsourceverification are absent from this optimisticvisitmodel. No actualalgorithm receives freeperfectknowledge.

Decision: onthese6fixed16anchorpanels, even oracleanchor/coreselection cannot produce10xacceptedfullrows perpointvisit versus thatdeclaredfullpanelreference. This doesNOTbound allpossiblealgorithms, perpointarithmeticsharing, newanchorlaws, otherfields/curves, completingruntimeor matrix/rankimprovements. It doesnotclaimthepracticalceilingsareattainable. Nevertheless it removesfreeadaptivecorechoiceas anunquantifiedescape fromthefixedcorebound onthese data. A newcandidate mustchangeanothermechanism orprovide explicitnew-panel evidence.

ADAPTIVE-BLOCKS.tsv,ADAPTIVE-SUMMARY.tsv,source/proof/input/binaryhashes andstderrretainallpostanalysis. Originalrawrecords/failures/gatesunchanged. No scientificretry,paidresources,S3polling,noveltyclaimor mathematical10xobjectivecompletion. Goalactive.
