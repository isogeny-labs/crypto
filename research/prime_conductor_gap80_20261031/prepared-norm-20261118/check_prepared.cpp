// Bounded native unit oracle. Direct root sums do not use trace weights.
#define main conductor_pipeline_fixture_entry
#include "../native/complete_pipeline.cpp"
#undef main
static Z value_at(const Poly& poly,const Z& root){
 Z value=0,coefficient;
 for(slong i=fmpz_mod_poly_length(poly.z,ctx);i-->0;){
  fmpz_get_mpz(coefficient.get_mpz_t(),poly.z->coeffs+i);
  value=mod(value*root+coefficient);
 }
 return value;
}
static std::array<Z,3> direct(const Jet& jet,unsigned long n){
 std::array<Z,3> sum{};
 for(unsigned long i=1;i<=n;i++){
  Z inverse_zero=inverse(value_at(jet[0],Z(i)));
  Z u=mod(value_at(jet[1],Z(i))*inverse_zero);
  Z v=mod(value_at(jet[2],Z(i))*inverse_zero);
  Z w=mod(value_at(jet[3],Z(i))*inverse_zero);
  sum[0]=mod(sum[0]+u);
  sum[1]=mod(sum[1]+v-u*u*inverse(2));
  sum[2]=mod(sum[2]+w-u*v+u*u*u*inverse(3));
 }
 return sum;
}
int main(){try{
 unsigned long positive=0,collisions=0,construct_rejections=0,prepared=0;
 for(const char* field:{"101","78125000378223625457769953627","4794168233664648458988367110514316682522236917173341621535751"}){
  p=Z(field);fmpz_t prime;fmpz_init(prime);fmpz_set_mpz(prime,p.get_mpz_t());fmpz_mod_ctx_init(ctx,prime);
  {
   for(unsigned long n=1;n<=32;n++){
    Poly h=constant(1),x;fmpz_mod_poly_set_coeff_ui(x.z,1,1,ctx);
    for(unsigned long i=1;i<=n;i++)h=h*(x-constant(i));
    PreparedNorm cache(h);prepared++;
    for(unsigned long nonce=0;nonce<4;nonce++){
     Jet jet;jet[0]=x+constant(nonce+1);
     for(unsigned long k=1;k<4;k++)for(unsigned long i=0;i<2*n+3;i++)
      fmpz_mod_poly_set_coeff_ui(jet[k].z,i,(i+1)*(nonce+3)*(k+2)+17*(n+1),ctx);
     const auto expected=direct(jet,n);
     const auto legacy=norm(h,jet),first=cache.evaluate(jet);
     require(legacy&&*legacy==expected&&first&&*first==expected,"prepared/direct-root mismatch");positive++;
     Jet bad=jet;bad[0]=h;
     require(!cache.evaluate(bad)&&!norm(h,bad),"collision accepted");collisions++;
     auto after=cache.evaluate(jet);require(after&&*after==expected,"cache changed after collision");positive++;
    }
   }
   for(Poly invalid:{constant(1),scale(constant(1)+[](){Poly x;fmpz_mod_poly_set_coeff_ui(x.z,1,1,ctx);return x;}(),2)}){
    bool rejected=false;try{PreparedNorm bad(invalid);}catch(const std::runtime_error& e){
     require(std::string(e.what())=="prepared norm requires monic nonconstant modulus","unexpected prepared rejection");rejected=true;
    }
    require(rejected,"invalid prepared modulus accepted");construct_rejections++;
   }
  }
  fmpz_mod_ctx_clear(ctx);fmpz_clear(prime);
 }
 std::cout<<"{\"scope\":\"bounded native direct-root/cache controls only; no timing\",\"fields\":3,\"modulus_preparations\":"<<prepared<<",\"positive_prepared_evaluations\":"<<positive<<",\"collision_controls\":"<<collisions<<",\"invalid_modulus_controls\":"<<construct_rejections<<",\"includes_202_and_96_bit_fields\":true,\"full_target_quotient\":null,\"runtime_measurements\":null}"<<std::endl;
 flint_cleanup();return 0;
}catch(const std::exception& e){std::cerr<<"prepared-check: "<<e.what()<<'\n';return 2;}}
