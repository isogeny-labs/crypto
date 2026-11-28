#include <gmpxx.h>
#include <iostream>
#include <stdexcept>

// Mathematical witness construction only. Nothing printed here is trusted by
// the formal proof; literal witnesses must be rechecked by the Lean kernel.
mpz_class mod(mpz_class a,const mpz_class& p) {a%=p;if(a<0)a+=p;return a;}
mpz_class inverse(const mpz_class& a,const mpz_class& p) {
  mpz_class r;if(!mpz_invert(r.get_mpz_t(),a.get_mpz_t(),p.get_mpz_t()))
    throw std::runtime_error("nonunit");
  return r;
}
int main() {
  const mpz_class m("257743850762632419871495"),j=-32768;
  const mpz_class p=11*m*(m+1)+3;
  const mpz_class a=mod(-3*j*inverse(mod(j-1728,p),p),p);
  const mpz_class b=mod(2*j*inverse(mod(j-1728,p),p),p);
  const mpz_class printedA("25706413842211054102700238164133538302169176474");
  const mpz_class px=25,py("37304648684346883938862473354554031475866783037");
  const mpz_class qx=20,qy("157931136836524102701922129702410179003466984543");
  std::cout<<"p="<<p<<"\na_from_formula="<<a<<"\nb_from_formula="<<b
    <<"\nP_residual_printed="<<mod(py*py-px*px*px-printedA*px-b,p)
    <<"\nQ_residual_printed="<<mod(qy*qy-qx*qx*qx-printedA*qx-b,p)
    <<"\nP_residual_formula="<<mod(py*py-px*px*px-a*px-b,p)
    <<"\nQ_residual_formula="<<mod(qy*qy-qx*qx*qx-a*qx-b,p)
    <<"\nj_printed="<<mod(1728*4*printedA*printedA*printedA*
      inverse(mod(4*printedA*printedA*printedA+27*b*b,p),p),p)
    <<"\nj_formula="<<mod(1728*4*a*a*a*
      inverse(mod(4*a*a*a+27*b*b,p),p),p)
    <<"\nj_target="<<mod(j,p)<<"\n";
}
