#include <algorithm>
#include <array>
#include <cassert>
#include <chrono>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
using U=unsigned __int128;
U hex(const std::string&s){U x=0;for(char c:s){unsigned d=c>='0'&&c<='9'?c-'0':c>='a'&&c<='f'?c-'a'+10:99;if(d>15)throw std::runtime_error("hex");if(x>>124)throw std::runtime_error("overflow");x=(x<<4)|d;}return x;}
std::string hx(U x){if(!x)return "0";std::string s;while(x){s+="0123456789abcdef"[unsigned(x&15)];x>>=4;}std::reverse(s.begin(),s.end());return s;}
int degree(U x){if(!x)return -1;int d=0;while(x>>1){x>>=1;d++;}return d;}
U polyrem(U x,U f){int n=degree(f);if(n<0)throw std::runtime_error("zero divisor");for(int i=degree(x);i>=n;i--)if((x>>i)&1)x^=f<<(i-n);return x;}
U gcdpoly(U a,U b){while(b){U r=polyrem(a,b);a=b;b=r;}return a;}
struct F{
 int n;U f,mask;std::array<std::array<U,16>,32>sqtab{};
 std::array<U,128> piv{},tr{};
 F(int nn,U ff):n(nn),f(ff),mask((U(1)<<nn)-1){
  if(n<2||n>90||degree(f)!=n||!(f&1))throw std::runtime_error("field");
  for(int i=0;i<n;i++)for(int v=0;v<16;v++)if((v>>(i%4))&1)sqtab[i/4][v]^=mul(U(1)<<i,U(1)<<i);
  for(int j=0;j<n;j++){U v=sq(U(1)<<j)^(U(1)<<j),t=U(1)<<j;while(v){int i=degree(v);if(piv[i]){v^=piv[i];t^=tr[i];}else{piv[i]=v;tr[i]=t;break;}}}
 }
 U mul(U a,U b)const{assert(!(a>>n)&&!(b>>n));U c=0;while(b){if(b&1)c^=a;b>>=1;a<<=1;if((a>>n)&1)a^=f;}return c;}
 U sq(U x)const{U y=0;int i=0;while(x){y^=sqtab[i++][unsigned(x&15)];x>>=4;}return y;}
 U inv(U x)const{if(!x)throw std::runtime_error("zero inverse");U r=1;for(int i=n-1;i>=0;i--){r=sq(r);if(i)r=mul(r,x);}assert(mul(x,r)==1);return r;}
 bool section(U rhs,U&out)const{out=0;for(int i=n-1;i>=0;i--)if((rhs>>i)&1){if(!piv[i])return false;rhs^=piv[i];out^=tr[i];}return true;}
 bool irreducible()const{
  std::vector<int>primes;int k=n;for(int q=2;q*q<=k;q++)if(k%q==0){primes.push_back(q);while(k%q==0)k/=q;}if(k>1)primes.push_back(k);
  U x=2,v=x;for(int i=1;i<=n;i++){v=sq(v);for(int q:primes)if(i==n/q&&gcdpoly(v^x,f)!=1)return false;}return v==x;
 }
 U s3(U x,U y,U z)const{U xy=mul(x,y);return sq(xy^mul(x,z)^mul(y,z))^mul(xy,z)^1;}
 std::vector<U> batch(const std::vector<U>&v)const{
  std::vector<U> pref(v.size()),out(v.size());U total=1;
  for(size_t i=0;i<v.size();i++){pref[i]=total;if(v[i])total=mul(total,v[i]);}
  U back=inv(total);
  for(size_t i=v.size();i-->0;)if(v[i]){out[i]=mul(back,pref[i]);back=mul(back,v[i]);}
  assert(back==1);return out;
 }
};
