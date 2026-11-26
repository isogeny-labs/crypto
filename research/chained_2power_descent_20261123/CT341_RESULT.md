# CT341 result — F103 rational-point cokernel and complete genus-two Jacobian census

**2026-11-23 UTC; executed locally with exact field arithmetic; producer/coordinator-authored only, [pcc:unverified].** Pre-registered at [CT341_PROTOCOL.md](CT341_PROTOCOL.md). The first executed source failed its independently computed Jacobian-cardinality gate; that negative is preserved in [CT341_AMENDMENT_A1.md](CT341_AMENDMENT_A1.md), committed **before the corrected experiment was rerun**. Unlike the original protocol's mistaken expectation of 2300 total classes, the corrected census establishes that a degree-four isogeny need not be surjective on rational points.

## Primary result: rational-point cokernel, not a transfer shortcut

For `C/F103: y²=∏_{j=1}^6(x-j)` and the two explicit CT336–340 elliptic quotients, let `Phi:J(C)→Eplus×Eminus` and `Psi:Eplus×Eminus→J(C)` be the pushforward/product pullback.

| Complete finite census | Observed |
| --- | ---: |
| `#Eplus(F103)` / `#Eminus(F103)` | 92 / 100 |
| `#(Eplus×Eminus)(F103)` | **9,200** |
| `#C(F103)` and `#C(F103²)` | 88 / 10,862 |
| `#J(C)(F103)` independently from curve zeta function | **9,200** |
| Distinct source classes enumerated by rational split/tangent and nonsplit degree-two divisor supports | **9,200** |
| `#Psi((Eplus×Eminus)(F103))` | **2,300** |
| Rational image index for `Psi`, and product image index for `Phi` | **4 / 4** |
| Rational `ker(Psi)` / `ker(Phi)` | **4 / 4** |
| Every product-point composition `Phi Psi=([2],[2])` | **9,200/9,200** |
| Every rational-Jacobian composition `Psi Phi=[2]` | **9,200/9,200** |
| Product `Psi` fiber multiplicities | **2,300 fibers, exactly 4 each** |
| Changed-source mutation controls | **6/6 expected failures** (3 variants × Python normal/-O) |

This is an exhaustive check on this exact small finite curve, **not** a proof for arbitrary curves, signed third-party validation, or a fast complete ECDLP on an ordinary prime-field elliptic curve.

The independent Hasse–Weil cardinality oracle uses `t_1=103+1-88=16`, `t_2=103²+1-10862=-252`, `e_2=(t_1²-t_2)/2=254` and

`#J(F103)=1-t_1+e_2-103t_1+103²=9200`.

The source enumeration separately counts **5,330 nonsplit quadratic supports, 3,700 split, 82 repeated/tangent, 85 ordinary degree-one, two z=0/infinity-chart degree-one, and identity**. This yields all 9,200 unique normalized classes and includes **6,900 classes not in the rational image of `Psi`**. The three Weierstrass-pair classes `(1,6), (2,5), (3,4)` plus identity give exactly `ker(Phi)(F103)`.

## Why the preregistered failure mattered

The original hypothesis incorrectly equated the 2,300 rational-image elements with the size of the *whole* Jacobian. Rationally defined isogenies give equal group orders over finite fields, but maps on rational points can have nontrivial cokernels. The first frozen protocol's `#J=2300` gate failed with `RuntimeError: Jacobian order from independent zeta != 2300`; its local initial source SHA-256 was `c7554c23482d69dc511d9d41f552c6577dfc9ef8380847a9cef974c63c0125d8`, not archived as executable bytes. The A1 amendment was committed before modifying the test and executing the corrected candidate. This failed test must remain a negative result, not be silently promoted.

## Source custody, reproducibility and tamper controls

Published immutable-source inputs: original CT339 Python SHA-256 `c798c3201da743091be50e1a3c3cd63200f2e995e72d17991518db3ca5345695`, frozen CT339 JSON SHA-256 `eb986754a030062db0aa629989edf1ac2705695f5d8b1863fc19dab28c085492`. The CT341 Python source SHA-256 is `4793c969e2c563ff55323e4c78c19affdda838445652cc277942da7afcc31086`, with gzip-base64 archive [ct341_full_jac.py.gz.b64](ct341_full_jac.py.gz.b64), Git blob `fe060e2b970be1afc00c9d3fe72a1cc2cb526e23`. The canonical [CT341_RESULT.json](CT341_RESULT.json) has SHA-256 `ecc449a98eeb3d1554329d27e61ef45fa3aa408760ce2aeb76e84c060f5ac514`, Git blob `3265f7b3bb2877c09bb8236718d3c2de3e67b03a`. Both blobs were reread from GitHub and matched local Git object hashes. One base64 transcription byte was caught by Git blob mismatch and corrected in an additive follow-up commit; the **verified** blob is the one above.

From `research/chained_2power_descent_20261123/`:

```sh
tmp=$(mktemp -d)
base64 -d ct339_f103_toy_dlp.py.gz.b64 | gzip -d > "$tmp/ct339.py"
cp CT339_RESULT.json "$tmp/ct339_run.json"
base64 -d ct341_full_jac.py.gz.b64 | gzip -d > "$tmp/ct341.py"
printf '%s  %s\n' 4793c969e2c563ff55323e4c78c19affdda838445652cc277942da7afcc31086 "$tmp/ct341.py" | sha256sum -c -
(cd "$tmp" && python3 ct341.py > ct341_normal.json && python3 -O ct341.py > ct341_opt.json && cmp ct341_normal.json ct341_opt.json)
cmp "$tmp/ct341_normal.json" CT341_RESULT.json
for m in mutant-sign mutant-w1 mutant-hash; do
  (cd "$tmp" && python3 ct341.py --"$m") && exit 1
  (cd "$tmp" && python3 -O ct341.py --"$m") && exit 1
done
```

The exact local normal and optimized JSON streams matched byte-for-byte. Six mutant processes all exited nonzero, rejecting respectively incorrect generic ordinate sign, invalid W1 exceptional handling, or original CT339 fingerprint corruption, in both Python modes. This was one coordinator running two interpreter modes, **not independent signed/PCC validation**.

## Actual roles, ownership, missing work

- Coordinator: read current AGENTS/ACTIVE, #3877/#4997/#5007/#4215/#3904 and frozen CT339–340 sources and handoffs; reserved only the new CT341 scope; committed draft PR #5045, protocol, additive A1 failure correction and source/results; ran finite Python checks and Git blob readback.
- Mathematics package: finite-field zeta/cokernel logic and the rational-kernel correction were coordinator-derived; primary references were CT336–340, Diem (https://arxiv.org/abs/math/0312413), and Magma Richelot handbook (https://magma.maths.usyd.edu.au/magma/handbook/text/1615).
- Implementation package: local Python full-support enumeration using unchanged CT339 Cantor arithmetic; not Rust, not original norm-point reuse.
- Validation package: local F103² independent point-count oracle, 18,400 checked compositions, and six deliberate failures; not separately staffed or signed. No native child worker slots, research-KB MCP, Conductor or Rust compiler exposed.

**Not executed:** original EKS-only authenticated 695/701 model or `C169/C13a/C13b/N2_actual/D` physical prices, same-host cold strongest rho/BSGS/IC, whole EKS fixture custody, ordinary-source DLP transfer, signed PCC, large-field timings or resource-spend action. No competing benchmark reservation, paid compute, protected merge or new automation. [pcc:unverified]

**Next:** source-pinned independently owned Rust/Cantor replay of the complete finite-Jacobian census and off-image classes, with projective/nonsplit/ramified exceptional controls and actual scalar recovery. Keep the original cold benchmark owner/reservations intact.


## Independent-language arithmetic addendum (same coordinator)

After the first full Python result was frozen, a separate, self-contained Node.js implementation enumerated the **10,609** elements of F103², built a square-root multiplicity table without importing CT339 or CT341, counted the genus-two odd model and both elliptic cubic models, and independently recovered `#C(F103)=88`, `#C(F103²)=10862`, `#Eplus=92`, `#Eminus=100`, `e2=254`, `#J=9200`. Its F103² square set has **5,305** elements and exactly five zeros of the odd quintic. This independently corroborates the mathematical correction of the original 2300 cardinality hypothesis, but the complete 9200-group dual-map identities were still verified by one imported Cantor implementation, not a signed independent validator.

Command: `node ct341_independent_zeta.cjs > /tmp/ct341_independent_zeta.json && cmp /tmp/ct341_independent_zeta.json CT341_INDEPENDENT_ZETA.json`. The source [ct341_independent_zeta.cjs](ct341_independent_zeta.cjs) has SHA-256 `56e84a2b186427393fa6b57fb001d0abed4aac2c96481eddd6ba5894edcbf335`, Git blob `9ed104606efc07b4c8f571fe5ed3be2b9891167e`; frozen [CT341_INDEPENDENT_ZETA.json](CT341_INDEPENDENT_ZETA.json) has SHA-256 `b06d861207ac5cca2c6e92d31581cc966c8f1f891c5e111a77f5e26b1716e058`, Git blob `ed77a2c08feec011ba48b8a6d20fb894e7b02c42`. Both GitHub blob IDs were reread and matched local objects. [pcc:unverified]
