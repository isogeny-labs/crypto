// Exact global closure and CM certificate checks; no point sampling substitutes
// for the rational-function identity checked here.
static Poly poly_power(Field& f, Poly a, unsigned e) {
 Poly result={Big(1)};
 for(;e;e>>=1){if(e&1)result=times(f,result,a);if(e>1)a=times(f,a,a);}
 return result;
}
static Poly homogeneous(Field& f,const Poly& a,const Poly& n,const Poly& d) {
 Poly result;
 if(a.empty())return result;
 for(size_t i=0;i<a.size();++i)
  result=plus(f,result,scale(f,times(f,poly_power(f,n,unsigned(i)),poly_power(f,d,unsigned(a.size()-1-i))),a[i]));
 return result;
}
struct DivisionPolynomials {
 Field& f; Poly cubic; std::map<unsigned,Poly> memo;
 DivisionPolynomials(Field& field,const Big& a,const Big& b):f(field),cubic({b,a,Big(),Big(1)}) {
  memo[0]={};memo[1]={Big(1)};memo[2]={Big(1)};
  Big a2=f.mul(a,a),b2=f.mul(b,b);
  memo[3]={f.sub(Big(),a2),f.mul(Big(12),b),f.mul(Big(6),a),Big(),Big(3)};
  memo[4]=scale(f,Poly{f.sub(f.sub(Big(),f.mul(Big(8),b2)),f.mul(a2,a)),f.sub(Big(),f.mul(Big(4),f.mul(a,b))),f.sub(Big(),f.mul(Big(5),a2)),f.mul(Big(20),b),f.mul(Big(5),a),Big(),Big(1)},Big(2));
 }
 // f_m = psi_m for odd m, psi_m/(2y) for even m.
 Poly get(unsigned n) {
  if(memo.count(n))return memo.at(n);
  unsigned m=n/2;Poly result;
  if(n&1){
   Poly left=times(f,get(m+2),poly_power(f,get(m),3));
   Poly right=times(f,get(m-1),poly_power(f,get(m+1),3));
   Poly y4=scale(f,times(f,cubic,cubic),Big(16));
   if(m%2==0)left=times(f,left,y4);else right=times(f,right,y4);
   result=plus(f,left,right,true);
  }else{
   result=times(f,get(m),plus(f,times(f,get(m+2),poly_power(f,get(m-1),2)),times(f,get(m-2),poly_power(f,get(m+1),2)),true));
  }
  memo[n]=result;return result;
 }
};
static std::pair<Poly,Poly> poly_divmod(Field& f,Poly a,const Poly& b) {
 require(!b.empty(),"polynomial division by zero");Poly q(a.size()>=b.size()?a.size()-b.size()+1:0);Big inverse=f.inv(b.back());
 while(a.size()>=b.size()){
  size_t offset=a.size()-b.size();Big c=f.mul(a.back(),inverse);q[offset]=c;
  for(size_t j=0;j<b.size();++j)a[offset+j]=f.sub(a[offset+j],f.mul(c,b[j]));trim(a);
 }trim(q);return {q,a};
}
struct KernelRing {
 Field& f; Poly kernel;
 Poly mod(const Poly& a){return poly_divmod(f,a,kernel).second;}
 Poly mul(const Poly& a,const Poly& b){return mod(times(f,a,b));}
 Poly power(Poly a,const Big& e){Poly out={Big(1)};for(int i=BN_num_bits(e.p)-1;i>=0;--i){out=mul(out,out);if(BN_is_bit_set(e.p,i))out=mul(out,a);}return out;}
 Poly inverse(Poly a){
  Poly r0=kernel,r1=mod(a),s0,s1={Big(1)};
  while(!r1.empty()){auto qr=poly_divmod(f,r0,r1);auto s2=plus(f,s0,times(f,qr.first,s1),true);r0=r1;r1=qr.second;s0=s1;s1=s2;}
  require(r0.size()==1,"kernel-ring denominator is not a unit");return mod(scale(f,s0,f.inv(r0[0])));
 }
};
static void verify_kernel_orientation(Runtime& r,DivisionPolynomials& d) {
 Field& f=r.field;const auto& edge=r.f.closure.at("first").at("edge");
 Poly kernel=coeffs(edge.at("kernel_polynomial"));
 require(kernel.size()==size_t((r.forward->degree+1)/2)&&kernel.back()==Big(1),"kernel degree and monic");
 require(times(f,kernel,kernel)==r.forward->D,"map denominator is kernel square");
 KernelRing ring{f,kernel};ring.inverse(deriv(f,kernel));
 require(ring.mod(d.get(unsigned(r.forward->degree))).empty(),"kernel divides odd division polynomial");
 int rho=edge.at("frobenius_eigenvalue").integer();
 require(rho>0&&rho<r.forward->degree&&rho==r.f.marked.at("orientation_root").integer(),"orientation certificate agrees with edge");
 Poly denominator=poly_power(f,d.get(unsigned(rho)),2);
 Poly term=times(f,d.get(unsigned(rho-1)),d.get(unsigned(rho+1)));
 if(rho&1)term=scale(f,times(f,d.cubic,term),Big(4));else denominator=scale(f,times(f,d.cubic,denominator),Big(4));
 Poly numerator=plus(f,times(f,Poly{Big(),Big(1)},denominator),term,true);
 Poly derivative=plus(f,times(f,deriv(f,numerator),denominator),times(f,numerator,deriv(f,denominator)),true);
 Poly scalar_x=ring.mul(numerator,ring.inverse(denominator));
 Poly scalar_y=ring.mul(derivative,ring.inverse(scale(f,times(f,denominator,denominator),Big(rho))));
 Big half=f.p;require(BN_sub_word(half.p,1)&&BN_rshift1(half.p,half.p),"Frobenius exponent");
 require(ring.power(Poly{Big(),Big(1)},f.p)==scalar_x,"signed kernel Frobenius x action");
 require(ring.power(d.cubic,half)==scalar_y,"signed kernel Frobenius y action");
}
static void verify_relation(Runtime& r) {
 const auto& z=r.f.closure; const auto& m=r.f.marked; Field& f=r.field;
 Big a=f.mod(Big(r.f.curve.at("a").str())),b(r.f.curve.at("b").str());
 require(r.forward->degree==r.back->degree,"equal dual degrees");
 unsigned ell=unsigned(r.forward->degree);require(ell>=3&&(ell&1)&&ell<=31,"bounded odd degree");
 require(r.forward->N.back()==Big(1)&&r.forward->D.back()==Big(1)&&r.back->N.back()==Big(1)&&r.back->D.back()==Big(1),"normalized leading terms");
 Big l{long(ell)};Big l2=f.mul(l,l),l3=f.mul(l2,l);
 require(f.mul(r.xscale,l2)==Big(1)&&f.mul(r.yscale,l3)==Big(1),"positive differential closing scales");
 require(r.back->target_a==f.mul(a,f.mul(l2,l2))&&r.back->target_b==f.mul(b,f.mul(l3,l3)),"global endpoint model");
 DivisionPolynomials division(f,a,b);
 verify_kernel_orientation(r,division);
 Poly sd=poly_power(f,division.get(ell),2);
 Poly sn=plus(f,times(f,Poly{Big(),Big(1)},sd),scale(f,times(f,division.cubic,times(f,division.get(ell-1),division.get(ell+1))),Big(4)),true);
 Poly cn=scale(f,homogeneous(f,r.back->N,r.forward->N,r.forward->D),r.xscale);
 Poly cd=times(f,r.forward->D,homogeneous(f,r.back->D,r.forward->N,r.forward->D));
 require(times(f,cn,sd)==times(f,sn,cd),"full global dual-composition identity");
 Big aa(m.at("alpha_a").str()),bb(m.at("alpha_b").str()),trace(z.at("trace").str()),rho(m.at("orientation_root").str());
 Big scalar,tmp,norm,a2,ab,b2;
 require(BN_add(scalar.p,aa.p,bb.p)&&BN_nnmod(scalar.p,scalar.p,r.n.p,f.ctx),"CM subgroup label arithmetic");
 require(scalar==r.mu,"CM label a+b modulo n");
 require(BN_mul(a2.p,aa.p,aa.p,f.ctx)&&BN_mul(ab.p,aa.p,bb.p,f.ctx)&&BN_mul(ab.p,ab.p,trace.p,f.ctx)&&BN_mul(b2.p,bb.p,bb.p,f.ctx)&&BN_mul(b2.p,b2.p,f.p.p,f.ctx)&&BN_add(norm.p,a2.p,ab.p)&&BN_add(norm.p,norm.p,b2.p),"CM norm arithmetic");
 require(norm==Big(m.at("norm").str()),"CM norm certificate");
 Big quotient,remainder;
 require(BN_div(quotient.p,remainder.p,norm.p,l.p,f.ctx)&&remainder.zero()&&quotient==Big(m.at("residual_degree").str()),"CM quotient degree");
 require(BN_mul(tmp.p,bb.p,rho.p,f.ctx)&&BN_add(tmp.p,tmp.p,aa.p)&&BN_nnmod(tmp.p,tmp.p,l.p,f.ctx)&&tmp.zero(),"selected eigenline annihilated");
 Big opposite(z.at("opposite_root").str());
 require(BN_mul(tmp.p,bb.p,opposite.p,f.ctx)&&BN_add(tmp.p,tmp.p,aa.p)&&BN_nnmod(tmp.p,tmp.p,l.p,f.ctx)&&!tmp.zero(),"opposite eigenline retained");
 require(f.mod(aa)==Big(m.at("differential").str()),"CM differential");
}
