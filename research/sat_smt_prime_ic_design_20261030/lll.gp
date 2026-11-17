\\ CPU time of PARI qflll on linearised S3 lattices. Proxy, not the full Coppersmith lattice.
\\ dim 9: monomials x1^i x2^j, 0<=i,j<=2, one congruence sum c*y = 0 mod p, weights Ymax/Y_ij.
\\ dim 49: monomials 0<=i,j<=6, modulus p^e (e=1 and 3) to mimic level-3 entry size.
build(dd, bound, modulus, B)={
  my(nn = (dd+1)^2, c = vector(nn, i, random(modulus)), degs = vector(nn), W = vector(nn), Y, Ymax, M, ci);
  while(gcd(c[1], modulus)!=1, c[1]=random(modulus));
  for(i=0,dd, for(j=0,dd, degs[i*(dd+1)+j+1]=i+j));
  Y = vector(nn, k, B^degs[k]); Ymax = vecmax(Y);
  W = vector(nn, k, max(1, floor(Ymax/Y[k])));
  ci = lift(Mod(c[1], modulus)^-1);
  \\ columns: modulus*e_1, and e_k - (c_k*ci mod modulus) e_1 for k>=2, then scale row k by W[k]
  M = matrix(nn, nn);
  M[1,1] = modulus*W[1];
  for(k=2, nn, M[1,k] = -lift(Mod(c[k]*ci, modulus))*W[1]; M[k,k] = W[k]);
  M;
}
bench(dd, nbits, e, reps)={
  my(p = nextprime(2^(nbits-1)+random(2^(nbits-1))), B = ceil(p^(1/18)), modulus = p^e, t, tot=0, M, R);
  my(Ms=vector(reps, r, build(dd, 0, modulus, B))); gettime(); for(r=1, reps, R = qflll(Ms[r])); tot = gettime();
  printf("dim=%d bits(p)=%d modulus=p^%d B=2^%.1f  qflll cpu = %.4f ms/call (reps=%d)\n", (dd+1)^2, nbits, e, log(B)/log(2), tot/reps*1.0, reps);
}
setrand(12345);
foreach([60,100,160,256], nb, bench(2,nb,1,3000));
foreach([60,100,160], nb, bench(6,nb,1,5));
foreach([60,100,160], nb, bench(6,nb,3,3));
