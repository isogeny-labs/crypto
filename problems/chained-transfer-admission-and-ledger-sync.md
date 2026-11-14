# Chained transfer measurement admission and append-only ledger synchronization

- **Status:** open
- **Kind:** access/tool limit, measurement gap, and PR synchronization gap
- **Affected work:** [PR #3877](https://github.com/isogeny-labs/crypto/pull/3877),
  [queue #3904](https://github.com/isogeny-labs/crypto/issues/3904),
  [consumer PR #1484](https://github.com/isogeny-labs/cryptanalysis/pull/1484),
  [CT05–CT07](../research/chained_jacobian_transfers_20261110/QUEUE.md)
- **Next action owner:** continuing research session; ledger policy reconciliation requires the repository's accepted procedure
- **Last updated:** 2026-11-12

## Problem

The fixed F101 native transfer can be developed and checked locally, but a
controlled solving-cost comparison is not admitted without S3 checkpoint
transport and live IsoLab budget/capacity visibility; the legacy PR branch
also has a PCC ledger fork relative to current main. [pcc:unverified]

## Evidence and uncertainty

- **Observed:** `scripts/checkpoint.sh gate` reports no AWS profile,
  `CHECKPOINT_UPLOAD_TOKEN` or token file. No new experiment/benchmark launched.
  The native tests and Lean development builds are distinct local development
  work. `crypto-autoresearcher` is vault locked without its KEY binding;
  research-KB MCP/CLI and ecdb credentials are unavailable. [pcc:unverified]
- **Observed:** `pcc ledger verify --base origin/main` fails. At the recorded
  comparison there are 569 common rows, 584 base rows and 573 head rows;
  the first new claims on the two sides differ at row 569. No entries were
  rewritten. A Copilot base synchronization request was posted to #3877,
  explicitly prohibiting picking one ledger history or rewriting hash links.
  Additional local claims after this comparison are retained in the branch.
  [pcc:unverified]
- **Observed:** the global bench-record gate names missing historical raw
  records after artifact manifests and referenced benchmark entry directories
  were materialized. No new benchmark record or leaderboard row is introduced
  by this work. Historical #3877 ecbench audit failures identify unrelated
  archived records as well. The exact global checks remain unsatisfied.
  [pcc:unverified]
- **Derived or proved:** none of these access or synchronization failures is
  a mathematical obstruction to a DLP transfer. The fixture has a genuine
  Richelot hop, a nonzero endpoint, zero projection controls and a complete
  finite-fixture shortcut comparison; all remain producer assertions until
  their actual PCC standing permits more. [pcc:unverified]
- **Missing capability:** admitted external experiment transport/accounting,
  vault access for lead-atlas writes, an accepted append-only ledger
  reconciliation, and signed independent verification.
- **Observed full-library failure:** completed root attempt04 has6184 pass,
  13 fail,133 ignored (PCC-20261111-8ffeb3). Restoring exact tracked fixture
  blobs fixes6 failing tests. Five still require an untracked archived
  targetwalk records.jsonl; two require absent container time-namespace
  metadata. Raw failure and targeted reruns are retained in the study evidence.
  The full run remains failed; no raw archive or kernel metadata was fabricated.
  [pcc:unverified]
- **Observed sparse-workspace PCC gap:** the broad local verify-all03
  partial snapshot/PCC94583f has304 records with187 missing-subject/project
  failures,109not_covered and8localpass. Every failed proof reason in that
  prefix is a missing historical file/project;examples are tracked in1e4
  but omitted by the current sparse profile. The process remains running,
  so these are partial totals. The new odd-descent theorem statements have
  separately frozen local pass verdicts in b33222. Neither a scoped pass
  nor this diagnostic supplies signed standing or a global PCC pass.
  Retain the final archive before deciding any materialization/recheck;
  no ledger/claim/subject rewrite or unchanged repeated audit.
  [pcc:unverified]
- **Final local PCC observation:** full03/PCCf117f7 completes371records,
  actual exit1:187fail/155not_covered/29localpass. All342 failed proof
  reasons are missing historical subjects/projects in the sparse workspace.
  The exact project-subject subset97 has68not_covered/29localpass/zero fail.
  Whole raw245413bytes→standard-xz20212bytes/hash/exact roundtrip retained;
  partial94583f is unchanged. Later three records are explicitly excluded;
  RootTable1da835 has its own local pass. No blanket/signed PCC pass or
  frozen-history rewrite. Do not rerun unchanged until the missing proof
  subjects are materially available. [pcc:unverified]
- **What this does not establish:** a speed advantage, its absence for all
  algorithms, security, or nonexistence of cross-characteristic computation.

## What would unblock it

- [ ] A successful checkpoint gate using configured credentials/token, with
  actual S3 readback where required by the repository contract.
- [ ] Live remaining daily/monthly IsoLab allowance and pool/quota visibility,
  then a finite frozen request under the EKS skill; no quota or identity changes.
- [ ] A ledger synchronization that retains both histories under an explicitly
  accepted repository procedure and passes the base-prefix/hash-link check.
- [ ] Final-head applicable CI/review gates and signed PCC standings; a local
  self-check or unsignatured verdict does not discharge these.

These conditions admit a comparison and PR integration. They do not themselves
establish an algorithmic gain. The comparison must include the direct elliptic
shortcut τ φ [2] discovered on all 102 source points. [pcc:unverified]

## Next discriminating step

- **Action:** reconcile the pending base synchronization request, preserve the
  current source commits, and recheck access before CT05. Continue CT09 native
  generalization/proof work if these external gates remain blocked.
- **Inputs and controls:** exact fixed source/target equations and subgroup in
  the report; direct elliptic, full Jacobian and collapsed-route arms; preserve
  historical Python and all failed receipts.
- **Limits:** no paid build/job until the existing budget and checkpoint gates
  admit it. No ledger rewrite, force push, protection bypass or plaintext vault
  write. A pending check is not a pass.
- **Outcomes:** admitted access permits frozen benchmark execution; denied or
  unknown access retains CT05 as blocked while other authorized work continues.

## Evidence links

- [Current report and proof map](../research/chained_jacobian_transfers_20261110/REPORT_20261111.md)
- [Source review receipt](../research/chained_jacobian_transfers_20261110/SOURCES_20261111.md)
- [Original broader prime-cover backend gap](prime-cover-balanced-jacobian-and-induced-maps-missing.md)

## 2026-11-11 consumer fork after focused Phi success

The consumer now has its own instance of this same append-only synchronization
obstruction. Exact Phi head2450815e2a3b84687b405ef8498dae9dc2663fe1 has60
ledger rows. Mainfb3f88a03d54bd81a8e83a1281633a54522c8286 has61 after
PR1485/1486 appended five checkpoint-bypass entries;the histories share only
56 rows. Current native continuation had65 rows at the captured comparison,
and its original chain remains intact. Prefix validation reports row56
differs;GitHub PR1484 reports mergeable_state dirty. No entry,hash link or
history was changed to make it pass. [pcc:unverified]

ConsumerPCC9c0b68 pins exact ledger digests/counts,failedprefix output,current
checks/review metadata and full compressed CI logs. Focused43085588914 on
exact2450815e2 succeeded25native/57reference unit+two integrations,one old
ignored benchmark. Requiredstore/gate/records and allCodeQL complete,zero
reviews/threads at the observation. Separate cppcheck/clang-tidy jobs have
actual findings in unchanged C files;they are not package-DNS failures or
clean-code verdicts. Existing consumerproblem static-analysis-ubuntu26-tool-drift.md
tracks the analyzer lane;these newer diagnostics are a new observation,not
proof that all historical causes are identical. [pcc:unverified]

Exit remains an accepted reconciliation retaining BOTH frozen histories and
passing actual base-prefix/hash-link requirements,plus final-head applicable
CI/review. Do not rebase,force,choose a lineage,rechain archived records,
drop claims or weaken the gate. Focused arithmetic success does not resolve
this integration requirement. Continue unblocked CT21 quotient/proof/source
work while the accepted reconciliation is unavailable. Root6905c86363
preserves the geometric stage;no ledger bypass or benchmark is authorized
by that checkpoint. [pcc:unverified]

## 2026-11-12 cycle/catalog checkpoint

Actualroot33/70a16a,writtenf48205(+V2annotationb7c7e7),algebra8d3d02 and
nineboundLean establish the scopedcycle/catalogcomponent. Consumerready1509
exact8f456314 retainsold98 andaddsindependent0796source replay. These do not
resolve livebudget/S3 or frozenledger/C/signing gates. Broadbenchrec still
reports189missingraw/archive custody problems in106unchangedrecords;full
wholelogs retained by cycle-governance-01. No benchmarkrecord modified or
upload/bypass performed. Acceptance remainsactualadmittedcosts andaccepted
ledgerintegration/current-headCI/review. Next technicalworkisactualunbalanced
models/maps andwholeexception/rationalreplay,withallcostrequirementsopen.
[pcc:unverified]

## 2026-11-13T22:24:21.857355+00:00 — authentic original fixture restored; original library regression retry

Actual1ea795:prospective708cb8/2aeebefore dataonly3baaworkflow;43516010720/job132198018704SUCCESS/original3dtool/config/manifest/pull0verify0;WHOLEartifact14081195285/150168bytesSHA4df508a9/all16members bytechecked. Exactoriginal2293097bytefixtureSHA8970667e restored inignoredoriginalpath/originalverify0/Gitstatusunchanged. SAMEcachedoriginalbinarySHA82029f63/exactformerfailedtestnow1pass/6329filtered/all2137guards0. Firstpreparation01stoppedbefore nativeon2historicalconsumerworkflowhashes;all2131otherinputs unchanged;37e18b keepsbothcurrent+historicalWFguards,no source/WFedit. [pcc:unverified]

Rootoriginalunfilteredretry03 is actuallyrunning at /workspace/scratch/ct68-root-library-regression-03:UNCHANGEDcargo test --release --lib --offline --locked/SAME1800sbound/originalwarmcache/2137guards,protocol7433ce. Ordinaryunitexception explicitlydocumented inAGENTS/checkpointhook;originalPCCappend-bypass0 andoriginalcheckpointpush1(noprofile/token) retained. This is notbenchmarkadmission;globalrootstillnotgreen beforeterminal. Closedderivedcompilerct65/ct67caches deletedonlyafter0/0/fullrawretained;ct55originalbinary andct57warmcache kept. [pcc:unverified]

Consumerrefinementfb9e fullb38c7f703/own12CI65dd08108native55canonical settled. ActualnewtargetICe869bd/all60attempts/all15publiccases/2543guards/82wholeJSON;allreportedanswers actualtarget+independentoriginalsourcechecked. Codecurrentlyuncommittednew-target-ic;nextmathematicalsoundness/faithfulLean/report/strictCIpublication. Fullcosts/strongestalgorithmreference/performanceadmission/completebetacoordinates/actualgeometry/mainmerge/vault/KB gapsremain. [pcc:unverified]

## 2026-11-13T23:09:58.433906+00:00 — whole new-target IC data handoff and fresh guard07

ActualnewICe869bd/all2543guards/ALL60attempts/all15publiccases/82wholeJSON;written1e65f0/twofaithfulbindings01efbf671d42/bothoriginalgates2/57total;TWO69377-byte finalPDFpages ALLinspected d1027f. Current729CI/730outer/9phasepairs/14workflow190native57canonical prepared,not yet published or replayed. [pcc:unverified]

TWELVEwholeclosedcaptures+TWELVEcompleteindices+ONEwholefinalPDF25objects require 13genuineS3migrations;other12 wholeobjectsordinaryGit. Freshcurrentguard07 holds 3455 scientific/code/proof/report/canonicalPDF+link/current729CI/latest703full/root2137 input bytes until exactgenuinebotFF/finalguards. Guard06historical3241/released/NOT reused. FinalPDFexplicitwholebyte .pdf.bin carrier appliesoriginalstricterdata policy;canonicaldocsPDF+researchsymlink preserveunchangedPCCdocs hydration/pins,negativecontrolsmissing/substitutionfail. No split/prune/threshold/policy/gate/verifier/trustchange;remoteavailabilitypending. [pcc:unverified]

Rootfull03 actuallyterminal124/1800s/all2137guards0/two visible diagnostic-reap failures;fulltotalsunknown. Wholecapture/log retained;next exact original failurediagnostics afterthisdataFF,not unchangedfullretry. OriginalS3checkpoint/livebudget/capacity/countercoverage/reference admission andfullgeometry/mainledger/signing/reviews/conflict/vault/KB gapsremain. Nextmateriallydifferent checkpoint route:originaluploader inCIcredentials withoutkeyexport/gateedit. Continue aftereachrung. [pcc:unverified]

## 2026-11-13T23:22:36.238211+00:00 — original diagnostic-reap failures located

### What was done (commands, files, features)
pcc/claims/PCC-20261113-633ab0.json:SAMEoriginalbinary exacttwo failures EACH101/onefailed6329filtered/all2137guards0. BOTHpanic at originalDiagnosticSessionClock::begin().unwrap beforepublicationassertions. Originalrequiredbootidread0;readonlynamespace/time-offset reads1/missingfiles. Wholelogs/backtraces/exits/sourceexcerpt/environmentreads retained. [pcc:unverified]

### Decisions made (and why)
Thiscontainer lacks mandatory/proc/self/ns/time and/proc/self/timens_offsets. Preserveoriginalclock/securitychecks/tests;futureoriginalglobalregression requirescapableLinuxhost. No unchangedglobalretry orsource/privilege/namespacechange. [pcc:unverified]

### Open threads (unfinished, pick up next time)
Rootfull03 remains124/unknowntotals;publicationinvariants notexecuted bytheseexactfailures. Actualdata43525890255/13replays/e563da1b1/exact15paths/3455guards hold07released43d6ba/genuineconsumerbabda6. NextconsumerICsourcecheckpoint/730full/14CI;originalS3uploaderCIroute/costcoverage/commonoriginbridge. [pcc:unverified]

### Errors (what went wrong, how it was handled)
Exact101failures preserved;originalS3checkpointpush1/KBconductor127 unchanged. No globalpass/benchmarkadmission ormergewaiver. [pcc:unverified]

## 2026-11-14T00:45:02.459782+00:00 — common-origin whole-data handoff and fresh guard08

### What was done (commands, files, features)
Native8a77f3/all3304guards/20pairs40reports51JSON/15correcteach/20outcomesmatch/sharedFB30failures;fouractualP0/fullFB/RNG/1004raw92triples/1004wrongorigincontrols. Written8ffc9c reusesEXISTINGfaithful01efbf671d42/57partialbindings. Report6271ce TWO79399-byte finalpagesALLinspected;new754CI/self755/ninephasepairs/16workflows241native57canonical prepared,ownsourcefullCIpending. [pcc:unverified]

SEVENwholeclosedcaptures+SEVENcompleteindices+wholePDF15objects require8genuineS3migrations,7smallwholeGitobjects. Includeswhole94file unpublishedsource/report/754CI/PCCledger/resume/base0a+patch snapshot torestorebeforeconsumercommit. Freshguard08 holds4230 currentinputbytes andPDFsymlinktypes/targets untilactualgenuinebotFF/finalguards. Old07historical/released,ONLYpostpublicationcanonicalREADMEdelta mapped/alloldscienceunchanged. Originalroot2137/compiledbinary/fixture kept. [pcc:unverified]

### Decisions made (and why)
WholePDFcanonicaldocs+transparentrelativealias preservesunchangedPCCdocs hydration;explicitroot.pdf.bin carrier appliesstrictoriginaldatapolicy. No split/prune/policythreshold/verifier/trust changes orfakeavailability. WholecontrolplaneAPI/logs/losslessXZ retaineddirectafterpublication. [pcc:unverified]

### Open threads (unfinished, pick up next time)
Normalrootpayloadcommit/push/originalpublisher8remote byte replays/exactFF/all4230guards+BOTHereposdirtybytes+linktypes/genuineconsumerprojections;thennormalconsumersource/755full/fouractualadmission/16CI241native57digests. Nextcompleteoperationcoverage/backendinventory/odddegreecoordinates/admittedrefs. Continueaftereachrung. [pcc:unverified]

### Errors (what went wrong, how it was handled)
Originaltransportffe675 succeedsremote+localORIGINALgate forexactclean39e9;laterstatechangesstaleit. Rootfull124/unknowntotals/twoexact101clockinit failures633ab0 remain;CIhostmetadata0iscapabilityonly. Fullgeometry/logextension/fullcost/livebudgetcapacityrefs/mainledger/signing/reviews/conflict/vault/KB gapsremain. [pcc:unverified]

## 2026-11-14T02:17:18.564338+00:00 — norm-assisted whole-data handoff and fresh guard09

### What was done (commands, files, features)
Native9bc5e6/all4082guards/225scalarpairs1125triples60ICpairs120reports/all15casesTHREEcommoncorrectsettings/45correcteach/all15FB30failures retained. Writtene7a860/faithful789d94a9a7afed533b/60partialunique/bothoriginal3gates0trusted/report510c8c TWO75055-bytefinalpagesALLviewed. StrictCIa2feab784+self785/tenphases/86JSON/17workflows327native60unique62actualchecks prepared/pendingnewsourcefullCI. [pcc:unverified]

pcc/claims/PCC-20261114-d81ee9.json:SEVENwholecaptures+SEVENcompleteindices+wholePDF15objects/8genuineS3pending+7smallGit;ALL115ownedpendingfiles/baseb57/patch/784CI/PCCledger/resume/report+alias snapshot119members. Transport84filenonsecretclosureALL28ZIPmembers/ONEprivate temporarydownloadAPIexcludedwithhash/fulloriginalprivatecapturelocal. Freshguard09 HOLDS5030 actualinputs andallreportlinktypes/targets. Old08historicalreleased/ONLYpostpublicationcommoncanonicalREADMEdelta mapped. [pcc:unverified]

### Decisions made (and why)
Useunchangedgenuinepublisher/wholeexplicitPDF.bin canonicaldocs+relativelink/no splitprune thresholdpolicy orverifiertrust changes. WholecontrolplaneAPI/loglosslessXZ recordedDIRECTafterpublication. [pcc:unverified]

### Open threads (unfinished, pick up next time)
Normalrootpayloadcommit/push/actual8remote-byte replays/exactFF/all5030guards+BOTHereposdirtybytes+linktypes/genuineconsumerprojections;thenconsumersourcecommit/785full/fouractualcontrols/17CI327native60unique62actualchecks. Continuecompletearithmetictaps/coldonlinecoverage/admittedmatchedrefs/odddegreecoordinates. [pcc:unverified]

### Errors (what went wrong, how it was handled)
AllsharedFB30failures/bindingcaller01/Typstinitialcompile/reportcustodycaller failuresretained. Originaltransportb08916passesexactclean39b/laterchangesstale. Root124/unknowntotals/twoexactclock101 errors633ab0 unchanged/capablehostrequired. Geometry/logextension/cost/mainledger/signing/reviews/conflict/vault/KB gapsremain. [pcc:unverified]
