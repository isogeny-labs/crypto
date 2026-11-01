# knowledge: Lean-certified exponent rules for index calculus (L10) -- DRAFT, do not open yet

## Files to add (all under crypto-autoresearcher)
- knowledge/frontiers/ecdlp/index-calculus/KR-IC-<new>.yaml x7 (drafts: leads/L10/rows/KR-IC-a10001..a10007.yaml; ids provisional, re-mint with tools/allocate_id.py --next known_result --area IC, then --check)
- knowledge/literature/KN-LIT-<new>.md for Akshima et al. (Bochum v0.2) and Caminata-Gorla 2112.05579 (no entry exists; rows a10001, a10005 carry TODOs)
- knowledge/findings/KN-FIND-<new>.md holding the Lean receipt (IcTensorRoute.lean + PrimePdpEncoding.lean::rho_floor), or vendor the Lean under formal/ per the formal skill, so rows can cite a resolving source
- knowledge/INDEX.md, SOURCES.md, sources.json regenerated (tools/build_knowledge_index.py, make sources)
- Leads: add known_results/related links on LEAD-EXO-hashed-factor-base-tensor-ecdl and LEAD-ICB-decomposition-encoding-and-sat (lead-atlas skill)

## Checks to run
python3 tools/build_frontier_map.py --check; tools/validate_ledger.py; tools/build_knowledge_index.py --check; tools/build_source_index.py --check

## Content
Seven known-negative / known-bound rows: black-box D.1 crossover and arity balance; no linear-vertex discount / no q-ary form; polynomial method vs guessing m-1 windows; target-family amortisation dominated; d_reg regrading; Fermat-indicator degree ~p; ladder rule + solver-saving floor reconciled with rho_floor. Lean builds: standard axioms only. Rows are scoped to the stated cost models; see REPORT.md for what is weaker than LEADS.md.

(Per the pr skill and AGENTS.md: merge origin/main first, never rebase; session receipt via tools/session_receipt.py.)
