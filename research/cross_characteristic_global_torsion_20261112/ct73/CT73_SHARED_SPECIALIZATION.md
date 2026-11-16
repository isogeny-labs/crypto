# CT73 — A common specialization matters; CT72's mixed trace is not a prerequisite

**Scientific standing: [pcc:unverified].** A scoped correction and fully enumerated negative control. This is not a uniform source-to-binary transfer or DLP speedup.

## The precise theorem and correction

Let \(E/K\) be an elliptic curve over a number field, \(\ell\) an odd prime, and \(v,w\) places of good reduction with residue characteristics prime to \(\ell\). Fix algebraic closures and **one coherent choice of residue embeddings**. Prime-to-characteristic finite-étale specialization gives geometric group isomorphisms

\[
s_v:E[\ell](\overline K)\cong\widetilde E_v[\ell](\overline{k_v}),
\qquad
s_w:E[\ell](\overline K)\cong\widetilde E_w[\ell](\overline{k_w}).
\]

The composite \(\Phi_{v,w}=s_w\circ s_v^{-1}\) therefore satisfies

\[
\boxed{\Phi_{v,w}([x]P)=[x]\Phi_{v,w}(P)}
\]

for every \(P\) and scalar \(x\). A *common* change of target residue embedding composes this map with a single group automorphism, so scalar relations still hold. In particular, knowing the numeric CT72 relative-Frobenius double-coset label \(u\) is **not itself required** to preserve a DLP relation. CT72's exact classification of separate local conjugacy classes remains correct. This result refines its interpretation: a proposed *implementation* may choose to compute \(u\), but the underlying transfer only requires that both encoded inputs use the **same** global torsion labeling and residue specialization. Neither \(s_v^{-1}\) nor its efficient evaluation from arbitrary input coordinates has been implemented here. [pcc:unverified]

**Independent-orientation bound (conditional).** Suppose instead the two input vectors are relabeled by *independently uniform* actions from a surjective \(\mathrm{GL}_2(\mathbb F_\ell)\) torsion image. If \(P\ne0\) and \(x\ne0\), the probability of still preserving the specified relation is exactly

\[
\Pr\bigl(g_Q([x]P)=[x]g_P(P)\bigr)=\frac{1}{\ell^2-1}.
\]

Proof: \(\mathrm{GL}_2(\mathbb F_\ell)\) acts transitively on the \(\ell^2-1\) nonzero torsion vectors. Conditional on \(g_P(P)\), the independently random \(g_Q([x]P)\) is uniform on that orbit; exactly one value works. When \(g_Q=g_P\), linearity preserves all scalars. **This is an error frequency in this specified random-orientation model, not an ECDLP complexity lower bound.** Reducible or non-surjective Galois images require separate counting. [pcc:unverified]

## Concrete CT69 two-characteristic witness

Reuse CT69's fixed 11a1 elliptic curve \(y^2+y=x^3-x^2-10x-20\), \(\ell=3\), source \(P=(18,5)\in E(\mathbb F_{29})\), target \(U=(19,2)\) on its reduction \(y^2+y=x^3+x^2\) over

\[
\mathbb F_{256}=\mathbb F_2[Y]/(Y^8+Y^6+Y^5+Y^4+Y^3+Y+1).
\]

The degree-eight CT69 global point field is **inert at 2**: the eight choices below are residue-field embeddings into the fixed byte-encoded \(\mathbb F_{256}\), not eight different primes of that field. Let \(U_i=\mathrm{Frob}_2^i(U)\) for \(0\le i<8\). They are eight distinct nonzero 3-torsion vectors with \(U_{i+4}=-U_i\). Source input \(Q=-P=[2]P\) must consistently go to \(-U_i\). If the input routine instead chooses \(i\) for the generator and \(j\) independently for the target, then the 64 ordered cases yield:

| Relation of orientations | Pairs | Result |
|---|---:|---|
| \(j=i\) | 8 | Correct logarithm 2 |
| \(j=i+4\pmod8\) | 8 | Wrong logarithm 1 |
| Other six offsets | 48 | Target outside \(\langle U_i\rangle\) |

Thus eight shared embeddings all preserve the original logarithm, but 56 of 64 independent selections fail in one of two distinguishable ways. The identity target is deliberately excluded from the mismatch test because all group homomorphisms send zero to zero. [pcc:unverified]

## Independently executable audit

The Python checker uses bitwise shift/reduction arithmetic for \(\mathbb F_{256}\); the independently written JS checker uses carryless polynomial convolution with descending modulus reduction. Both independently check the binary curve equation, Frobenius orbit, all 64 choices, and **every** \(\mathrm{GL}_2(\mathbb F_\ell)\) matrix/scalar for \(\ell=3,5,7,11\). The larger-prime tests are abstract finite linear groups, not reductions of the 11a1 curve. [pcc:unverified]

| \(\ell\) | GL2 size | Nonzero vectors | Correct / total independent pairs |
|---:|---:|---:|---:|
| 3 | 48 | 8 | 8 / 64 |
| 5 | 480 | 24 | 24 / 576 |
| 7 | 2016 | 48 | 48 / 2304 |
| 11 | 13200 | 120 | 120 / 14400 |

Reproduce from repository root:

~~~bash
D=research/cross_characteristic_global_torsion_20261112/ct73
python3 -B "$D/ct73_common_place.py" > ct73-py.jsonl
python3 -O -B "$D/ct73_common_place.py" > ct73-pyopt.jsonl
node "$D/ct73_common_place_independent.js" > ct73-js.jsonl
node --no-opt "$D/ct73_common_place_independent.js" > ct73-jsopt.jsonl
cmp ct73-py.jsonl ct73-pyopt.jsonl
cmp ct73-py.jsonl ct73-js.jsonl
cmp ct73-py.jsonl ct73-jsopt.jsonl
echo '717eca797dcbe13c8b0bd9719fc916813766b7025c38a5af1fd4cc7252cdfc88  ct73-py.jsonl' | sha256sum -c
~~~

Producer-local execution passed all four modes byte-for-byte; scoped GitHub CI and independent PCC scientific signing must be checked separately. No paid EKS/GPU compute or full cold-cost benchmark was executed. [pcc:unverified]

## Sources, precise remaining obligation and negative scope

Prior repository evidence reviewed: CT69 explicit point field, CT70 two-prime factor cycles, CT71 stabilizer, CT72 double cosets, root [PR #3877](https://github.com/isogeny-labs/crypto/pull/3877), cross-characteristic [PR #4015](https://github.com/isogeny-labs/crypto/pull/4015) and related cryptanalysis/crypto-autoresearcher code and PR searches. Primary classical result: [Snowden, *Elliptic curves over DVRs*, torsion specialization](https://websites.umich.edu/asnowden/teaching/2013/679/L08.html). The result follows from standard group-scheme specialization and elementary orbit counting; no mathematical novelty claim. Research-KB MCP and its checked-out fallback were unavailable in the runtime; the prior-art review is necessarily incomplete. [pcc:unverified]

**Next discriminating experiment:** design an algorithm that takes both arbitrary encoded \(P,Q\) to globally compatible torsion representatives and then uses **one** target residue embedding, without searching the unknown scalar. Measure the cost of global-field construction, two input lifts, matching roots/embeddings, target solve and recovery before comparing with matched negation-map rho and an admitted index-calculus baseline. Determining CT72's mixed trace \(u\) independently is optional unless an actual proposed algorithm requires it. This CT73 result proves neither a cryptographic-scale lift nor an advantage, and leaves the broader cross-characteristic DLP question open. [pcc:unverified]
