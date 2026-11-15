# Root validation v2 — caller-owned systemd log files

Preserve prior failed1800-second attempt and all native receipts. It failed beforecompiler: systemd created root-owned0600unit.stderr, launcher flush_logs raised PermissionError before durable parsedownership, worker permitexpired at10seconds with commands[]. Parent joined rawcapturedquery/worker/currentunit sameInvocationID and absentcgroup before stopping. Newmanualattempt is not an automaticretry. [pcc:unverified]

## Minimal source correction

Only durable-launcher.py changes. precreate_unit_logs exclusively creates unit.stdout/unit.stderr as caller via O_EXCL|O_NOFOLLOW, fchmod0600, checks currentEUID/EGID/mode and fsyncs eachfile and directory before launch. Existing files are rejected, never truncated or chowned. Existing flush_logs/sealing/resources stayunchanged.

call can defer checkpoint. show defers checkpoint until parsing and storing first observed InvocationID/cgroup, with exactidentity consistency checks before overwriting. Thus checkpointfailure retains parsedownership for finallycleanup. Launchclient query recording also defers checkpoint to initialownership observation. No foreignunit kill is added; absentidentity remainsunresolved. Existing heartbeat can still fail before initialquery; retainnoownership/no-stop behavior in that case.

## Prospective bounds and qualification

Reviewer prepares independent<=60-second nativeunitfixture: callerprecreated0600logs, actualsystemdappend, callerflush/fsync/hash, sameInvocationID/cgroupterminalcleanup, positive sentinel and existingfile rejection. Publication/review/rootrelease precede fixture. No compiler in fixture. Receipt must distinguish actualnativecheck from prior26syntheticcontrols. root-log-qualification-20261109 is intended independentpacket.

After fixturepass and review, publish finalizedsource/config/receiptbindings and release one1800-second manualrootvalidation: unchanged2CPU/16GiB/swap0/tasks512, offline locked defaultfeature release compile, four exactorderedcontrols, fullunfilteredlibrarysuite, same2026-11-10T05:11:39Zdeadline. Preserveallresource/disk/sealingchecks. Currentcharged2880/remaining40320; rootchargesfixture and reservesattempt prospectively. No newdeadline/allowance.

Reuse materialized-inputs.json SHA c5178d2138d79f1ea8cac1fc96cb8d968913ba403d68ee66fa36afdd60fdf3f2, unchanged /var/tmp/pencil-root-materialize-t1953-20261109 workspace. No source regeneration. Newunit/output/control names useT1970. admission-draftfalse and noadmission.json blocklaunch untilfinalization. Oldtestcandidate, helper/worker, dollarwaiver and exactcontrols remain unchanged.

Sourceschecked: exactfailednative root-terminal.json and currentlauncher; priorindependentwrapperreview and inheritedOPERATE. KBunavailable; no noveltyorperformanceclaim. No payload/tests/compile duringpreparation.

## Serial accounting and artifact update

Root selected normreplay2400 before nativefixture60; draft now projects cumulative5340/remaining37860 before v2root1800reservation,36060unreserved. These are conditional until normterminal and qualification receipts arrive. Admission staysfalse. Materializedmanifest is recorded in .artifacts.json and ignored byGit; storepush reusedexistingobject, pull reusedlocalfile, verify--remote reportedzero problems. Exactc5178bytes retained. No source/fixture change or payloadexecution by thisagent. [pcc:unverified]

## Final binding amendment

Native logfixture passed: exactsourcehelper/systemdappend yieldedUID1000mode0600, callerflush/hash, injectedcheckpointownership retained, sameinvocation cleanup; result/exit/review/sourcepublication copied and bound. Normcompletion and2400exitdebit copied. Actualcumulative5340, remaining37860, manual1800reservation leaves36060. T1970reclaimedattempt2epoch3 explicit. Admission nowtrue conditional on rootfinalbindingreview/publicationreadback/livefence/resources; exactsource unchanged. Earlier drafts/conditionalbudgetnotes are historical and superseded by admission.json/launch.json. No launchby preparationagent. [pcc:unverified]
