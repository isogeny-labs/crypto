# Destination arithmetic gate, frozen before execution

Scope: correctness and reuse, not speed measurement or explicit isogeny evaluation.
Input is the two exact quartics in geometry13 RUN.txt. No input selected by results.
Reuse the pinned plane_quartic.rs section-space arithmetic, without modifying any
existing source. extract.sh records the exact extraction; reused.rs is committed.
Only standalone prime checking, scalar double-and-add and SplitMix wrappers added.
Reverse coefficient order and normalize X^4 to 1, preserving projective model.

Before scientific execution commit source, Cargo.lock, this protocol and SESSION;
upload source checkpoint. Build is allowed before checkpoint. Run once in release,
retain stdout/stderr and failures. No adaptive fixture replacement or rerun.

For both curves count points over F127 and F127², compare to registered predictions
(127,16043) and (129,16043), and construct the existing Jacobian representation.
Choose exactly two triples from enumerated rational points, indices 4..6 and 7..9.
Check identity, inverse, doubling and predicted order annihilation on both;
check commutativity and one associativity identity. On kernel1 double each class,
require nonzero and annihilation by prime 1028893. Print all class encodings.
Failures remain terminal evidence. Doubling/double-and-add comparison is a wrapper
control, not independent validation of the group law. Raw field counts are diagnostics,
not an optimized algorithm comparison. No third-extension exhaustive count.

Order inference: a1=128-N1, a2=(a1²-(127²+1-N2))/2. Any possible a3 has
|a3|<=20*127^(3/2)<28625, so the full candidate Jacobian-order interval has width
<57250<1028893. A nonzero prime-order class and counts N1,N2 uniquely determine
the group order inside that interval. This argument is conditional on reused
Jacobian arithmetic being correct; tests here are local checks, not an independent
implementation. No scalar preimage or source DLP used. Divisor transport remains
unevaluated and must not be inferred from target class encodings.

Prior art: Khuri-Makdisi, https://arxiv.org/html/math/0105182v3 §§2–5,
sections as divisor subspaces, product/colon/flip; reuse repo implementation.
Tian https://arxiv.org/html/2007.03172 §4.2: Kummer coordinate alignment and lifting
still required. Earlier KB lookup found the existing plane-quartic implementation;
KB revision9141e386254340a9bdba3036f2bb706a refreshed2026-11-14, predates geometry13.
No novelty claim. No change to closed records, defaults or leaderboard.
