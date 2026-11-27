#include <gmpxx.h>
#include <fstream>
#include <array>
#include <iostream>
#include <stdexcept>
#include <vector>
using Z=mpz_class; using Poly=std::vector<Z>;
void need(bool b,const char*s){if(!b)throw std::runtime_error(s);}
Z pow2(unsigned n){Z z=1;return z<<n;}
Z choose(unsigned n,unsigned k){Z z;mpz_bin_uiui(z.get_mpz_t(),n,k);return z;}
Z ev(const Poly&p,int t){Z z=0;for(auto i=p.rbegin();i!=p.rend();++i)z=z*t+*i;return z;}
Z norm_eval(const Poly&p,int t){Z a=0,b=0;for(auto i=p.rbegin();i!=p.rend();++i){Z nexta=-2*b+*i;Z nextb=a+t*b;a=nexta;b=nextb;}return a*a+t*a*b+2*b*b;}
int main(int argc,char**argv){try{
 need(argc==2||argc==3,"output TSV path, optional GP field file required");
 std::ofstream out(argv[1]);need(bool(out),"TSV output");out<<"t\tm\tN_m\tclosed_places\n";
 std::array<unsigned,5> models{};
 for(unsigned mask=0;mask<32;mask++){
  unsigned a1=mask&1,a2=(mask>>1)&1,a3=(mask>>2)&1,a4=(mask>>3)&1,a6=(mask>>4)&1;
  unsigned b8=(a1*a6+a1*a3*a4+a2*a3+a4)&1;
  unsigned disc=(a1*b8+a3*(1+a1))&1;if(!disc)continue;
  int count=1;for(unsigned x=0;x<2;x++)for(unsigned y=0;y<2;y++)if(((y+a1*x*y+a3*y+x+a2*x+a4*x+a6)&1)==0)count++;
  int t=3-count;need(t>=-2&&t<=2,"elliptic Hasse bound");
  if(models[t+2]++==0)std::cout<<"elliptic_t="<<t<<" fixture_a1_a2_a3_a4_a6="<<a1<<a2<<a3<<a4<<a6<<"\n";
 }
 for(unsigned n:models)need(n>0,"all five elliptic classes occur");
 std::vector<Z>s(263);s[0]=2;s[1]=-1;for(unsigned m=2;m<s.size();m++)s[m]=-s[m-1]-2*s[m-2];
 Poly dividend(263),p(261),h(131);dividend[262]=1;dividend[131]=-s[131];dividend[0]=pow2(131);
 for(int d=262;d>=2;--d){Z q=dividend[d];p[d-2]=q;dividend[d]-=q;dividend[d-1]-=q;dividend[d-2]-=2*q;}
 need(dividend[0]==0&&dividend[1]==0,"exact Weil quotient");
 Poly rem=p;
 for(int k=130;k>=0;--k){h[k]=rem[130+k];for(int i=0;i<=k;i++)rem[130+k-2*i]-=h[k]*choose(k,i)*pow2(i);}
 for(const auto&x:rem) { need(x==0,"real Weil conversion"); }
 need(h[130]==1,"monic real polynomial");
 Z u0=0,u1=1;for(int m=2;m<=131;m++){Z u=-u1-2*u0;u0=u1;u1=u;}
 need(ev(h,-1)==131*u1,"parent U identity");
 std::cout<<"s131="<<s[131]<<"\nh_A(-1)=131*U131 confirmed\n";
 for(int t=-2;t<=2;t++){
  Z r=ev(h,t);if(r<0)r=-r;need(norm_eval(p,t)==pow2(130)*r*r,"independent quadratic norm resultant");
  std::vector<Z>b(263),ts(263);ts[0]=2;ts[1]=t;for(unsigned m=2;m<ts.size();m++)ts[m]=t*ts[m-1]-2*ts[m-2];
  unsigned failure=0;
  for(unsigned m=1;m<263;m++){Z atrace=(m%131==0)?Z(130*s[m]):Z(-s[m]);Z n=pow2(m)+1-atrace-ts[m];Z numerator=n;for(unsigned d=1;d<m;d++)if(m%d==0)numerator-=d*b[d];need(numerator%m==0,"integral place count");b[m]=numerator/m;if(b[m]<0&&failure==0)failure=m;out<<t<<'\t'<<m<<'\t'<<n<<'\t'<<b[m]<<'\n';}
  std::cout<<"t="<<t<<" reduced_resultant="<<r<<" first_negative_place="<<failure<<" N1="<<2-t<<" N2="<<6-t*t<<"\n";
 }
 std::cout<<"PASS exact arithmetic; zero failure means only necessary place conditions through262\n";
 if(argc==3){
  std::ofstream field(argv[2]);need(bool(field),"field output");field<<"H=Polrev([";
  for(unsigned i=0;i<h.size();i++){if(i)field<<",";field<<h[i];}field<<"]);\n";
  Poly derivative(130);for(unsigned i=1;i<h.size();i++)derivative[i-1]=i*h[i];
  Z residue=ev(derivative,-1)%263;if(residue<0)residue+=263;
  need(residue!=0,"simple prime ideal root at263");
  need(395*395-917*13*13==4*263,"quadratic positive norm certificate");
  std::cout<<"h_A_prime(-1)_mod263="<<residue<<"\n";
  std::cout<<"positive quadratic norm: (395+13*sqrt(917))/2 has norm263\n";
 }
 return 0;
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
