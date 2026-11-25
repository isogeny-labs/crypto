#include <array>
#include <cstdint>
#include <iostream>
#include <numeric>
#include <stdexcept>
#include <string>
#include <vector>
using U=std::uint64_t;using W=__uint128_t;
U mul(U a,U b,U m){return U(W(a)*b%m);}
U pw(U a,U n,U m){U r=1;for(;n;n>>=1,a=mul(a,a,m))if(n&1)r=mul(r,a,m);return r;}
void need(bool b,const char*s){if(!b)throw std::runtime_error(s);}
bool trial(U n){if(n<2)return false;for(U d=2;d<=n/d;d++)if(n%d==0)return false;return true;}
W decimal(const std::string&s){W n=0;for(char c:s){need(c>='0'&&c<='9',"decimal digit");n=10*n+unsigned(c-'0');}return n;}
void lucas(U n,const std::vector<U>&f){W product=1;for(U q:f){need(trial(q),"trial prime factor");product*=q;}need(product==n-1,"n-1 product");U a=2;for(;;a++){if(pw(a,n-1,n)!=1)continue;bool ok=true;for(U q:f)if(std::gcd(pw(a,(n-1)/q,n)+n-1,n)!=1)ok=false;if(ok)break;}std::cout<<"prime "<<n<<" Lucas_witness "<<a<<" factors";for(U q:f)std::cout<<" "<<q;std::cout<<" mod131 "<<n%131<<" mod7 "<<n%7<<"\n";}
int main(){try{
 need(W(523)*183691462360100297ULL==decimal("96070634814332455331"),"trace-2 factor product");
 need(W(131)*263*146505763881528721ULL==decimal("5047563083010309024613"),"trace-1 factor product");
 need(W(142248971)*156652511177ULL==decimal("22283658519494248867"),"trace0/1 factor product");
 need(W(14149)*1213355897362451ULL==decimal("17167772591781319199"),"trace2 factor product");
 for(U n:{131ULL,263ULL,523ULL,14149ULL}){need(trial(n),"small prime");std::cout<<"prime "<<n<<" trial mod131 "<<n%131<<" mod7 "<<n%7<<"\n";}
 lucas(142248971ULL,{2,5,131,108587});
 lucas(156652511177ULL,{2,2,2,7,7,131,1093,2791});
 lucas(1213355897362451ULL,{2,5,5,23,984359,1071857});
 lucas(183691462360100297ULL,{2,2,2,7,131,211,367,323356153});
 std::cout<<"ell primality certificate remains ../check.cpp; all new listed factors certified\n";
 return 0;
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
