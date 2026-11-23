# CT340 — complete rational-point dual composition for an explicit bielliptic genus-two Jacobian

**Executed 2026-11-23 UTC; [pcc:unverified].** The preregistered protocol [CT340_PROTOCOL.md](CT340_PROTOCOL.md) was committed at `601d2e1dd3a03ca8b533f8eb3021451044e008dc` before CT340 source implementation or execution. The frozen predecessor #4997 head was `186fc8076629ecfb46e4677a6c77801726762d78`. This is a deliberately split **F103 structural positive control**, not an ordinary prime-field ECDLP performance comparison.

## Mathematical construction: actual dual homomorphism

Let `C:y²=∏_(j=1)^6(x-j)`, `h=7/2`, `a=h-1=5/2`, and `(z,eta)=(1/(x-1),yz³)`. The odd quintic is `eta²=F(z)=∏_(j=1)^5(1-jz)`. The existing CT336/339 quotient maps `pi_+:C→E_+` and `pi_-:C→E_-` give `Phi=(pi_+*,pi_-*):J(C)→E_+×E_-`. **New implementation:** explicit `Psi(P,Q)=pi_+^*(P-O)+pi_-^*(Q-O)` computed as actual F103 Mumford classes via the original CT339 Cantor arithmetic. For the generic elliptic fiber, the two preimages are encoded without factoring:

- `P=(u,v)∈E_+`, `d=a²-u≠0`: `U=z²−(2a/d)z+1/d`, `V≡v z³ (mod U)`.
- `Q=(X,Y)∈E_-`, `d=a²X−1≠0`: `U=z²−(2aX/d)z+X/d`, `V≡Y(1−az)³ (mod U)`.

Each pair is verified with the unchanged `norm` guard `U|V²−F`. At the W1 fiber `(u,v)=(25/4,0)` and `(X,Y)=(4/25,0)`, the returned class is `[W6−W1]` (in z coordinates `U=z−1/5,V=0`). The Eminus ramification points `(X,Y)=(0,±1)` return `2[Q∞±−W1]` through Cantor. Group identities return the zero class. This preserves the exceptional infinity/ramification geometry rather than relying on a denominator-limited generic substitution.

**Proof argument (ordinary mathematical reasoning, not formal Lean/PCC):** The quotient involutions on `C` are `sigma_+:(t,y)↦(−t,y)` and `sigma_-:(t,y)↦(−t,−y)`. Their composition is the hyperelliptic involution `iota`, acting as `−1` on `J(C)`. For separable degree-two covers, `pi_i^*pi_i* = 1+sigma_i*`. Therefore

`Psi Phi=(1+sigma_+*)+(1+sigma_-*)=[2]`.

Each diagonal `pi_i*pi_i^*=[2]`. Each cross map vanishes: the source involution acts as `+1` on the corresponding pullback, as `−1` on the complementary one; the cross morphism is annihilated by 2. Since `Hom(E_+,E_-)` and `Hom(E_-,E_+)` have no additive torsion, the cross maps are zero. Thus `Phi Psi=([2],[2])`. In particular, `Phi` is injective on **every odd-order subgroup** of this Jacobian, since `Phi(D)=0 ⇒ [2]D=0`. This is a standard bielliptic quotient fact, not a newly claimed general transfer on arbitrary ordinary elliptic curves. Relevant prior sources: [Magma Richelot handbook](https://magma.maths.usyd.edu.au/magma/handbook/text/1615), [Diem, degree-two genus-two elliptic covers](https://arxiv.org/abs/math/0312413), existing [CT339_RESULT.md](CT339_RESULT.md).

## Actual bounded finite arithmetic

The implementation SHA-256 is `d81cc062589682b7b8454e40cc63027949a314bed3e7e2aa564c6b042d6b0603`, stored as [ct340_dual_pullback.py.gz.b64](ct340_dual_pullback.py.gz.b64) (Git blob `2890e789e87cf2ad1b4469f3f780322e06da1960`). It verifies the unchanged upstream decoded CT339 source SHA-256 `c798c3201da743091be50e1a3c3cd63200f2e995e72d17991518db3ca5345695` and the upstream raw result `eb986754a030062db0aa629989edf1ac2705695f5d8b1863fc19dab28c085492`, then runs **the original CT339 source in both normal and optimized Python** and checks both byte-identical against that source's previously frozen JSON.

Fresh frozen output [CT340_RESULT.json](CT340_RESULT.json), SHA-256 `db359ab64fbc49a3e12eb8a55d1a0a65ae937a69475fdc7dc80d7ceb9ad965f0` (Git blob `79183bf7ab1575f8443be8d768738ec3740bef04`). Counts:

| Gate | Passed |
|---|---:|
| Every `E_+(F103)` point, including O | **92/92** |
| Every `E_-(F103)` point, including O | **100/100** |
| Full diagonal and cross-factor `Phi(Psi(P,O))`, `Phi(Psi(O,Q))` identities | **192/192** |
| Actual Cantor `Psi(Phi([k]D_i))=[2][k]D_i`, original eight classes times k=0..20 | **168/168** |
| Special W1 fiber / Eminus ramified points | **2/2 W1 branches; 2/2 X=0** |
| Existing order-23 generator and inverse of multiplication by 2 modulo 23 | **passed** |
| Intentionally wrong elliptic fiber sign and wrong W1 fiber, tested with Python normal/-O | **4/4 rejects**, nonzero process exits |

The 192 tested elliptic pullbacks are 186 generic supports, two group identities, two W1 special fibers and two Eminus Xzero ramification fibers. The 168 original source outputs include 89 nonsplit quadratic supports, 47 split, 16 repeated, eight degree-one, eight zero. Zero unexplained exclusions. The `Eminus X=0` divisor example is reduced `U=z²,V=102+59z`, and both W1 exceptional classes reduce to `U=z+41,V=0` over F103. These are source-checked actual divisor classes, not exclusively point-coordinate identities.

## Reproduce at exact published revision

```sh
cd research/chained_2power_descent_20261123
base64 -d ct339_f103_toy_dlp.py.gz.b64 | gzip -d > /tmp/ct339.py
base64 -d ct340_dual_pullback.py.gz.b64 | gzip -d > ct340.py
printf '%s  %s\n' d81cc062589682b7b8454e40cc63027949a314bed3e7e2aa564c6b042d6b0603 ct340.py | sha256sum -c -
python3 ct340.py > /tmp/ct340_normal.json
python3 -O ct340.py > /tmp/ct340_optimized.json
cmp /tmp/ct340_normal.json /tmp/ct340_optimized.json
cmp /tmp/ct340_normal.json CT340_RESULT.json
python3 ct340.py --mutant-sign     # must exit nonzero with specific sign invariant
python3 -O ct340.py --mutant-sign  # must exit nonzero
python3 ct340.py --mutant-fiber    # must exit nonzero with W1 invariant
python3 -O ct340.py --mutant-fiber # must exit nonzero
```

The frozen code automatically loads the sibling original compressed CT339 source and original frozen CT339_RESULT.json; it does not modify CT339 code or the original raw data. The four mutant nonzero exits are **expected failures**, not test failures of the unchanged candidate. Python normal/-O produced byte-identical CT340 JSON. No wall-clock timings, same-host complete solver costs, external baselines or peak memory were collected.

## Boundaries, ownership and next action

One coordinator designed mathematics, wrote and executed Python, inspected both mutation outputs and integrated the already-owned branch. **Zero native child workers** were available; no independent signer, Rust compiler, separate Rust verifier or formal Lean kernel check occurred. The local container had no compiler and no authenticated original norm-point model. Concurrent GitHub reads are not parallel scientific validation. Research KB/Conductor unavailable; prior repository evidence and primary literature used instead. [pcc:unverified]

The original 695/701-record Jacobian norm-reuse experiment and physically priced `C169/C13a/C13b/N2_actual/D` plus cold matched strongest optimized rho/BSGS/index calculus remain held by their separate owners. PR #5007 owns the genuine F3/E7 Rust transfers, and #4992 owns the formal two-hop subgroup gate; neither was modified. No paid/EKS compute, competitive timing, unauthorized branch merge, CI poll or new automation was launched.

**Next:** separately reproduce these published dual-elliptic pullback formulas in the approved existing Rust/Cantor owner (including full subgroup kernel and exceptions); then obtain an independently signed frozen-source replay. No complete single-target advantage is inferred from this intentionally split toy curve.

### Parent transition after CT340 publication

The formerly stacked parent #4992 merged on 2026-11-23 and its base branch was removed during the CT340 publication sequence. GitHub changed #4997's base to `main`. #4997 is deliberately restored to **OPEN/DRAFT** pending native and independent gates. This changes PR metadata only, not the frozen CT340 source, raw result, or claims. Queue record: [#3904 comment 7107972060](https://github.com/isogeny-labs/crypto/issues/3904#issuecomment-7107972060).
