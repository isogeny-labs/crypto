# M31 division-free Horner resultant norm [pcc:unverified]

Semaev-only continuation, no L change. Local native C++17 development diagnostic, p=2147483647, y^2=x^3-3x+21. No new paid compute, production F4, or independent PCC. Prior art: existing VERONESE and TWO_PLANE findings, previous pair-power resultant norm, Kosters–Yeo arXiv:1503.08001 and Kousidis–Wiemers arXiv:1906.05594 (different Weil-descent setting). No priority claim.

## Exact circuit

For F(z)=sum c_i z^i of fixed degree D and g=d z²+e z+f, define H_0=c_D, H_k=zH_(k-1)+c_(D-k). For roots r1,r2 of g and d!=0 define W_k=d^k, T_k=d^k(H_k(r1)+H_k(r2)), U_k=d^(k+1)(r1 H_k(r1)+r2 H_k(r2)), N_k=d^k H_k(r1)H_k(r2). Start W=1,T=2c_D,U=-e c_D,N=c_D². At each step v=c_(D-k), set

    W'=d W
    T'=U+2W'v
    U'=-e U-df T-e W'v
    N'=f N+v U+W'v².

Then N_D=Res(g,F), by Vieta and the quadratic root equation. This is a universal polynomial identity, so leading-degree drops including d=0 are covered. Sign is unchanged reversing the resultant. All state equations have degree <=3; the S3 coefficient definitions have degree <=4. Interpolation of intermediate coefficient vectors on fixed nodes is linear. The unrolled circuit is a definitional graph extension with a unique state for each original input, and N_D=0 eliminates exactly to the original Semaev ideal. Explicit interpolation circuit size O(4^m) is still exponential; no bounded global first-fall or solving-degree implication. Under auxiliary-first lex elimination the high-degree original relation reappears.

## Local diagnostic

C++17 g++ 14.2.0, Linux x86_64 Xeon Platinum 8573C, single unisolated process. Seed 20261123; holdout 20261124. 600 generic fixed-degree Sylvester/norm comparisons (100 each D=2,4,8,16,32,64), including leading drops and zero quadratics, matched exactly. Mutation c=(1,2,3), g=(5,7,11) ascending coefficients: determinant 19 vs altered c_0=0 result 235. No equivalence counterexample found. 50 random affine lines per m=3,4,5 with fixed target and repeated/constant-coordinate cases: all three coefficient vectors and complete rational-root polynomials gcd(F,t^p-t) equal, 150/150 line gates. 600 off-grid evaluations also agree. Roots summed across lines: 60,61,62; no all-field line. Not an unspecialized multivariate solution-set gate.

|m|max monolithic degree|Sylvester generation muls|prior pair-norm gen muls|Horner gen muls|common gcd muls|
|---:|---:|---:|---:|---:|---:|
|3|12|223555|74250|68400|933954|
|4|32|3483020|1036400|904400|5122448|
|5|80|27283370|8424950|6420200|29311300|

Local scalar multiplication counts per call: old 2D²+9D+2 versus new 8D+3, giving 1.47x at D=2 and 17.03x at D=64. Complete line generation ratios vs old norm 1.086x,1.146x,1.312x; including identical gcd 1.006x,1.022x,1.056x. These are field multiplication counts, not production F4 or isolated wall speed. GCD costs unchanged. No solving-degree measurement. Main goal open.

Exact local source SHA256 d3ad43cc2326f60f64ace9d9354312c420eff64bdec05d16331da33d4d93ddcb; raw bench.csv ed4c7a6b67c86af471d586ce14a46552c781b00ac11aca6bd35d30b56317c94b; norm_counts.csv 10f3d7e404aed768c0115fefb1db9dd7a0ea317ef1242f167d0d55fc45719a41; root_certificates.txt 8e5e6c75ab1f9ca94a2a9800dc7289415d4e2848039e37ca46ffa6724111761e; run.log 257e8b883a23145f356ee5504680bc81285da099f70f360da684b9077f86480a. Full harness and raw files are conversation artifacts, not falsely claimed present in repository. Native C++ fallback because Sage, msolve, FGb, Singular and Rust unavailable. KB/conductor/EKS unavailable; keep draft. Next: instrumented graded F4 comparison on M31 with fixed original auxiliary relations, compact symmetry versus actual Horner DAG. Preserve the 50-case gate and charge extraction. No graph-IC or factor-base diversion.
