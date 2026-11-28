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
#include <map>
#include <set>
#include <utility>
using Key=std::pair<U,U>;
Key key(F f){return {f.u,f.v};}
F frob(F f){return {sub(f.u,f.v),f.v?ell-f.v:0};}
int main(){try {
 need(pw(124,65,131)==130,"131 inert in Q(sqrt(-7))");
 need(pw(256,131,263)==1,"263 split in Q(sqrt(-7))");
 std::map<U,std::vector<std::string>> small;
 std::vector<U> ar;for(U x=0;x<263;x++)if((x*x+x+2)%263==0)ar.push_back(x);
 need(ar.size()==2,"two small roots");U w=2;while(pw(w,131,263)!=1)w++;
 need(w!=1&&pw(w,131,263)==1,"small z order");
 for(std::size_t i=0;i<2;i++)for(U j=1;j<131;j++)small[mul(ar[i],pw(w,j,263),263)].push_back(std::string(i?"b":"a")+std::to_string(j));
 std::size_t singles=0,doubles=0;for(const auto&[value,labels]:small){need(labels.size()<=2,"small at most double");if(labels.size()==1)singles++;else doubles++;std::cout<<"263:"<<value;for(const auto&s:labels)std::cout<<" "<<s;std::cout<<"\n";}
 need(singles==2&&doubles==129&&small.size()==131,"small counts");
 std::cout<<"SUMMARY263 single="<<singles<<" double="<<doubles<<" index_exponent="<<doubles<<"\n";
 F a{0,1},b{ell-1,ell-1},one{1,0};F z=power(times(b,times(b,F{(ell+1)/2,0})),66);
 need(power(z,131)==one&&!(z==one),"large z order");
 std::map<Key,std::vector<std::string>> large;std::set<Key> roots;
 for(U j=1;j<131;j++){F zj=power(z,j);roots.insert(key(zj));large[key(times(a,zj))].push_back("a"+std::to_string(j));large[key(times(b,zj))].push_back("b"+std::to_string(j));}
 need(roots.size()==130,"distinct large roots");
 std::size_t single_orbits=0,double_orbits=0,real_double=0,real_single=0,total=0;std::set<Key> seen;
 for(const auto&[value,labels]:large){need(labels.size()<=2,"large at most double");total+=labels.size();Key conjugate=key(frob(F{value.first,value.second}));need(large.count(conjugate)&&large.at(conjugate).size()==labels.size(),"Frobenius orbit");
 std::cout<<"ell:"<<value.first<<","<<value.second;for(const auto&s:labels)std::cout<<" "<<s;std::cout<<"\n";
 if(!seen.insert(value).second) { continue; }
 seen.insert(conjugate);
 if(value==conjugate){if(labels.size()==2)real_double++;else real_single++;}else if(labels.size()==2)double_orbits++;else single_orbits++;
 }
 need(total==260&&real_double==1&&real_single==0&&double_orbits==64&&single_orbits==1,"large orbit counts");
 std::cout<<"SUMMARYell real_double="<<real_double<<" quadratic_double="<<double_orbits<<" quadratic_single="<<single_orbits<<" index_exponent="<<real_double+2*double_orbits<<"\n";
 std::cout<<"PASS residue collision census; local order identification remains a written proof\n";return 0;
 } catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
