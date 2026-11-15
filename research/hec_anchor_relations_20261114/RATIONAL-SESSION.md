# Rational-family working log, 2026-11-15

The shared dated log is held by another task (T-6020); this scoped log preserves
the work without modifying that task's file or the shared resume file.

## Protocol freeze

### What was done (commands, files, features)
Verified current PR4274 checks, read the held-out handoff and dated log, and
computed per-case conditional remaining-cost ratios with native jq. Added
RATIONAL-PROTOCOL.md with the algebra, primary prior art and frozen tests.

### Decisions made (and why)
Previous goal turn made progress through the remaining-cost diagnostic.
Pursue rational signed interpolation because it changes tuple arity and the
possible anchor dimension together. Treat its interpolation identity as
established mathematics, not a new result. Gate algebra before timing.

### Open threads (unfinished, pick up next time)
Implement direct subset and Walsh evaluation, independent Gaussian reference,
and exact norm/root/Jacobian gates, then inspect all degeneracy counts.

### Errors (what went wrong, how it was handled)
Conductor MCP uses HTTPS against HTTP; a per-command HTTP CLI override works.
Scoped task T-6024 holds this study directory. Shared session log is reserved
elsewhere and has not been edited. KB is lexical and predates current work;
primary paper Theorems 2.2 and 3.1 were read directly as fallback.

## Algebra gate completed

### What was done (commands, files, features)
Implemented rational.rs direct subset and Walsh evaluators plus an independent
homogeneous Gaussian reference. All five tests passed. The frozen curve panel
contains 9408 sign classes and 1108 verified complete Jacobian relations;
694 nonprimitive pairs were retained and rejected, with 81 degree drops.
There were no zero pairs or higher-nullity cases in the curve panel. Separate
all-zero controls exercise higher nullity; 2401 arbitrary F7 assignments and
branch-anchor/duplicate-abscissa controls pass. RATIONAL-GATE.txt is raw stdout.

### Decisions made (and why)
The established interpolation construction passes this finite algebra gate.
It warrants a bounded rank/yield screen. Common-factor rejection is essential;
neither the raw sign count nor successful interpolation establishes useful
independent relation supply. No timing or novelty inference follows.

### Open threads (unfinished, pick up next time)
Freeze a small-anchor rank/yield screen, compare against the original lower-pole
family, and count distinct admitted relations and attempted functions to rank.
The full cold pipeline and >=10x objective remain open.

### Errors (what went wrong, how it was handled)
No algebra mismatch occurred. Duplicate-abscissa panic in raw stdout is the
expected passing rejection control. Standalone rustc --test -O used the pinned
existing native library; this does not independently verify that library.
