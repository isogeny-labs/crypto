#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Split quartic proof remains resource limited")

#text("Gate quartic-lean-v4 checks the same universal-ring quartic discriminant factorization and its nonrepetition consequence, separately from the coordinate theorems. Exact source and command are preserved in its metadata.")


#image("../../figures/joint-mechanism.png", width: 100%)

#text("The compiler still reports std::bad_alloc under 8 GiB. The gate returns FAIL_build; neither theorem receives a passing Lean or axiom-audit label. The retained build log (../../results/proof-checks/quartic-v4/build.log) records the actual failure. This is a resource outcome, not evidence that the polynomial identity is false.")


#text("The separate integer identity has exact zero remainder and a rejected mutation: IDC1h4254a7cf15bd8c71. It proves the integer polynomial factorization, with its own certificate scope. A feasible checked Lean proof remains an open obligation in the proof map (../../proof/PROOF_MAP.md).")


#text("One worker, 8 GiB, a 900 s watchdog, pinned Lean 4.34.1 and the busy reservation are retained. No performance comparison is admitted; the diagram shows why native certificates and formal proof receipts differ.")
