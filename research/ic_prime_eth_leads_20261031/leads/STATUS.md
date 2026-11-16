# Lead evaluation status (2026-10-31, first launch)

Twelve Sonnet evaluators were launched, one per lead in ../LEADS.md section 5. The session's
permission mode ("don't ask") refused many calls that create source files or compile/run code
(and the Workflow tool outright); which calls passed was inconsistent. Agents that were refused
stopped rather than route around it. Results so far:

- L06 measured (Python, structural model of the Rust encoder): verdict closed. REPORT in L06/.
- L07 measured (FFT convolution over Z_r, 9 curves, b = 14/18/22, m = 2/3/4): constant_only; base
  choice is free; shared negation-closed x-windows LOSE Pr[X>0] (ratio 0.71 at m=4, b=14, closing
  with b). REPORT in L07/ (copied from /tmp/claude-501/L07).
- L10 partial: seven draft KR rows + PR description in L10/; Lean rebuilds with standard axioms;
  lists where LEADS.md overclaims the Lean (no_crossover needs c >= 1; interface_lower_bound is an
  injectivity count only; polymethod theorem covers only the 1/(2d) saving; items (b),(d) have no
  Lean receipt; sibling review's KR-2 is withdrawn, KR-1 is an upper bound only).
- L08 blocked, reading findings only (unverified): the ECC2K-130 prototype already uses a char-2
  Schönhage gadget with block width fixed at J/4; freeing it cuts transform words ~9x; hand bound
  edges < 80d + 228 sqrt(d) vs Bochum 1100(d+1); at b~130 balanced arity is 2 and N ~ 2^65.5
  before contraction vs rho 2^60.81; Frobenius folding breaks Lemma 10 pairwise independence.
- L11 blocked after build: four-mode patched sweeper (school|kara|pc|kpc, KISSAT_CONFLICTS) built
  at /tmp/claude-501/L11; running the binary was refused.
- L02, L03, L04, L05 (first run), L12 blocked before any code ran; designs recorded in their
  agents' reports (orchestrator transcript).

Environment facts (checked): kissat 4.0.4, cadical 3.0.1 CLI + cadical.hpp (ExternalPropagator) +
libcadical.a, cryptominisat5, clang++, cargo; no pysat/pycadical; Python 3.9.10 + numpy 2.0.2;
host load 50–66 (conflicts are the unit); /usr/bin/time -l unavailable.
Path corrections to LEADS.md: prime_sat_prefix_search_20261030 is under crypto-autoresearcher/research
(Python transcript adapter, no CaDiCaL propagator); Bochum hash is in
cryptanalysis/bindings/rust/hashed-ecdl-tensor/src/reduction.rs; prime CNF emitter is the sweep
worktree's sweeper (bakes the target into constants); fourier-width.md path does not exist.
Ladder correction: existing m = 3 prime sweep times out at 120 s with 2e5–9e5 conflicts even at
b = 12, l = 4; feasible pilots are the diagonal b = 6, 9, 12, 15 with a conflict cap.
