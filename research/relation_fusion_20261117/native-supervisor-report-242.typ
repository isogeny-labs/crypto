#set page(paper: "a4", margin: 1.45cm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.45em, spacing: 0.8em)
#heading(level: 1, "Native supervisor: functional qualification")
The opt-in Linux supervisor passes five OS tests and 216 functional proof cases. Each of 108 wrapped/native pairs has identical complete proof bytes, constructor metadata, independently checked bases and replay statistics. Six transport controls preserve expected failures, later arms and proof-directory ownership. No parent-performance statistic or speedup admission is produced. [pcc:unverified]
#parbreak()
#image("visual-242/supervisor-path.svg", width: 100%)
#parbreak()
Native supervision removes three utility execs and retains a fresh worker. CPU0 is required; child hard limits remain 2 GiB address-space and 16 MiB per file, with an absolute 30 second deadline. Private-group cleanup and pidfd/poll supervision refuse unsupported signal handling. A parent-death signal with a parent-PID race check closes a source-review gap in the first prototype.

Scope: fixed normal unprivileged single-process workers, with no credential changes or guard disabling. This is a research supervisor, not a general process sandbox. The native timeout kills the private group at 30 seconds; the old wrapper keeps its TERM at 30 / KILL at 31 seconds behavior. Failed/censored arms never count as wins.
#table(columns: (1.3fr, 0.55fr, 2fr), inset: 5pt,
[*Check*], [*Result*], [*Scope*],
[OS 229, first source], [4 pass], [Real limits/refusals, deadline/descendant cleanup, SIGCHLD refusal],
[OS 238, refined source], [5 pass], [Adds abrupt-supervisor-death worker termination],
[Functional 240], [216 cases], [108 native + 108 wrapped; 6 transport controls],
[Saved audit 241], [216 proofs], [108 launcher identities; 6 transport controls; 3 rejected sabotage classes],
[Repeated timing], [Unrun], [Original 15× criterion stays open],
)
Flow 240: planned 216, attempted 216, completed 216, excluded 0. Both primes 2147483647 / 9007199254740881, n3/4/6, seed 7 LAST-star; 18 fixed control/candidate arms. Complete original-ideal membership, input containment, reduced monicity and strict-standard/checked-product S-pair obligations remain. No F4 solve; observed solving degree stays null. [pcc:unverified]

Source/tools 239 were published at 2647188b4a before final 240 calls. Earlier 229/234/237 contexts remain unchanged and do not establish parent-death protection. Build 230 failed on workspace membership; that failure and every subsequent build/log remain. Artifact audits do not independently re-execute OS limits or attest execution. All evidence is unsigned producer data. [pcc:unverified]

#link("https://github.com/isogeny-labs/crypto/pull/4989")[Continuation PR #4989] follows merged #4530. Evidence: NATIVE_SUPERVISOR_QUALIFICATION_238.json, SUPERVISOR_FINAL_OUTCOME_240.json, SUPERVISOR_FINAL_AUDIT_241.json and immutable source/tool pins 239.
#pagebreak()
#heading(level: 1, "Parent trace 216: where the call spends time")
The prospectively published trace completes 324 calls: three rounds × six inputs × 18 arms. Independent arithmetic replays all 324 whole proofs; raw/summary/input/basis/stat bindings and exclusive parent partitions agree. Archive 225 contains 1984 indexed files plus index; extraction 226 reproduces audit 223 byte for byte. [pcc:unverified]
#parbreak()
#image("visual-242/parent-phases.svg", width: 100%)
The graph shows only the fixed primary packed-dag-487: 18 calls, three per case. All 18 arms remain in PARENT_TRACE_ANALYSIS_223.json. Dots are all three recorded totals, not confidence intervals. Stacks are recorded exclusive-phase means. The wait phase includes the worker computation and process lifecycle; it is not all wrapper overhead.
#table(columns: (1fr, 1fr, 1.6fr, 1.2fr), inset: 4pt,
[*Field / n*], [*Mean parent ms*], [*Median process minus worker ms*], [*Mean controller replay ms*],
[p31 / 3], [4.912], [3.110], [0.202],
[p31 / 4], [4.357], [2.870], [0.199],
[p31 / 6], [5.567], [2.933], [0.341],
[p53 / 3], [4.478], [3.005], [0.228],
[p53 / 4], [5.177], [3.500], [0.215],
[p53 / 6], [5.585], [3.139], [0.367],
)
These scopes and summary statistics differ and establish no independent end-to-end gain. Trace instrumentation and every outlier remain. Zero monitor-flagged CPU-contention samples is not an independent isolated-host attestation. Fewer utility execs are a testable implementation hypothesis, not a predicted ratio. [pcc:unverified]

The latest completed repeated comparison 200/211 has 1260 calls; primary median ratios 1.0027–1.1244 remain inside measured A/A spread. No improvement or 15× admission. Any new comparison must upgrade strongest matched controls with the same generic launcher, retain the full agreed boundary, repeats, neutral controls and every raw outcome, and publish its source/protocol/pins before data. [pcc:unverified]

*Open dependency:* canonical protocol/schema/checker files in crypto-autoresearcher are encrypted, and supported vault unlock reports no KEY binding. A settings request is pending; canonical protocol/export validation remains open. No proof, measurement or publication gate is waived. Historical full-root and signed-execution qualifications remain separate.

Sources: Linux man-pages pidfd_open, poll, getrlimit, PR_SET_PDEATHSIG and setsid; official Rust CommandExt docs; local Linux/glibc headers. KB/conductor unavailable; primary-source review recorded. LocalCPU0/one-thread/zero-cloud and checkpoint/S3 waiver persist. Canonical speed graphs are unchanged; no newly admitted performance fact.
