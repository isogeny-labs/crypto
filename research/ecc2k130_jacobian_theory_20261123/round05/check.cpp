#include <gmpxx.h>
#include <array>
#include <iostream>
#include <stdexcept>
#include <vector>
using Z=mpz_class;
static void need(bool b,const char*s){if(!b)throw std::runtime_error(s);}
static Z p2(unsigned n){Z z=1;return z<<n;}
static Z rem2(Z a,const Z&b){
  need(b>0,"nonzero binary divisor");
  while(a!=0 && mpz_sizeinbase(a.get_mpz_t(),2)>=mpz_sizeinbase(b.get_mpz_t(),2)){
    auto k=mpz_sizeinbase(a.get_mpz_t(),2)-mpz_sizeinbase(b.get_mpz_t(),2);
    Z t=b<<k; a^=t;
  } return a;
}
static Z sq2(const Z&a){Z b=0;for(unsigned i=0;i<mpz_sizeinbase(a.get_mpz_t(),2);++i)if(mpz_tstbit(a.get_mpz_t(),i))mpz_setbit(b.get_mpz_t(),2*i);return b;}
static Z gcd2(Z a,Z b){while(b!=0){Z c=rem2(a,b);a=b;b=c;}return a;}
static unsigned remsmall(unsigned a,unsigned b){
  while(a!=0 && (31-__builtin_clz(a))>=(31-__builtin_clz(b)))a^=b<<(__builtin_clz(b)-__builtin_clz(a));
  return a;
}
static bool irredsmall(unsigned f,unsigned n){
  for(unsigned d=1;d<=n/2;++d)for(unsigned g=(1U<<d);g<(2U<<d);++g)if(remsmall(f,g)==0)return false;
  return true;
}
struct FF {
  unsigned n,q,mod;
  explicit FF(unsigned degree):n(degree),q(1U<<n),mod(q){while(!irredsmall(mod,n))++mod;need(mod<2*q,"small irreducible exists");}
  unsigned mul(unsigned a,unsigned b)const{unsigned c=0;while(b){if(b&1)c^=a;b>>=1;a<<=1;if(a&q)a^=mod;}return c;}
  unsigned pow(unsigned a,unsigned e)const{unsigned r=1;while(e){if(e&1)r=mul(r,a);a=mul(a,a);e>>=1;}return r;}
  unsigned inv(unsigned a)const{need(a!=0,"nonzero inverse");return pow(a,q-2);}
  unsigned tr(unsigned a)const{unsigned t=0;for(unsigned j=0;j<n;++j){t^=a;a=mul(a,a);}need(t<2,"absolute trace in F2");return t;}
};
int main(){try{
  std::vector<Z>s(132);s[0]=2;s[1]=-1;for(unsigned m=2;m<s.size();++m)s[m]=-s[m-1]-2*s[m-2];
  std::vector<Z>v(263),P(261);v[262]=1;v[131]=-s[131];v[0]=p2(131);
  for(int d=262;d>=2;--d){Z a=v[d];P[d-2]=a;v[d]-=a;v[d-1]-=a;v[d-2]-=2*a;}
  need(v[0]==0&&v[1]==0,"exact quotient");
  unsigned f=0;for(unsigned i=1;i<=130;++i)if(mpz_odd_p(P[260-i].get_mpz_t()))f=i;
  need(f==130,"A ordinary by coefficient parity");
  auto countA=[&](unsigned m){return Z(p2(m)+1-((m%131==0)?Z(130*s[m]):Z(-s[m])));};
  need(countA(1)==2&&countA(2)==2&&countA(3)==14,"A small counts");
  need(mpz_odd_p(countA(131).get_mpz_t()),"A degree131 parity");
  std::cout<<"A p_rank="<<f<<"; N1="<<countA(1)<<"; N2="<<countA(2)<<"; N3="<<countA(3)<<"; N131="<<countA(131)<<"\n";
  need(countA(2)<3,"hyperelliptic degree2 fiber contradiction");
  std::cout<<"genus130_hyperelliptic=fails N2>=3\n";
  for(int t=-2;t<=2;++t){
    std::vector<Z>b(132);b[0]=2;b[1]=t;for(unsigned m=2;m<b.size();++m)b[m]=t*b[m-1]-2*b[m-2];
    auto N=[&](unsigned m){return Z(countA(m)-b[m]);};
    std::vector<Z>C(263);for(unsigned j=0;j<P.size();++j){C[j]+=2*P[j];C[j+1]-=t*P[j];C[j+2]+=P[j];}
    unsigned rank=0;for(unsigned i=1;i<=131;++i)if(mpz_odd_p(C[262-i].get_mpz_t()))rank=i;
    need(rank==((t%2==0)?130U:131U),"carrier p rank");
    need(mpz_odd_p(N(1).get_mpz_t())!=mpz_odd_p(N(131).get_mpz_t()),"forced odd count of131 branch cycles");
    std::cout<<"genus131 t="<<t<<"; p_rank="<<rank<<"; branch_points="<<rank+1<<"; N1="<<N(1)<<"; N2="<<N(2)<<"; N3="<<N(3)<<"; N131_parity="<<mpz_odd_p(N(131).get_mpz_t())<<"; ";
    if(t==2||t==-2){need(N(2)<3,"fiber bound for even trace2");std::cout<<"fails N2>=3";}
    else if(t==1){need(N(3)>18,"upper hyperelliptic bound");std::cout<<"fails N3<=18";}
    else if(t==0){need((131+1)%131!=0,"single branch orbit genus divisibility");std::cout<<"fails 131 divides genus+1";}
    else std::cout<<"survives these necessary tests; branch_cycles=131+1";
    std::cout<<'\n';
  }
  Z Q=p2(131)+p2(13)+p2(2)+p2(1)+1;Z x=2,z=x;
  for(unsigned j=2;j*j<=131;++j)need(131%j!=0,"131 prime");
  for(unsigned j=0;j<131;++j)z=rem2(sq2(z),Q);
  need(z==x&&gcd2(Q,sq2(x)^x)==1,"prime degree131 irreducibility criterion");
  std::cout<<"fixture_Q_irreducible=pass; Q="<<Q.get_str(16)<<"\n";
  unsigned first=0;
  for(unsigned m=1;m<=8;++m){
    FF F(m);Z e=p2(m)+1-s[m];
    unsigned directE=1,traceE=2,curve=1,directC=1;
    for(unsigned a=0;a<F.q;++a){
      for(unsigned y=0;y<F.q;++y)if((F.mul(y,y)^F.mul(a,y))==(F.pow(a,3)^1U))++directE;
      if(a!=0)traceE+=F.tr(a^F.inv(a))?0:2;
      unsigned qa=F.pow(a,131)^F.pow(a,13)^F.pow(a,2)^a^1U;
      need(qa!=0,"Q has no roots in degrees below131");
      unsigned rhs=a^F.inv(qa);curve+=F.tr(rhs)?0:2;
      for(unsigned y=0;y<F.q;++y)if((F.mul(y,y)^y)==rhs)++directC;
    }
    need(Z(directE)==e&&directE==traceE,"independent E0 controls");
    need(curve==directC,"trace versus direct quadratic count");
    Z target=p2(m)+1; if(Z(curve)!=target&&first==0)first=m;
    std::cout<<"fixture degree="<<m<<"; small_field_modulus="<<F.mod<<"; E0_controls=pass; C_count="<<curve<<"; required="<<target<<"; direct_AS=pass\n";
  }
  Z presentations=2*(p2(131)-2)*(p2(131)-1)/131;
  need((2*(p2(131)-2)*(p2(131)-1))%131==0,"integral presentation count");
  std::cout<<"fixture_first_mismatch="<<first<<"; normalized_presentations="<<presentations<<"\n";
  return 0;
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
