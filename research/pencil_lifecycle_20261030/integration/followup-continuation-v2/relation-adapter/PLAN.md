# Offline augmented-row adapter verification

Source inspection before implementation: native_solver.rs GraphSolver::relation stores point signs, b, -a; candidate_verification checks d+sum=0 and walk_generation sets d=tau(aG+bQ). Thus the last stored column is a right-hand side, not the constant of Ax+bk+c=0. Negate that column for the existing offline information diagnostic. This is standard augmented-matrix semantics, not a new information criterion.

Software check scope frozen before execution: compile and test explicit F7 relation fixture, all343 single-row F7 augmented systems across49 assignments, wrong-domain/shape controls. Bound each command to120seconds. No curves, scalar search, timing comparison or scientific samples. Exact source and commands retained. Independent source fidelity must inspect the complement and label conventions, not only tests.

This adapter copies rows and performs diagnostic elimination outside measured collection; it does not capture production rows or supply missing historical costs. Full1,260target and matched-cost obligations remain unchanged.
