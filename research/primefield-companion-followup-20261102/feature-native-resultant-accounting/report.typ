#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Reversible resultant accounting
Three native polynomial backends share an explicit scalar-return interval. Source substitutions are recorded and reversed byte-for-byte. Counters cover field arithmetic, inversions, integer EEA steps, projection, helper calls and observed allocator activity.
#image("flow.svg", width: 100%)
== Software checks
The self-test verifies handwritten p13 arithmetic counts, direct paired allocator callbacks, repeatability, empty/unsupported inputs, failed allocation bookkeeping, nested spans and overflow flags. See validation-01/receipt.json for pinned compiler, binary, source and logs. Producer software checks; no independent review. No comparison panel was executed.
== Interpretation
Counters are separate dimensions, not a weighted total. Helper calls overlap their internal arithmetic. Requested live bytes exclude allocator overhead, RSS and transient realloc copying. Allocation calls may be removed or merged by compiler optimization. Single-thread execution and lifetime discipline are required.
== Construction remaining
COMPARISON-DRAFT.md proposes48 matched polynomial cases. The generator, positive-control entry point, oracle, complete-panel exporter/analyzer, canonical frozen protocol and budget/capacity admission are still required. No funds are reserved and no cloud work is authorized by this draft.
== Sources and retained failures
The preserved scalar, Euclidean and half-GCD sources; prior reversible instrumentation; Rust GlobalAlloc documentation. KB unavailable. Initial generation rejected an unlisted conditional helper; its name table was completed. Two deprecation warnings are retained. The original4563-case cohort is unchanged.
