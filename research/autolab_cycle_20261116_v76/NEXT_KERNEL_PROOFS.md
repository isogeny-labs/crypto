# Two exact kernel hypotheses

## Binary semilinear Frobenius jumps
For K[X]/H, sigma(a)=a^2 is a ring endomorphism even when H has repeated roots. Let P_k=X^(2^k) mod H. Applying sigma^k to a polynomial uses coefficient Frobenius a_i -> a_i^(2^k) followed by composition with P_k modulo H. Precompute coefficient-field Frobenius maps from the declared basis; charge setup and memory.

For T_k=sum_(i=0)^(k-1) sigma^i(t), combine lengths by P_(a+b)=sigma^a(P_b) and T_(a+b)=T_a+sigma^a(T_b). Binary powering of this pair computes the full trace with logarithmically many compositions. It trades linear quotient squarings for higher-cost compositions and is not automatically faster. Test degree3/5/12, odd/even binary extensions, repeated H and zero-root strata. Compare every coefficient with sequential trace. Measure cold setup and amortization over a frozen batch; refute if total cost does not improve.

## Sparse-prime modular powers
For any quotient-ring base a, r_k=a^(2^k-1) satisfies r_(2k)=r_k*(r_k)^(2^k) and r_(k+1)=r_k^2*a. Using k squarings in the first recurrence, following the binary expansion keeps exactly k-1 squarings while reducing dense-exponent multiplications to O(log k). For P521, p=2^521-1 and Euler exponent=(p-1)/2=2^520-1 fit directly without inversion, including nonunit bases. This is an operation-count prediction, not a measurement.

Other NIST primes have sparse signed power-of-two forms. Negative terms require a unit base. First compute gcd(base,H): split a nontrivial factor, handle a zero root explicitly, or use the generic fallback. Never assume quotient elements invertible. The existing linear-factor splitter already performs this preliminary gcd. Prove each integer exponent and compare full roots against frozen generic powering, including zero, repeated and characteristic-three controls.

Freeze distinct source directories, inputs, bases, seeds and budgets before either test. Use <=90-second one-core4GiB components, then a separate cold panel only after exactness and total-cost screening pass. Strong native/FLINT univariate comparators and genuine full F4/F5/F6 are separate comparisons. Classical addition chains and modular composition do not by themselves establish novelty.
