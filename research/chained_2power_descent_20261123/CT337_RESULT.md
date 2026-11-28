# CT337 — explicit Prime31 Mumford divisor pushforward and bielliptic kernel

**Executed 2026-11-23 UTC on the existing #4997 lane. Producer-authored/source-bound finite checks only; [pcc:unverified].** This is an additional positive-control algebraic component, not a complete ECDLP experiment or a new general-purpose transfer. It has no paid compute or cross-owner edits.

## Source identity and preregistration

- Before-work lane head: `258000400d474fd5257c9e638ca9d8792cbeb6cc`.
- [CT337_PROTOCOL.md](CT337_PROTOCOL.md), first committed `3e1ad9feb5ef6c58c41eaab7aa8e47538e792b98`, **before the published JS producer computation**. A local Python feasibility reconnaissance existed before protocol commit; it is explicitly excluded from the frozen-result provenance.
- [ct337_quotient_divisor_pushforward.js](ct337_quotient_divisor_pushforward.js): commit `f2cf52286f35325bad0c0c3001a35a072a1dde60`; Git blob `afba6fefcdaeb9cc8caa9ec70231daeeba2826de` (7,511 UTF-8 characters). The exact source was read back and source-identity checked before evaluating its top-level `ct337()` again; the standalone Node command is a replay recipe, **not claimed as executed**.
- [CT337_RESULT.json](CT337_RESULT.json): commit `deabec7212d998be0c5d9fc014209fdf8fa8425a`, Git blob `41bd449ee8bbfe64a116a27dd01719547e01159d` (11,444 bytes); source-produced JSON preserved in repository and read back.
- Actual coordinator tool execution: an embedded V8 call of the same source function, then a second V8 execution compiled from the exact remotely read-back source with its blob SHA checked; both returned the same count panel. The second call produced the frozen JSON. The separately written Python check ran with `python3 /mnt/data/ct337/ct337_verify.py` and `python3 -O /mnt/data/ct337/ct337_verify.py`; both passed; Initial Python checker SHA-256 `23348adf4d223a001f766bbd3cd58e38ae9750a809d375cbad819e87307abd4b` had Python `assert` statements disabled by `-O`; its earlier optimized-mode success is **withdrawn as a meaningful invariant check**. A1 hardened every assertion to an unconditional `if not ...: raise AssertionError` guard. The **final local, not published-to-GitHub** verifier SHA-256 is `7c945f0f337976afe46e5ff9b945fcde687b0416a78fad874f797229bb4db547`, executed successfully under both normal and `-O` Python with all fingerprints equal. One coordinator authored both languages; this does not meet the signed independent PCC gate.

## Derived generic divisor-map formulas

For `p = 2147483647`, let `h=7/2`, `t=x-h`, `u=t²` on
`C: y² = f(x) = ∏_(i=1)^6 (x-i)`.
With `a=1/4,b=9/4,c=25/4`, the two quotient curves are
`E+: v²=(u-a)(u-b)(u-c)` and `E-: w²=u(u-a)(u-b)(u-c)`.
The birational Weierstrass chart of E- is
`X=1/u, Y=w/u²=y/t³, Y²=(1-aX)(1-bX)(1-cX)`;
the E- point `(u,w)=(0,0)` maps to its cubic identity.

Let a generic **effective** Mumford support pair be
`U(x)=x²-sx+z`, `V(x)=Ax+v0`, `U | (V²-f)`.
Define `B=2h-s`, `C0=h²-hs+z`, `D=Ah+v0`,
`S=B²-2C0`. For `B*C0 ≠ 0` with distinct image-support roots, pushforward under the two explicit degree-two curve maps has coordinate-polynomial representatives:

```text
R+(Z) = Z²-S Z+C0²
V+(Z) = -(A/B)Z + D-A*C0/B

R-(X) = X²-(S/C0²)X+1/C0²
V-(X) = D/(B*C0) + (A-D/B-D*S/(B*C0))X
```

Derivation uses the identity `t²=-B t-C0` in the quotient algebra and `X=t^(-2)` on the E- chart. Both outputs satisfy `R± | (V±² - f_E±)` exactly. Crucially these are **effective image support/ordinate records**; obtaining the actual group image of a `Pic^0` class must also subtract the appropriate image of the reference degree-two divisor and add/reduce the elliptic points. The generic formulas deliberately reject `B=0` (coincident quotient x-support) and `C0=0` (E- chart at identity). They do not claim an everywhere-defined Cantor API.

## Executed finite-field results

| Check | Actual result |
|---|---|
| Rational source point panel | **36**, with exact C/E+/E- equations checked |
| Both finite degree-two fiber cross/diagonal pushforward identities | **36/36** point panels |
| Distinct rational source-point Mumford pairs, generic chart | **627** verified; 3 pairs excluded due to `B=0` |
| F_(p²) irreducible quadratic support, genuine nonsplit `U=x²-d` | **20**, constructed with quadratic extension square roots and checked as full quotient-ring identities |
| Effective deliberate mutation rejections | **4/4**, including altered E+ ordinate, altered E- ordinate, invalid source ordinate and wrong involution sign |
| Explicit 2-torsion kernel candidates | `(1,6)`, `(2,5)`, `(3,4)`: each has zero image in **both** elliptic group laws |

The independent-language Python rederived and checked every generic split pair and all 20 nonsplit supports using separate finite-field arithmetic and cubic group addition. Its deterministic FNV64 fingerprints on rational-point rows / 20 nonsplit records / first generic example matched the frozen JSON exactly:

```text
880ffdccfc1fb888
6522201d2f68639e
d1cf6837376e28ea
```

These fingerprints are comparisons of the concrete finite records, **not cryptographic authentication or a PCC signature**. The first nonsplit control `d=7` has F_(p²) ordinate `(1122322858,861401909)`, with quotient-image polynomials given in the frozen JSON. The actual base-field E+ and E- ascending cubic coefficient arrays are respectively `[1040187388,402653200,536870903,1]` and `[1,536870903,402653200,1040187388]`.

For each finite source point P, write `σ(t,y)=(-t,y)`, `ρ(t,y)=(-t,-y)`. On **actual cubic point additions**, `π+P+π+σP=2π+P` and `π-P+π-σP=O`; reversing to the ρ-fiber gives `π+P+π+ρP=O` and `π-P+π-ρP=2π-P`. The two source infinity images on E- are `(X,Y)=(0,±1)` and cancel; both infinity images on E+ are O. Thus these controls realize both diagonal-`[2]` and vanishing-cross-factor **fiber pushforwards** for the public sample. They are not native Jacobian/Cantor group-law tests.

## Kernel interpretation and mathematical boundary

Define `D_(i,j)=[W_i+W_j-∞_+-∞_-]` for branch points `W_i=(i,0)`. Direct elliptic addition shows the three nonzero classes `D_(1,6)`, `D_(2,5)`, `D_(3,4)` are killed by `Φ=(π+*,π-*):J(C)→E+×E-`. They sum to zero because `div(y)=Σ_i W_i-3(∞_++∞_-)`; `div((x-i)(x-j))=2D_(i,j)`. Standard genus-two 2-torsion classification gives their distinctness. Since the CT336 pulled-back differentials are independent and the two quotient maps have degree 2, the classical bielliptic `(2,2)` splitting yields degree 4 and these four elements form the expected full kernel. The corresponding dual composition is `Φ∘(π+^*+π-^*)=[2]×[2]`, with cross maps zero. This statement has a mathematical proof via involution eigenspaces, but **no Lean verification or full native implementation**. On odd-order torsion, `Φ` is injective and its image has the same DLP, yet this deliberately split/weak positive control gives no improved complete ordinary-ECDLP solver.

An exceptional `B=0` quotient-coordinate collision appears exactly for the three Weierstrass paired supports in this rational sample and must not be interpreted as a malformed Mumford input. The `t=0` source fiber lies over `f(h)=-abc`, a nonsquare in Fp, so the exceptional E- identity chart has **no Fp-rational affine source point** for this model; its geometric/extension-field behavior is specified but not natively tested. Ramified and degree-drop cases remain for a true Cantor implementation.

## Controls, limitations, ownership and next gate

The local exploratory Python verifier initially had a wrong coefficient-order unpacking/array orientation; those errors were corrected before its final two successful normal/-O runs. No published source or frozen JSON changed. The exact source and data were checked back after commit; there is no paid timing run, source mutation of other lanes, extra CI job, root repair, EKS call or protected merge.

Native parallel subagent slots and Rust compiler were unavailable in this execution context. One coordinator performed the algebra, V8 producer, separate Python arithmetic review, frozen readback and repository writes. The native F3/E7 transfer/Cantor lane remains [crypto #5007](https://github.com/isogeny-labs/crypto/pull/5007); original authenticated 695/701 norm-reuse, `C169/C13a/C13b/N2_actual/D` physical pricing and matched strongest cold rho/BSGS/IC plus memory/recovery/setup are owner-reserved and **not touched**.

**Next:** independently validate the exact committed source/results, implement affine/projective exceptional cases in the owned Rust divisor path, compare actual Cantor `Pic^0` images and dual composition with verified public toy DLP. Only then consider the fully charged single-target baseline/candidate comparison. Keep #4997 draft and stacked, preserve the CI/review/custody/paid STOP gates.

Sources checked: [CT336](CT336_RESULT.md), [CT335](CT335_RESULT.md), [Shaska's genus-two elliptic-subcover survey](https://arxiv.org/abs/1209.3187), [Magma's Richelot documentation](https://magma.maths.usyd.edu.au/magma/handbook/text/1615). The research-KB MCP and local wrapper and Conductor were not available in this runtime, so the existing published repo reports and accessible primary mathematics served as fallback; no literature novelty claim.


## CT337/A1 validation hardening — same-day amendment [pcc:unverified]

Post-publication audit found a **validation-control defect in the local Python oracle**, not in frozen JS source or raw results: its checks were ordinary Python `assert` expressions, so `python3 -O` did not execute them. The initial `-O` invocation was not a valid guard. The checker was transformed to unconditional `if not <invariant>: raise AssertionError` checks, retained locally with SHA-256 `7c945f0f337976afe46e5ff9b945fcde687b0416a78fad874f797229bb4db547`. Both normal and optimized invocations then rechecked 36 fibers, 627 split supports, 20 nonsplit supports and all three original frozen-record FNV64 fingerprints. A separate **deliberately corrupted expected checksum** causes both Python modes to exit nonzero (mutant file kept local). The source JS Git blob `afba6fef`, raw JSON Git blob `41bd449e`, protocol and all counts remain unchanged. Earlier local checker bytes are not represented as an independent validation artifact. This is still a producer/coordinator-run cross-language control, not signed PCC.
