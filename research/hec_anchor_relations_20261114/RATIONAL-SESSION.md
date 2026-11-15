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
