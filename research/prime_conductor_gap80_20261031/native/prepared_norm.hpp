// Typed version of optimized.c's established log-norm formula.
// The caller owns a fixed monic h and the validated prime-field ctx throughout.
// No text encoding, new arithmetic identity, or performance claim.
struct PreparedNorm {
 const Poly& h;
 Poly hi,weights,gcd,gi,u,v,w,u2,u3,uv,c2,c3;
 Z half,third;
 explicit PreparedNorm(const Poly& modulus):h(modulus),half(inverse(2)),third(inverse(3)){
  slong n=fmpz_mod_poly_degree(h.z,ctx);
  require(n>=1&&fmpz_is_one(fmpz_mod_poly_lead(h.z,ctx)),"prepared norm requires monic nonconstant modulus");
  Poly rev,dh;
  fmpz_mod_poly_reverse(rev.z,h.z,n+1,ctx);
  fmpz_mod_poly_inv_series(hi.z,rev.z,n,ctx);
  fmpz_mod_poly_derivative(dh.z,h.z,ctx);
  fmpz_mod_poly_reverse(rev.z,dh.z,n,ctx);
  fmpz_mod_poly_mullow(weights.z,rev.z,hi.z,n,ctx);
 }
 PreparedNorm(const PreparedNorm&)=delete;
 PreparedNorm&operator=(const PreparedNorm&)=delete;
 std::optional<std::array<Z,3>> evaluate(const Jet& jet){
  // Preserve the existing collision gate and distinguish it from API failure.
  fmpz_mod_poly_gcd(gcd.z,h.z,jet[0].z,ctx);
  if(fmpz_mod_poly_degree(gcd.z,ctx)>0)return {};
  require(fmpz_mod_poly_invmod(gi.z,jet[0].z,h.z,ctx),"prepared norm inverse failed");
  fmpz_mod_poly_mulmod(u.z,jet[1].z,gi.z,h.z,ctx);
  fmpz_mod_poly_mulmod(v.z,jet[2].z,gi.z,h.z,ctx);
  fmpz_mod_poly_mulmod(w.z,jet[3].z,gi.z,h.z,ctx);
  fmpz_mod_poly_mulmod_preinv(u2.z,u.z,u.z,h.z,hi.z,ctx);
  fmpz_mod_poly_mulmod_preinv(u3.z,u2.z,u.z,h.z,hi.z,ctx);
  fmpz_mod_poly_mulmod_preinv(uv.z,u.z,v.z,h.z,hi.z,ctx);
  c2=v-scale(u2,half);
  c3=w-uv+scale(u3,third);
  const Poly* polys[3]={&u,&c2,&c3};
  std::array<Z,3> result;
  fmpz_t sum;fmpz_init(sum);
  for(int k=0;k<3;k++){
   fmpz_zero(sum);
   slong count=std::min(fmpz_mod_poly_length(polys[k]->z,ctx),fmpz_mod_poly_length(weights.z,ctx));
   for(slong i=0;i<count;i++)fmpz_addmul(sum,polys[k]->z->coeffs+i,weights.z->coeffs+i);
   fmpz_get_mpz(result[k].get_mpz_t(),sum);result[k]=mod(result[k]);
  }
  fmpz_clear(sum);return result;
 }
};
