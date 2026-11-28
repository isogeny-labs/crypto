#include "/usr/include/gmpxx.h"
#include <algorithm>
#include <cassert>
#include <chrono>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>
using I=mpz_class;
struct Field {
 I p;
 I mod(I x)const{x%=p;if(x<0)x+=p;return x;}
 I mul(const I&a,const I&b)const{return mod(a*b);}
 I power(I a,I e)const{I r=1;a=mod(a);while(e>0){if((e&1)!=0)r=mul(r,a);e>>=1;if(e>0)a=mul(a,a);}return r;}
 I inv(const I&a)const{if(mod(a)==0)throw std::runtime_error("inverse zero");return power(a,p-2);}
 bool sqrt_ts(I a,I&root)const{
  a=mod(a);if(a==0){root=0;return true;}
  if(power(a,(p-1)/2)!=1)return false;
  I q=p-1;unsigned s=0;while((q&1)==0){q>>=1;++s;}
  I z=2;while(power(z,(p-1)/2)!=p-1)++z;
  I c=power(z,q),r=power(a,(q+1)/2),t=power(a,q);unsigned m=s;
  while(t!=1){I v=t;unsigned k=0;while(v!=1&&k<m){v=mul(v,v);++k;}if(k==m)throw std::runtime_error("TS invariant");I b=c;for(unsigned j=0;j<m-k-1;++j)b=mul(b,b);r=mul(r,b);c=mul(b,b);t=mul(t,c);m=k;}
  if(mul(r,r)!=a)throw std::runtime_error("sqrt membership");root=r;return true;
 }
 bool sqrt_cipolla(I n,I&root)const{
  n=mod(n);if(n==0){root=0;return true;}I leg=power(n,(p-1)/2);if(leg!=1)return false;
  I a=0,w;unsigned tried=0;for(;;){w=mod(a*a-n);I test=power(w,(p-1)/2);if(test==p-1)break;if(++tried>4096)throw std::runtime_error("Cipolla parameter cap");++a;}
  struct Pair{I real,imag;};auto product=[&](const Pair&x,const Pair&y)->Pair{return {mod(x.real*y.real+x.imag*y.imag*w),mod(x.real*y.imag+x.imag*y.real)};};
  Pair r{1,0},b{a,1};I e=(p+1)/2;while(e>0){if((e&1)!=0)r=product(r,b);e>>=1;if(e>0)b=product(b,b);}if(r.imag!=0||mul(r.real,r.real)!=n)throw std::runtime_error("Cipolla membership");root=r.real;return true;
 }
};
