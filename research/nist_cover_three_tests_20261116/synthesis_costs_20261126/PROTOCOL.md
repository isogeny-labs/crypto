# Fixed selected-kernel construction replay

Question: can the retained kernel-3 construction be rebuilt from its recorded
torsion/module inputs under the existing execution caps, with every executed
stage cost retained and its outputs bound to the maps already verified?
This is construction and checking evidence, with zero new PCC claims.
[pcc:unverified]

The inputs are GIVEN: the P-192 d7 Prym model, recorded actual-cover torsion
basis, selected isotropic kernel list, and module metadata. Generating those
inputs and discovering/selecting kernel 3 are outside this replay. Their
historical construction clocks have not been recovered; no total including
them may be inferred from the replay time. The original scripts are executed
unchanged. `prepare_kernel.cpp` performs their existing guarded selector
derivation. The arithmetic formulas are unchanged.

Prediction/acceptance: successful native exits and the original scripts'
checks; regenerated quartic, backmap and formal/global correspondence must
match retained artifacts at explicitly compared fields. A failed stage or
timeout stops that attempt. It is retained with its exit, output and elapsed
cost, and is not a negative mathematical result. Any fix requires a fresh
attempt with its source changes described. No cap increases.

The chain is Weil3, exceptional Weil charts, level2, quotient theta, Riemann,
descent, quartic export, destination relations, target Weil, differential,
elliptic backmap, formal series and global correspondence. The unchanged
stage programs include their existing adversarial controls; a new JSON/file
checker binds the regenerated artifacts to the retained map inputs. It does
not independently reimplement the arithmetic. Earlier separate OpenSSL map,
composition and KM-interface checks remain the independent arithmetic evidence.

Instrumentation: `CXX` points to a transparent compiler wrapper. Every compiler
invocation runs on one selected allowed CPU, under 1 GiB virtual address
space, with a 300 s timeout. A successful output is copied into the attempt's
ignored `bin` directory and replaced at its temporary original path by a shim
that runs that exact ELF under the same CPU/memory limits and a 600 s timeout.
Each invocation retains original argv (NUL-separated), cwd, SHA-256 of its
executable, stdout/stderr, exit status, and Bash wall/user/system time. The
scripts' own executable digests therefore bind the shim; `bin/*.sha256` and
call receipts bind the true ELF separately. The complete outer interval is
also recorded. Nested call costs must not be added to that outer interval.

All original top-level sources/scripts/protocols and the four given inputs
are copied and hashed before execution. The replay uses the copy. Fresh output
directories are mandatory, original evidence remains read-only, and source
hashes are checked after execution. Failed attempts remain alongside successful
ones. Compiler version, CPU affinity, OS and source commit are recorded.

Prior work reviewed: the original `check-recorded-kernel.sh` and
`prepare_kernel.cpp`; `QUOTIENT_THETA_PROTOCOL.md` and `RIEMANN_PROTOCOL.md`
(Tian, [arXiv:2007.03172](https://arxiv.org/abs/2007.03172), §4.1 equations
4.1–4.3 and Theorem 4.2); `DESCENT_PROTOCOL.md`
([arXiv:1409.5032](https://arxiv.org/abs/1409.5032) and
[arXiv:1612.02049](https://arxiv.org/abs/1612.02049)); and the retained
elliptic-backmap, formal-series and global-correspondence protocols. This
reuses established construction code and makes no novelty or speedup claim.
[pcc:unverified]

The user waived coordinate-screen and matched-rho requirements for this task
on 2026-11-26. Scalar recovery is outside the acceptance criteria.
