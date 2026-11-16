#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set heading(numbering: none)
= ECCp-131 setup now reaches full network construction
2026-10-30 · exact prime proofs · development validation

The unmodified delivery baseline stops in AKS. The follow-up extends strictly
bounded exact primality tests, verifies bundled public proofs automatically,
and attempts bounded Pocklington proofs before the exact fallback. The already
landed checked-hash context is preserved. No probable prime is accepted as
certified.

#image("integration.svg", width: 95%)

*Observed construction control.* The native package passes 94 tests, strict
all-target lint, formatting and Rust 1.78 checks. Original-input preflight
charges 17,471 abstract steps; explicit certified-hash preflight charges 5,897.
The matched construction control deliberately omits p/r certificates: the
unchanged baseline exhausts AKS, while the patch charges 19,722 through setup
and 19,925 through the network. These are uncalibrated diagnostics; no runtime
ratio is asserted. The earlier all-proofs control is retained separately.
The graph vector policy is two million; the original fixture's 200,000-entry
cap is too small for this graph. No base enumeration, count, witness or solve
was requested by this control.

*Validation limit.* The broader parent-library run did not finish one unchanged
legacy isogeny-walk case before SIGKILL. All 18 missing-fixture failures replay
successfully on the same executable after restoration. No full parent-suite
pass or general mathematical audit is claimed; the follow-up is held for that
remaining validation. Full-parent raw archival also awaits an authenticated
S3 connection; its local-only status is explicit.

*Sources and replay.* Sorenson/Webster, arXiv:1509.00864 Theorem 1.1;
HAC Chapter 4 Fact 4.40 and Chapter 3 Algorithm 3.9; pinned Bochum v0.2 Section 3
(fixed prime s in [2p,4p]). README.md, commands.txt, native-setup-check/ and
delivery records retain exact source, input, lockfile and output identities.
