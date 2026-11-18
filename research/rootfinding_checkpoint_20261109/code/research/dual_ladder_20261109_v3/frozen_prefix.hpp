#include "../nist_sum_gmp_20261109_v1/gmp_field.hpp"
#include <array>
#include <fstream>
#include <sstream>
#include <map>
#include <cmath>
#include <cstdint>
using Poly=std::vector<I>;using Quad=std::array<I,3>;using Clock=std::chrono::steady_clock;
void need(bool b,const char*m){if(!b)throw std::runtime_error(m);}I fd(I a,I b){need(b!=0,"division");if(b<0){a=-a;b=-b;}I q=a/b,r=a%b;return q-(r<0?1:0);}I cd(I a,I b){return-fd(-a,b);}std::string str(const I&a){return a.get_str();}
struct Pt{I x,y;};struct Line{I A,B,C,D;};
Poly mul(const Field&f,const Poly&a,const Poly&b){Poly c(a.size()+b.size()-1);for(unsigned i=0;i<a.size();i++)for(unsigned j=0;j<b.size();j++)c[i+j]=f.mod(c[i+j]+a[i]*b[j]);return c;}
Poly divide(const Field&f,const Poly&a,I x){Poly q(a.size()-1);q.back()=a.back();for(int j=int(q.size())-2;j>=0;j--)q[j]=f.mod(a[j+1]+x*q[j+1]);need(f.mod(a[0]+x*q[0])==0,"literal exact synthetic division");return q;}
Line line(const Field&f,Pt z,Pt x,Pt y,I h){need(x.x!=y.x&&x.x!=z.x&&y.x!=z.x,"anchor denominators");auto ab=[&](Pt v){I iz=f.inv(v.x-z.x);I al=f.mod((z.y-v.y)*iz),be=f.mod(-(v.x*v.x+v.x*z.x+z.x*z.x)+h*(z.x*z.y-v.x*v.y)*iz);return std::pair<I,I>{al,be};};auto aa=ab(x),bb=ab(y);I iz=f.inv(x.x-y.x),A=f.mod((aa.first-bb.first)*iz),B=f.mod((aa.second-bb.second)*iz);return{A,B,f.mod(aa.first-x.x*A),f.mod(aa.second-x.x*B)};}
Poly residual(const Field&f,I a,I b,Pt z,Pt x,Pt y,I h,const Line&l,I k){I r=f.mod(l.A*k+l.B),s=f.mod(l.C*k+l.D-z.x*r),t=f.mod(-(z.x*z.x*z.x+r*z.x*z.x+s*z.x+(h*z.x+k)*z.y));Poly P{t,s,r,1},N=mul(f,P,P),FQ=mul(f,Poly{b,a,0,1},mul(f,Poly{k,h},Poly{k,h}));for(unsigned j=0;j<FQ.size();j++)N[j]=f.mod(N[j]-FQ[j]);auto H=divide(f,N,z.x),C=divide(f,divide(f,H,x.x),y.x);need(mul(f,mul(f,Poly{f.mod(-x.x),1},Poly{f.mod(-y.x),1}),C)==H,"all six literal norm coefficients");return C;}
I l2(I S,I M){if(S<=M)return 0;if(S<=2*M)return M*(S-M);return 2*M*S-3*M*M;}I u2(I S){return S*S/3;}I l3(I S,I M){return S<=2*M?I(0):I(M*M*(S-2*M));}I u3(I S){return(S/3)*((S+1)/3)*((S+2)/3);}I qe(const Quad&q,I S){return q[0]*S*S+q[1]*S+q[2];}
std::pair<I,I> range(const Quad&q,I L,I R){I a=qe(q,L),b=qe(q,R),lo=a<b?a:b,hi=a>b?a:b;if(q[0]!=0){I t=fd(-q[1],2*q[0]);for(I x:{t,I(t+1)})if(L<=x&&x<=R){I v=qe(q,x);if(v<lo)lo=v;if(v>hi)hi=v;}}return{lo,hi};}
uint64_t gmp_arithmetic_checks=0,gmp_root_checks=0,gmp_rounding_checks=0;
void backendcontrols(){
 for(long p:{5L,7L,11L,13L,19L}){Field f{I(p)};auto md=[&](long a){long r=a%p;return r<0?r+p:r;};
  for(long a=-p;a<=p;a++)for(long b=-p;b<=p;b++){need(f.mod(I(a))==md(a)&&f.mul(I(a),I(b))==md(a*b),"GMP signed mod/mul versus machine oracle");gmp_arithmetic_checks++;}
  for(long a=0;a<p;a++){long v=1;for(long e=0;e<=p;e++){need(f.power(I(a),I(e))==v,"GMP power versus independent linear repeated multiplication");v=md(v*a);gmp_arithmetic_checks++;}
   if(a){long inv=1;while(md(a*inv)!=1)inv++;need(f.inv(I(a))==inv,"GMP inverse independent small search");gmp_arithmetic_checks++;}
   bool exists=false;for(long y=0;y<p;y++)exists|=md(y*y)==a;I ts,ci;bool bt=f.sqrt_ts(I(a),ts),bc=f.sqrt_cipolla(I(a),ci);need(bt==exists&&bc==exists&&(!bt||f.mod(ts*ts)==a)&&(!bc||f.mod(ci*ci)==a),"GMP TS/Cipolla versus independent all roots");gmp_root_checks++;
  }
 }
 for(long a=-32;a<=32;a++)for(long b=-9;b<=9;b++)if(b){long lo=long(std::floor(double(a)/double(b))),hi=long(std::ceil(double(a)/double(b)));need(fd(I(a),I(b))==lo&&cd(I(a),I(b))==hi,"GMP signed floor/ceil versus bounded exact double quotient");gmp_rounding_checks++;}
}
void controls(){for(int M=1;M<=4;M++)for(int S=0;S<=3*M;S++){I a=100000,b=-1,c=a,d=-1;for(int x=0;x<=M;x++)for(int y=0;y<=M;y++)for(int z=0;z<=M;z++)if(x+y+z==S){I t=x*y+x*z+y*z,u=x*y*z;if(t<a)a=t;if(t>b)b=t;if(u<c)c=u;if(u>d)d=u;}need(a==l2(S,M)&&b==u2(S)&&c==l3(S,M)&&d==u3(S),"bigint independent envelope controls");}for(int a=-2;a<=2;a++)for(int b=-2;b<=2;b++)for(int cc=-2;cc<=2;cc++)for(int L=0;L<=12;L++)for(int R=L;R<=12;R++){Quad q{a,b,cc};I lo=qe(q,L),hi=lo;for(int S=L;S<=R;S++){I v=qe(q,S);if(v<lo)lo=v;if(v>hi)hi=v;}need(range(q,L,R)==std::pair<I,I>{lo,hi},"bigint independent quadratic range");}}
struct Term{I L,R;bool rejected;};struct Row{std::string name,domain;unsigned pair,hindex,nodes=0,rejected=0,frontier=0,leaves=0;I total=0,rejectmass=0,unresolved=0,excludedk=0;bool A0=false;};
