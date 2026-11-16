#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Controls successor: recorded native result
[pcc:unverified] T-2110; retained November11,2026 attempt. No compiler, tests or solver rerun for this report. Independent machine audit remains separate.
#image("phases.svg",width:100%)
== Recorded checks
The compiler sequence consists of BranchRootMultiplicity, BranchCoefficientMultiplicity, IndependentControls and Readback exiting0, followed by ExpectedFailure exiting1 as required. Readback contains13named obligations, each reporting only propext, Classical.choice and Quot.sound. The negative control reports one unsolved goal False. Native completion and parser report pass.
Pre, compiler and post phases all passed and confirmed terminal-owner custody. Dependency, retained-import and runtime manifests are byte-identical before and after. Native output and raw receipts are retained in full; this is the producer archive, not an independent proof review.
== Durations and accounting
Recorded outer worker-wall duration287.1954594349954seconds. Phase durations:55.443576349993236pre,185.60343303100672compiler and46.03922345099272post. These are one-attempt worker-wall measurements, not CPU time or a controlled performance comparison.
Full600second reservation charged. Renewed campaign now has1200charged and128400remaining. The preceding controls failure is retained separately in branch-cast-renewal-results-20261111 and remains charged. No cohort targets or cost ratio are supplied by this proof qualification; full1260target scope is unchanged.
== Provenance
Source201bf9f1c17224abb9e1255edf5fddb37084697f; freezed20691095b0e6f932a5f34f9107e09c56629b57da85ea94dea35c007d87f3a40; releasebcbe1b0b3a1da832b45e3d1fed9dabfc6c8d8fe2afdc8c60b4be9fb1c054bdfe. result.json and phases.csv retain exact quantities; phases.svg is editable. raw-files.json and archive.json bind full native/outer evidence. Small receipts and Readback/ExpectedFailure stdout are copied under receipts/. Archive remote status is recorded independently.
