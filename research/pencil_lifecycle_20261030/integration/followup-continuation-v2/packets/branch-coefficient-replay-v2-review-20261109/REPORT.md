# Branch coefficient v2 source repair review

The four replacements address the four retained unsolved proof tails without changing the theorem statement or hypotheses. Each `change` exposes the local valuation expression; `rw` uses the already proved order equality; `norm_num` discharges the resulting finite ENat constant property. No compiler was run by this reviewer.

- Only BranchConjugateOrders changed among 18 Lean source files; theorem statement and hypotheses unchanged.
- Four proof tails now expose definitional local expressions, rewrite established hm/hp/hprod valuation equalities, then normalize ENat constants. Each matches its retained failed goal.
- No new axiom, premise or sorry added. Kernel acceptance remains a future replay result.
- All other Lean files, 18 ordered modules, 13 named readbacks, direct import/runtime/tactic bindings byte-identical. Independent positive and two negative controls remain unchanged.
- Runner changes exclusively fresh v2 workspace and unit names; 900/1800/900 seconds, 1 CPU, 8 GiB, deadline and failure/custody logic unchanged.
- All frozen local file hashes match inspected bytes. Prior failure log was read; original failed workspace remains intact.

Coordinator binds prior terminal/accounting, current lease/release and published exact source before one bounded replay; no new approval or test gate introduced.
