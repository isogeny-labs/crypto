#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Serif",size:10pt)
= Interrupted prime Lefschetz certificate run
Observations only; no mathematical verdict. Run n83-prime-lefschetz-20261030-r1 started 2026-10-30T16:51:42Z. The resumed command was stopped after native source auditing because the prepared certificate phase uses Python, contrary to the current repository native-execution requirement.

#image("pipeline.svg",width:100%)

*Recorded flow.* 42 native source audit records; one of four phases completed. Certificate production, source replay and certificate replay were not started (status125 each). Zero completed certificate cases out of42 planned. No degree claim was produced. The diagram shows phase status because no comparable degree measurements were collected.

*Termination correction.* Controller exit143 was observed. The original EXIT trap wrote exit0 after SIGTERM at16:51:51Z. That value does not establish success. The original terminal remains unchanged; evidence/prime-lefschetz-interrupted/interruption.json records the corrected overall status. This is an operational interruption, not a mathematical counterexample.

*Scope and identity.* Prime m3–9, original/baseline/spike40 arms, planted/natural targets;42 frozen sources. Source revision61299f87e53d4f4ad4ace01a51039d6234546bac. Protocol SHA256: fa97636060d9129a4137c9280e9fff119f81e7ce6fc915204c177854b971fc8a. Four120second phase caps,2GiB address space,720second outer deadline; no retries. Existing-host backend; wall_valid=false, no performance claim. Exact command and source are in the preserved raw archive. Prior controls are development evidence and do not complete this production run.

*Custody.* Raw archive is content addressed through prime-lefschetz-archives/.artifacts.json. Downloaded archive SHA256 and all restored payload hashes passed. The42 source audit rows, original terminal, correction and source archive identity are retained under evidence/prime-lefschetz-interrupted. Earlier datasets are preserved.

*Remaining work.* Replace certificate execution/replay with native code, correct signal handling, freeze a new protocol and admit the execution backend before a fresh attempt. No change to prior published FFD bounds, LFD or regularity. The full Koblitz/binary/prime m2–9 fixed-base objective remains open. Independent review/formalization of the written argument remains pending.
