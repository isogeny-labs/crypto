# CT223 — genuine quadratic prime-divisor norm and projector audit

**2026-11-19 UTC; scoped public finite-field test. [pcc:unverified].** Continuation of [CT222](CT222_PROJECTOR_DIVISOR_AUDIT.md) on [PR #4715](https://github.com/isogeny-labs/crypto/pull/4715).

## New question

CT222 checked nonsplit *reduced supports* only after constructing divisor classes as sums of rational points. Does the same `109`-primary projector identity hold on *genuine degree-2 closed prime divisors*, whose individual support points do not exist over `F101`?

On `C/F101: y²=u⁹+u³+46` and `E/F101: y²=x³+x+46`, with `π(u,y)=(u³,y)`, test

`[80,597,544]D = π*([73] π_*D)`,

where `D=P+Frob(P)-2∞` and `P=(u,y)∈C(F101²)\C(F101)`. Here `π_*D = π(P)+Frob(π(P))-2O` is computed by actual extension-field elliptic addition, **not by summing preselected rational supports**.

## Exact executable result

[ct223_extension_norm_oracle.py](ct223_extension_norm_oracle.py) is an independent, dependency-free implementation of:

- `F101²=F101[z]/(z²-2)` with explicit field inversion, powering, Frobenius, elliptic addition and trace;
- deterministic enumeration of extension-field points and Frobenius orbits;
- degree-2 monic Mumford pairs `U(X)=Norm(X-u)`, `V(u)=y` via conjugate interpolation, with `U\mid(V²-f)`; and
- separate genus-4 Cantor composition, reduction, scalar multiplication, and comparison against the extension-field elliptic trace.

**Producer-local results, all exact arithmetic [pcc:unverified]:**

| Check | Local result |
| --- | ---: |
| Genuine irreducible degree-2 Mumford prime-divisor inputs | 12 |
| Exact divisor equality of scalar and traced-norm projectors | 12 / 12 |
| Full Jacobian-order annihilation `[115593846]D=0` | 12 / 12 |
| Nonzero projected 109-primary classes | 12 / 12 |
| Wrong-sign norm mutation rejected | 12 / 12 |
| Mixed classes (two degree-2 primes and one rational point) | 4 / 4 |
| Geometric branch-pair elliptic trace at `u=0` | 0 (checked) |

The branch pair `(0,±√46)` lies over `F101²`, is exchanged by Frobenius and by the hyperelliptic involution, and has principal rational trace as a Jacobian divisor. Hence it is **not** a nonzero ramified-divisor test: nonreduced local-parameter/ramification behavior is still untested.

The computation uses the exact CT221 cardinalities as **inputs**, not a fresh point-count proof. CT223 shares no Cantor code with CT222's Node.js implementation, but both programs and test selection are produced by the same research session and are **not** an independently signed PCC validation.

## Reproducibility and negative controls

```sh
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct223_extension_norm_oracle.py --self-test
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct223_extension_norm_oracle.py > /tmp/ct223.json
cmp /tmp/ct223.json research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct223_f101_extension_norm.json
sha256sum /tmp/ct223.json
# Deliberately fails closed because Python -O removes assertion checks:
python3 -O research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct223_extension_norm_oracle.py --self-test
```

Local development on 2026-11-19: normal self-test `CT223_SELFTEST_PASS 12 4 12`; normal JSON `cmp` passed; optimized-mode run exited with `CT223 requires assertions: Python -O prohibited`. Frozen witness JSON SHA-256: `aa6b13caaceb581922271736e066d33c3ade329f52908406e1dded17776ddfbb`. Bounded CI in `.github/workflows/ct205-toy-transfer.yml` separately exercises the exact output and expected fail-closed control. Hosted exact-head CI verdict must be checked after publication.

## Interpretation and next technical gate

This closes one concrete CT222 coverage gap: **norm/pullback agreement for genuine quadratic prime classes and four mixed-support classes** on this toy field. It does not implement general nonsplit/ramified norm arithmetic in Rust, price physical `C169/C13/D`, solve a growing-field ECDLP faster, or determine composite-order IC performance. Each of those remains open.

Priority remains the original authenticated 695/701-record norm-point reuse fixture and matched same-host complete-cold direct rho/BSGS/IC versus transferred native composite-order IC. Previous records document S3 HTTP 403 on the original-model source and an EKS budget STOP; neither was bypassed. The next unblocked native task is to connect the verified 109-primary projector to composite-order genus-4 relation collection and recovery, including explicit offset accounting, matrix rank, target verification, memory, and full cold charges. Stop or switch arms if the candidate is slower.

## Prior work and coordination

Reviewed CT217–CT222; primary concurrent [crypto #4693](https://github.com/isogeny-labs/crypto/pull/4693) (prime-order genus-4 IC, not a split-J transfer), [crypto #4697](https://github.com/isogeny-labs/crypto/pull/4697) (transfer infrastructure), original [#3877](https://github.com/isogeny-labs/crypto/pull/3877), and [cryptanalysis #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523). Methods: Cantor arithmetic, finite-field Frobenius trace and Albanese/Jacobian norm functoriality; **no novelty claim**. Shared KB MCP and its repository wrapper were not available from this connector runtime; these source files were the fallback, not an exhaustive prior-art search. No paid compute or protected merge performed.
