# Frozen development check, 2026-10-30

Pure exact-arithmetic correctness check, not a performance/scaling experiment.
Before execution freeze the source hash. Use C++20/GMP over rational numbers.

1. Construct all eight Cauchy puncture polynomials W_delta(T) for delta in
   [-4,3], using C(T)=product_{q=-4}^3(T-q), normalization C'(0).
2. Independently check W_delta(0)=1[delta=0], the full 8-slot parity-code
   restricted product kernel has identity origin, C'(0)=-144,
   C'(1)=240, W_1(1)=-5/3, [z]W_2(1+8z)=40/3.
3. The physical 2-axis slots x=(0,0), y=(2,1) both satisfy even total
   binary popcount, have equal axis-1 selected low bits, and axis-2 difference1.
   Check the passive coefficient is (-5/3)*(40/3)=-200/9, not zero.
4. For n=2,...,9 construct T_{j,k}=1[k>j] overF2 and a cyclic shift P;
   independently eliminate the rows of T(P-I), expecting rank n-1.
5. Fail on every mismatch. Report finite cases and exact values only;
   CPU timing is neither measured nor used to infer any asymptotic result.

Prospective extension, before executing version003:
6. For L=4,8,16,32 construct the even-popcount code and the actual normalized
   first-axis passive derivative matrix at matched odd nodes1. Compute the
   rational source derivative ratios rho=C'(1)/C'(0); check all retained
   entries rho^2/(1-delta) have odd numerator and denominator. Construct the
   residue matrix overF2 and independently eliminate it. Expected rank is
   2(L-1), from the proved decomposition into that many disjoint nonempty
   all-ones blocks. No other a values or alternative codes are searched.

The finite check supplements the symbolic proof; it does not implement a
full deformation recursion, give a benchmark, or close all common kernels.
