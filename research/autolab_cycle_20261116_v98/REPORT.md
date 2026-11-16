# C98: whole-chain integration loses to the separator join

Three immutable experiments passed. Their correctness checks succeed,but the performance/alignment hypotheses are refuted on the frozen tests. The C96conditional611.81xblock gain is preserved; it does not establish a whole-chain gain. The research program remains active and the full gate remains unpassed.

## Complete original-chain contract

The new controller enumerates every(x1,x2,u1)andevery(x5,u3),then invokes exactC96rebracketing for every(x3,x4,u2). It emits all ordered original8-coordinate tuples. The old separator join is retained as a strong comparator. The filtered controller additionally skips a middleblock only when BOTH finite rebracketing witnesses are certified empty,and zero/infinity branches are absent. Empty finite-w lists alone reject only the ordinary branch. Shared global variables and every intermediate are retained; capped blocks demote whole-chain completeness.

An independent literal-adjacency oracle enumerates original paths over the entire small field,without quadratic orrebracketing code. All28targets overGF4/8/16matchjoin,blockandfilteredcontrollers,with2,219completeoriginal roots. Zero,repeated,torsion,ghost andequal-endpoint cases are included. This is complete tiny full-chain evidence,not merely pair membership.

## Actual outer-image alignment

The actualGF(2^83)/D18profile usesT001target,4degeneratecontrols+124frozenseed981611source triples. Everyoriginalu1/u3branch is retained. The128source triples generate136endpointblocks and117finitew witnesses. None ofthe117iscoveredbythe62structuredcells. No block is rejected by a cell. Seventy-six emptyblockcuts come from the existing no-finite-w endpoint test,withzero/infinityexceptions checked. The unprunedC96blocksolver independently verifies every claimedemptyblock. No sampled block has a root; this sample doesNOTclaim globalUNSAT or fullsourcecoverage.

Thus the tested structuredfamily also fails to show useful alignment withthisactualouterimage. It should not be integrated as a defaultpruner. This doesnotruleout other families or target distributions.

## Full cold costs

Three repetitions,alternatingmethodorder,18completecoldchildren;everymethodincludesfield/familyconstruction,endpointdiscovery,allmiddlework,originalrootverification,dedupandcomplete serializedoutput. Complete rootsets agree:

| Field/domain,target | Originalroots | Separatorjoinmedian | Rebracketcontrollermedian | Cell-filteredmedian |
| --- | ---: | ---: | ---: | ---: |
| GF(2^7),d2,T3 |97|0.034994s|0.042284s|0.042817s|
| GF(2^11),d3,T17 |281|0.049728s|0.207974s|0.302470s|

On thelargercalibration,rebracketingis4.18timesslowerthanjoin,andfilteringis6.08timesslower. Allslowrunsarepreserved. Thefilteredcontrollerdoesfind83cellcuts and39no-wcuts among1,053initialendpointblocks,reducingblockcalls to931,butcertificate/setup/filtercostmorethanoffsetsthesavings. This isdirectevidenceagainstdefaultintegration.

The top-levelquadratic-call counter excludes calls INSIDEtheC96blockoracle. It mustnotbecomparedasatotalarithmeticcount againstthejoin. Repeatedfield/matrixsetupandblockrestorationcostsarechargedincoldtime. Sharedpreparationormemoizationcouldbe afreshhypothesis,butneitherchangesO(|D|^3)outer enumeration,andcachehitratecannotbe assumedatGF83.

ColdchildRSS12,800–13,312KiB. Outerworkerwalls0.298946/2.051053/2.154609s,andhigh-waterRSS153,864/153,872/156,212KiB. Theseareindividualprocess/inheritedhigh-waterrecords,notaggregateRSSorqualifiedallocationbenchmarks. Everyworkerhad90s,4GiB,onecore;nonearequalifiedhostisolation/A/A.Allcasesaresubsecondandfailthehardbaselinefloor. TheyarenotgenuineF4/F5/F6comparisons.

## Decision and next hypothesis

RetainrebracketingasanexactCONDITIONALoraclewhereotherworkalreadyfixesendpoints. Rejectthecurrentk^3outercontrollerandboundedcellfamilyasdefaultglobaloptimizations. Returntheglobalresearchfocus toimplicitresidue-state/supportsearch ratherthanflatendpointorwholefieldcellenumeration.

NEXT_RESIDUE_STATE_HYPOTHESIS.md statesa fresh exactaffine-stateMDDhypothesisforcommonliftbitlocaltables:canonicalremainingproblemstatesmaymergebeforeenumeratingthe reported2^44necessaryassignments. Firstrecoverandhash-checktheprecisepredecessorartifacts;missingdatamustnotbereconstructedfromsummarycounts. Thepersistent20261115_v18report inspectedhereisadifferentearlierE2compilercomponent,anditslimitsremainpreserved. Tinyshared-qexactoracles,state/liftinvariantsandaboundedactualprefixmustprecedeanyhardretry. Fullmatchinggenuinebaselineengineeringremainsanindependentactiveworkstream.

Allpriorresultsandsourcesareunchanged. Theoriginal339-variablehardinstance,noveltyclearance,20preregisteredhardcompletedgenuineF4/F5/F6matches,requestedprime/odd-extension/binary/Koblitzcoverage,qualifiedisolation/A/Aand50xend-to-endgoalremainopen. Nooverallfailureorimpossibilityisclaimed.

Replay:PYTHONDONTWRITEBYTECODE=1 python research/autolab_cycle_20261116_v98/REPLAY.py
