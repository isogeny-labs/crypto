## 2026-11-05 — Universal linear constructor (T-1609)

### What was done (commands, files, features)

Derived the h-shift finite-difference identity for every odd prime and a matching Vandermonde lower bound for fixed affine-linear shifted-character combinations. Implemented the native label-free factorial constructor with a max_terms allocation limit. All 41 optimized overflow-checked tests pass: 2197 point identities across 31 fields, 30 independent polynomial expansions and eight unchanged-source integrations using 239 signs. Sixteen source/data locks verify before and after build.

### Decisions made (and why)

Keep this as a general constructor and a structural control. At r479 it uses 239 character queries versus the cubic packet's 16; no speed claim follows. The lower bound redirects compression effort toward nonlinear feature construction or a directly evaluated aggregate. Finite differences and Vandermonde arguments are standard; no novelty claim is made.

### Open threads (unfinished, pick up next time)

Compact nonlinear coefficient construction and full source/setup/verification accounting remain. The journal was appended after T-1607 became terminal and a fresh exclusive scope was acquired. The original 4563-case cohort stays separate.

### Errors (what went wrong, how it was handled)

No native test failed. Four compiler warnings are preserved. Conductor initially denied the shared dated-journal scope; the held file was left untouched until the owner became terminal and T-1611 acquired it. No cloud submission or external controller action occurred.
