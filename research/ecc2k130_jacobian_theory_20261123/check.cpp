// C++17 exact arithmetic certificate replay. No timings or curve search.
#include <algorithm>
#include <array>
#include <cstdint>
#include <iostream>
#include <numeric>
#include <stdexcept>
#include <string>
#include <vector>
using U=std::uint64_t; using W=__uint128_t; using I=__int128_t;
constexpr U ell=146505763881528721ULL;
void need(bool b,const char* s){if(!b)throw std::runtime_error(s);}
U mul(U a,U b,U m){return U(W(a)*b%m);}
U pw(U a,U b,U m){U r=1;for(;b;b>>=1,a=mul(a,a,m))if(b&1)r=mul(r,a,m);return r;}
bool trial(U n){if(n<2)return false;for(U d=2;d<=n/d;d++)if(n%d==0)return false;return true;}
std::string dec(I x){if(!x)return "0";bool n=x<0;if(n)x=-x;std::string s;while(x){s+=char('0'+x%10);x/=10;}if(n)s+='-';std::reverse(s.begin(),s.end());return s;}
U sub(U a,U b){return a>=b?a-b:ell-(b-a);}
struct F{U u,v; bool operator==(const F&b)const{return u==b.u&&v==b.v;}};
F times(F a,F b){U vv=mul(a.v,b.v,ell);return {sub(mul(a.u,b.u,ell),mul(2,vv,ell)),sub(U((W(a.u)*b.v+W(a.v)*b.u)%ell),vv)};}
F power(F a,U n){F r{1,0};for(;n;n>>=1,a=times(a,a))if(n&1)r=times(r,a);return r;}
int main(){try{
 I a=0,b=1;for(int n=2;n<=131;n++){I c=-b-2*a;a=b;b=c;if(n==3||n==5||n==131)std::cout<<"U"<<n<<"="<<dec(b)<<"\n";}
 need(b==I(263)*ell,"Lucas product");
 std::vector<U> fac{2,2,2,2,3,5,17,31,4001,289510489};W product=1;for(U q:fac){need(trial(q),"small prime factor");product*=q;}need(product==ell-1,"complete n-1 factorization");
 U witness=2;for(;;witness++){if(pw(witness,ell-1,ell)!=1)continue;bool ok=true;for(U q:fac)if(std::gcd(pw(witness,(ell-1)/q,ell)+ell-1,ell)!=1)ok=false;if(ok)break;}
 // Full Lucas n-1 criterion proves ell prime, using trial-proved factors.
 std::cout<<"Lucas primality witness="<<witness<<"\n";
 need(trial(131)&&trial(263),"small primes");
 need(pw(2,130,131)==1&&pw(2,65,131)!=1&&pw(2,26,131)!=1&&pw(2,10,131)!=1,"ord_131(2)=130");
 need(ell%131==130&&ell%7==5,"prime congruences");
 need(pw(ell-7,(ell-1)/2,ell)==ell-1,"quadratic nonsquare");
 F alpha{0,1},beta{ell-1,ell-1},one{1,0};
 need(times(alpha,beta)==F{2,0},"alpha beta");need(power(alpha,ell)==beta,"quadratic Frobenius");
 F alpha_inv=times(beta,F{(ell+1)/2,0});
 F ratio=times(beta,alpha_inv);F z=power(ratio,66);
 need(power(z,131)==one&&!(z==one),"primitive 131st root");
 need(times(z,z)==ratio,"square root choice");
 F pi=times(alpha,z),barpi=times(beta,power(z,130));
 need(pi==barpi&&power(pi,ell)==pi,"F minus V vanishes");
 need(power(z,ell)==power(z,130),"cyclotomic Frobenius");
 std::cout<<"ell="<<ell<<"; ell mod131=130; ell mod7=5\n";
 std::cout<<"z="<<z.u<<" + "<<z.v<<"*alpha\npi="<<pi.u<<" + "<<pi.v<<"*alpha\n";
 std::cout<<"PASS: arithmetic witness only; geometric theorem is a written argument\n";
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
