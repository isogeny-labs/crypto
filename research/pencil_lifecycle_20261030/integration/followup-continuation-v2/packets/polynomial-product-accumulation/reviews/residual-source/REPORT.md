Read-only source audit completed against current files:

- `src/cryptanalysis/three_leads.rs` SHA256 `26e99b56dace8f848d055e8709bb989622e4caea65aec26d63e00805153dc3c6`
- `src/cryptanalysis/three_leads_geometry_events.rs` SHA256 `a60e3894f4178cf5894b3c277991627ab15c1279a78daf2000e897064f0f20ee`

**Exact residual evaluation order**

| Source | Actual operation |
|---|---|
| `three_leads.rs:972–980` | `Jacobian.valid` short-circuits `degree <= 2`, then `u[degree] == 1`, then tests empty `prem(residual, up())`. |
| `:960–965` | `up()` copies `u[..=degree]`; `vp()` copies and trims `v[..degree]`. Stored tails outside these prefixes do not participate. |
| `:835–847` | `pmul(a,b)` returns empty immediately if either input is empty. Otherwise allocate zero vector of length `a.len()+b.len()-1`; traverse `i` outer, `j` inner; update `r[i+j] = add(r[i+j], mul(a[i],b[j]))`; trim only after all updates. |
| `:814–820` | `pneg` maps `neg` over every coefficient, preserving length; **it does not trim**. |
| `:801–812` | `padd` allocates `max(a.len(),b.len())`, substitutes zero for missing coefficients, computes one addition per slot, then trims. |
| `:822–823` | `psub(a,b)` first computes the complete `pneg(b)` vector, then invokes `padd(a,negative_b)`. |
| `:795–799` | `trim` repeatedly removes stored trailing words exactly equal to zero. |

Thus the residual proof needs the row-major convolution invariant plus padded subtraction:
\[
\operatorname{decode}(\operatorname{psub}(f,\operatorname{pmul}(v,v)))
=\operatorname{decode}(f)-\operatorname{decode}(v)^2.
\]

A fused coefficientwise subtraction model may have the same polynomial result but does **not** reproduce this source’s full operation order or instrumentation.

**Necessary field and representation assumptions**

1. `p != 2`, `n == 1`, `tables == None`. `mul` checks diagnostic tables **before** the binary/prime-field branches (`:210–225`); only then does it reach `bm`.
2. Prime \(0<p<2^{31}\), with canonical input words `< p` for the curve polynomial, participating `u` prefix, and participating `v` prefix. Canonicality is preserved by the arithmetic below.
3. For canonical `a,b`, multiplication computes `(a*b)%p` (`:194–197`), with \(ab < p^2 < 2^{62}\). Addition computes `(a+b)%p` (`:161–168`), with \(a+b<2p<2^{32}\). These intermediate bounds avoid `u64` overflow.
4. `neg` (`:178–189`) uses `(p-a%p)%p`, not the conditional form `if a==0 {0} else {p-a}`; for `n==1`, the loop has one iteration and terminal `k *= p` is safe. Canonical subtraction then calls `add(a,neg(b))` (`:191–192`).
5. Array-size arithmetic and allocation remain Rust execution obligations. For this caller `vp.len() <= 2`, so `pmul(v,v)` has at most three coefficients; its `i+j` is in bounds. The arbitrary-length curve polynomial’s allocation/copy behavior is outside a pure polynomial-value theorem.

**Caller implications and gaps**

- The `degree <= 2` guard makes both fixed-array slices and `u[degree]` access safe; the second guard ensures the chosen divisor list is nonempty and ends in one. These supply the monic remainder model’s structural preconditions.
- `Jacobian.valid` itself does **not** enforce canonical words, prime modulus, `n==1`, or disabled tables. Those must be premises or established by the caller path.
- `Scholten::new` constructs `j.f` as `Field::new(p,1,0)` (`:1144`, `:1158–1161`); `Field::new` initializes tables to `None` (`:71–81`). It explicitly checks its curve-polynomial words `<p` (`:1140–1143`).
- `Scholten::new` checks `p>13`, `p%4==3`, and primality (`:1095–1096`), but does **not** itself assert `p<2^31`. `Field.validate` has that bound (`:139`) but must actually be invoked on the relevant construction path.
- `tau` checks every returned `u/v` word `<p` immediately before `self.j.valid(out)` (`:1197–1201`). Its separate `self.j2.valid(d)` call operates over the quadratic field and is **outside** the prime-field model.
- For degree zero, the checker uses `u=[1]` and `v=[]`; residual divisibility is automatic. It ignores other stored array entries. Any theorem identifying the accepted record with the specific stored identity `Divisor::O` needs stronger tail premises.
- Geometry helpers preserve vector contents on normal return (`geometry_events.rs:120–136`), but `record` uses checked counter additions which can panic (`:98–118`). Field counters also mutate before arithmetic. A value theorem may abstract these effects; total compiled execution equivalence must address them explicitly.

The existing `monic-remainder-zero-test/REPORT.md` accurately marks residual decoding/canonicality and compiled equivalence as unresolved. This audit supports the source decomposition above; it is **not** an independent Lean proof replay, compiled-equivalence certificate, or performance result. No files changed and no scientific jobs launched.
