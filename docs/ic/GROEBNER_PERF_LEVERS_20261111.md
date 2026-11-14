# Gröbner-basis performance levers for the decomposition oracle (2026-11-11)

Owner's question: "Any other perf optimizations here? Can we use tricks like
dynamic programming or things that I'm not aware of for Gröbner bases?"
This note answers it against what the repository already does. The oracle
solves many small structurally identical boolean systems (the Weil descent of
one Semaev polynomial, 12 to 24 unknowns, one system per decomposition
target), so the levers are ranked by how much of that workload they touch.
Measurements cited are stage diagnostics from the three M4GB studies
(`research/m4gb_*_2026111*/`), local container, one thread, `[pcc:unverified]`.

## Where the time goes today

- Per call, the in-house `inherited-f4` is 15 to 25× below M4GB at every size
  (1 vs 16 ms at 14 unknowns, 18 vs 462 ms at 24). M4GB passes the in-house
  full-basis `f4-f2` from about 22 unknowns (3.7× at 24). M4GB's GF(2) time
  is in the Gebauer–Möller `update()` and in dense tail rows, not in the
  reduction kernels the 0001 patch vectorised for GF(p).
- At 12 to 16 unknowns the solve is a few milliseconds and the fixed cost of
  one process (start, field tables, thread pool, pipes) is of the same order.

## The levers, ranked

1. **Amortise the process (done here).** Patch 0003 adds `--serve`; the
   adapter `m4gb-f2-server` keeps one process per binary. Removes the fixed
   cost only; the solve is unchanged. Measured in
   `research/m4gb_persistent_server_20261111/`.

2. **Reuse structure across the systems of one cell.** This is the
   dynamic-programming shaped lever. Every target of one cell has the same
   monomial support and the same shape of Macaulay matrix; only coefficients
   differ. Three forms:
   - *Gröbner trace (Traverso 1988).* Record, on the first system of a cell,
     which S-pairs were useful and which reductions were zero; replay that
     trace on the next system, skipping the pairs the trace discards and
     checking the result with one reduction pass. M4GB has no trace facility;
     in-house, `inherited-f4` already does the closest thing: children
     specialise their parent's reduced basis instead of restarting
     (`docs/ic/FRAMEWORK.md`, engine table), which is why it wins every cell.
     The open version is a trace across *targets*, not across tree nodes: a
     recorded pair schedule keyed by the cell's shape, replayed with a
     verification pass. Risk: traces are only valid when the systems are
     generic in the same way; a target whose system degenerates (lower rank,
     an early linear relation) needs a fall-back to the full computation.
   - *Shared symbolic preprocessing.* The monomial-to-column map, the
     multiplier choice for each reducer and the sorted column order are the
     same for every system of a cell; computing them once and only refilling
     coefficients cuts the symbolic phase to zero after the first target.
     `f4-f2` recomputes them per call.
   - *Linear-algebra memoisation.* Rows that depend only on the Semaev
     polynomial and the subspace basis (not on the target) are identical
     across targets; their reduced forms can be stored once. The
     target-dependent rows are the minority.

3. **Stop at the decision, not at the basis.** The oracle needs "is there a
   solution in the subspace, and which", not a reduced basis. A degree cap
   (M4GB has `#ifdef MAXDEGREE`; the in-house `matrix-f4`/`matrix-f5` stop
   at `max_degree` and split on a variable) plus propagation of forced
   variables is what makes `inherited-f4` cheap. For M4GB the cheap
   experiment is a `MAXDEGREE`-bounded build per cell with the exhaustive
   check on the residual; it trades completeness for a bounded matrix.

4. **Bit-pack GF(2) rows inside M4GB.** M4GB's tail rows are
   `std::vector<coefficient_t>` with one byte per coefficient and byte-wise
   XOR; the in-house kernels (`gf2_elim.rs`) are word-packed with Four
   Russians tables, which is most of the 15 to 25× gap at small sizes. A
   packed `gf2` specialisation of M4GB's `polynomial_dense` type is a
   contained patch (one type, the kernels of patch 0001 already isolate the
   arithmetic) and is the next M4GB-side lever with a predictable payoff.

5. **Gebauer–Möller update data structures.** M4GB's `update()` builds a
   `std::map` per new basis element and tests divisibility on integer-coded
   monomials one at a time. A divisor tree or a bitset over the current
   leading monomials per degree, and batching the update over all new
   elements of one degree step, removes the allocation churn that profiles
   show at GF(2).

6. **Signature criteria (F5, GVW, M4GB's own criteria).** They remove
   reductions to zero up front; in-house `matrix-f5` exists and does not
   beat `inherited-f4` on these systems `[pcc:unverified]` because the systems are small
   and the zero reductions are few after the Gebauer–Möller criteria. Low
   payoff here.

7. **Hybrid and guess-and-determine (Bettale–Faugère–Perret; crossbred,
   Joux–Vitse).** Fix k variables, solve 2^k smaller systems; in-house
   `crossbred-f2`, `xl-f2`, `fes-f2` (Bouillaguet et al. exhaustive search)
   and `sat-cdcl` cover this family. At 12 to 24 unknowns exhaustive search
   is already competitive (`fes-f2` decides every cell of the descent ladder
   below M4GB); the hybrid regime starts past about 30 unknowns.

8. **Symmetry and Frobenius.** The Semaev polynomial is symmetric in its
   arguments and, on Koblitz curves, Frobenius-invariant; the
   `koblitz-symmetrised` and `koblitz-orbit` factor bases already quotient
   the system by that symmetry before the solver sees it, which lowers the
   degree of regularity rather than the solver's constant.

9. **Threads.** M4GB parallelises reduction; at these sizes one thread is
   best and the e2e ladder ran with one. Throughput across targets comes from
   running cells in parallel processes, not from threads inside one solve.

## What this means for the question

Dynamic programming in the textbook sense (overlapping subproblems with a
memo table) appears in lever 2: the overlapping subproblems are the shared
symbolic structure and the target-independent rows of one cell's systems,
and the memo is the trace or the preprocessed matrix skeleton. The
repository's `inherited-f4` is already the strongest instance of that idea
across the search tree of one system; the unexploited direction is across
the targets of one cell. Everything else on the list is a constant-factor
lever inside one solve.

## Sources checked

KB (`research-kb`) unavailable in this session (no executable). Read:
`docs/ic/GROEBNER_F4F5F6_AUDIT_20261010.md` (§5, "the fastest version of
this"), `docs/ic/FRAMEWORK.md` (engine table), `docs/ic/M4GB_INTEGRATION_ASSESSMENT_20261110.md`,
M4GB `src/m4gb.hpp` (`update()`, `MAXDEGREE`), `lib/polynomial_dense.hpp`.
Literature: Traverso, "Gröbner trace algorithms" (ISSAC 1988); Makarim and
Stevens, "M4GB" (ISSAC 2017); Bettale, Faugère, Perret, "Hybrid approach"
(J. Math. Crypt. 2009); Joux and Vitse, "A crossbred algorithm" (2017);
Bouillaguet et al., "Fast exhaustive search for polynomial systems in F2"
(CHES 2010).
