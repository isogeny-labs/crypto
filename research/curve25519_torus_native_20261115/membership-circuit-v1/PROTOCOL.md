# Compact many-coset membership interface: pre-implementation contract

Question: does the lifted quadratic system describe the same fixed raw
membership set without an O(k) selector table, and can it expose a valid
nonidentity cofactor-image point for later target-conditioned decomposition?
Circuit size is not a decomposition-work or exponent result.

## Public inputs and semantic contract

Use a prime field Fp with nonsquare2, the Fp2 model sqrt2, a norm-one h of
exact order a*q, and 0<2k<a. These are external instance contracts. Actual
values are the already-used h, a60824497, q213156431, k20633, p2^255-19.
The constructor receives only public parameters, not the control's j,t or
curve witness. Compiler output is independent of any planted decomposition.

Let w be the bit length of k-1. Boolean b_i and d_i encode x and d in
[0,2^w). Require p>2^(w+1), and impose

    b_i(b_i-1)=d_i(d_i-1)=0,
    sum_i 2^i(b_i+d_i)=k-1.

Then x+d=k-1 as integers, so 0<=x<k and j=x+1. The p bound matters:
x31,d3,k18,p17 gives a field equality but violates the intended range.
Do not hide an integer comparison in a field operation.

Compute z^q with Fp2 squaring/multiplication gates. Compute the selected
power as omega*product_i(1+b_i(omega^(2^i)-1)), omega=h^q. Equate both
coordinates; keep the norm-one constraint. Every multiplication output
gets its own defining quadratic equations. The earlier primitive-root
membership characterization then identifies exactly S={h^(j+a*t)} under
the external field/order contracts. The high univariate degree remains
compatible with a small lifted circuit; neither gives its solving cost.

## Optional elliptic image

For public A486662, add a Boolean branch c, raw affine point (u,v), and
inverse of z.a. Set u=-A/2+(1-2c)*A/(2*z.a) and impose the curve equation
v²=u³+A*u²+u, with a named u² auxiliary. Over the actual Fp, the earlier
Elligator branch contract supplies one rational u branch and both signs v.
That property is not a theorem about arbitrary constructor parameters.

Apply three affine doubling gates using inverse(2v) at each step. Each
gate uses lambda=(3u²+2Au+1)/(2v), U=lambda²-A-2u,
V=lambda*(u-U)-v. The output is [8](u,v) and is affine/nonidentity.
Retaining nonidentity cofactor images is explicit: any original point with
[8]P nonidentity has all three denominators nonzero, so these guards do
not remove another retained point. Poles/identity images must be excluded
or recorded, never converted into a successful witness.

This is a conditional semantic argument using existing field/group laws.
Native controls do not formalize the compiler, the actual Fp2 instance,
the map-degree/energy argument or normalized signed-point indexing. No
target-sum oracle is implemented by a single-member circuit.

## Controls, acceptance and resources

- Fixed Fp59 arithmetic-only fixture h=(54,37), a5/q3/k2. Validate h's
  exact order by h^15=1, h^3!=1, h^5!=1. Enumerate its six expected raw
  members directly; compare circuit projection for all3481 field-pair
  candidates and both valid coset selectors. This is a finite correctness
  check, not a curve/IC measurement or parameter search.
- Actual fixed j in{1,floor(k/2),k}, t in{0,1,q-1}, with both raw v signs.
  Check all equations and compare the output with the existing curve API's
  [8]P. Preserve a pole/identity failure if one occurs; do not change inputs.
- Check malformed range parameters, an actual overflow selector and
  nonbinary bits. Change every variable of one valid actual assignment
  and require rejection, recording any unexpected accepted mutation.
- Canonical field values and exact witness length are required. Reject
  extra/omitted values; native witness generation is outside the compiler.
- Preserve source/binary pre-call hashes, exact inputs, raw output and
  failures. Limit this step to these development controls, focused Rust
  test/fmt/Clippy and new Lean numeric endpoints. No solver invocation,
  wall-time comparison, sampling, EKS spend or broad root-library run.

Acceptance: all stated finite checks pass with complete output, and the
conditional projection/domain contracts remain explicit. A counterexample
rejects this encoding or implementation. A small circuit does not satisfy
the prior solver/yield or exponent criteria. Preserve ROUTE_CRITERIA_V3.

Sources checked (KB unavailable): PKM2016 section3.3; existing torus-s4
quadratic polynomial and torus-s4-tree Fp2/curve APIs; frozen many-coset
report and primitive-root endpoints; Mathlib ZMod.val_natCast_of_lt and
Nat.cast_add. No new general probability or group law is proposed.
