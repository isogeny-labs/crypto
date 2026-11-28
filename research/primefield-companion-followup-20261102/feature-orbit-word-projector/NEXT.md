# Proposed next construction: endpoint-corrected trace factors

Not implemented or qualified in T1940. Preserve the zero-safe implementation
as the reference. Hypothesis: explicit corrections at the publicly known queried
offsets can remove T_b^2 from the word-product circuit.

Define K(x)=product_i (1+s_i*chi(x-a_i))/2 for queried offsets a_i=b*zeta^j.
Away from the distinct offsets A, K equals the exact raw-word indicator J.
At an offset a in A, J(a)=0 because all observed signs are nonzero, whereas K(a)
can be nonzero. Thus pointwise J=K except for finitely many public endpoints.
Initial phase restricts the sums to chi(x)=epsilon; exclude x=0 throughout.
If t is the same observed rotational stabilizer, candidate-image moments are

  count = (1/t) [sum_(chi(x)=epsilon) K(x)
                       - sum_(a in A,chi(a)=epsilon) K(a)]
  first = (1/t) [sum_(chi(x)=epsilon) x^m*K(x)
                       - sum_(a in A,chi(a)=epsilon) a^m*K(a)].

The scalar sums are m times the corresponding traces of section zero of K.
This suggests evaluating two section traces and subtracting O(B*m) scalar
corrections without materializing the endpoint delta polynomials. K(a) can be
computed from known offsets by field character powers; charge them, deduplicate
A, and include repeated factors exactly as recorded. No unknown scalar scan.
For a supplied image, membership correction must aggregate endpoint weights
with a^m=y and divide by t. Do not apply the old exact-indicator phase test to
uncorrected K, and do not assume K takes only0/1 at endpoint images.

Required checks: exact agreement with T1940 on all retained transcript/image
fixtures and public recovery; duplicate-offset and mixed-phase endpoints;
L1; malformed/cap controls; actual circuit/monomial-pair and correction counts.
This would remove squaring and some nodes, not the need to evaluate the large
word-product traces. No lower complexity or speed claim is made here.
