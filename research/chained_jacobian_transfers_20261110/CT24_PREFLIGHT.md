# CT24 fixed-field preflight protocol

Prospective bounded development test, before any new Jacobian transfer is
accepted. It runs independently of CT23 complete guard07 and leaves those
consumer/native/engine inputs unchanged. No timing or paid job is admitted.

Question: do the actual E1'=(26,58),E2'=(69,50) have unique nondual degree7
directions that preserve the accepted Frobenius5/6 kernel choices, and
what HLP model results from the actual transported2-torsion matching?

Reuse unchanged consumer seven.rs (published994869034, SHA256
bb640465bfe9406e5b5e275b5127c73417b48cec5a90006d2f2bb87f72f40ae9)
for its exact Kohel construction/curve/global dual-x certificates. The copied
fixture wrapper is not a new arithmetic engine. Existing isogeny_algos
field/poly/curve/Kohel/HLP engines remain unchanged. The new test uses
fixed F101^6 modulus[33,46,2,18,56,12,1], fixed F101^3 modulus[56,13,6,1]
and seed0x241017, no fitted selection.

For each source, construct both kernels. Identify the old dual by exact
kernel polynomial equality and independently observe its Frobenius action.
Require that it has eigen2/4 and that the unique other kernel has eigen5/6.
Check all7 kernel points/closure/Frobenius and actual zero images, global
curve and both dual-x identities via the reused constructor, and full109/113
rational-point bijection and dual[7]/recovery. Print new kernels, target
models/j-invariants and exact transported2-roots.

Transport the original accepted paired2-roots through old alpha and new
beta; verify exact point validity/order/Frobenius/addition, then call the
unchanged HLP gluer. Require coefficientwise prime-field descent, smooth
even sextic, product L-polynomial and actual order12317. Record raw model,
leading-square status and normalized constant-square status. A nonsquare
or unsupported chart is retained as a boundary, without retwisting or
substituting a different kernel. A new j-equality is evidence to assess,
not permission to choose another fitted candidate.

Acceptance here covers only elliptic maps and the proposed HLP model.
Actual quotient charts, new full Jacobian Phi/Psi/global quotient/degree/
polarization/dual/descent/distinction and source/cost controls remain CT24's
next obligations. Read CT24_PLAN.md after the preflight result.

Sources checked (KB): unavailable MCP/fallback127; existing seven.rs,
seven_glue.rs and accepted global proof, primary HLP §3.2 Propositions3–4
(field characteristic not2, Galois-equivariant2-gluing not induced by a
geometric elliptic isomorphism), and Kunzweiler §3.2. Actual order/trace/j
checks and smoothness must validate the specific fixture; no novelty claim.
[pcc:unverified]
