#include <array>
#include <vector>
#include <string>
#include <iostream>
#include <chrono>
#include <sstream>
#include <sys/resource.h>
#include "../k0_zero_cubic_20261115_v43/lab_variant_02/native_gf83.h"
using E=gf83::Element;
using Poly=std::array<E,6>;
E readhex(std::string s){E x=0;for(char c:s){if(c=='x')continue;int v=c>='0'&&c<='9'?c-'0':c>='a'&&c<='f'?c-'a'+10:-1;if(v<0)throw 1;x=(x<<4)|v;}return x;}
std::string hex(E x){if(!x)return "0";std::string s;while(x){s="0123456789abcdef"[int(x&15)]+s;x>>=4;}return s;}
int top(E x){return (x>>64)?127-__builtin_clzll((unsigned long long)(x>>64)):63-__builtin_clzll((unsigned long long)x);}
E eval(const Poly&H,E x,E x2,E x3){return H[0]^gf83::mul(H[1],x)^gf83::mul(H[2],x2)^gf83::mul(H[3],x3)^gf83::mul(H[4],gf83::square(x2))^gf83::mul(H[5],gf83::mul(x2,x3));}
struct Cell{int rank=0;bool consistent=true;E part=0;std::vector<E>kernel;};
Cell solve(const Poly&base,const std::vector<Poly>&cols,unsigned x){
 E xv=x,x2=gf83::square(xv),x3=gf83::mul(xv,x2);std::array<E,83>piv{},comb{};Cell out;
 for(unsigned j=0;j<cols.size();j++){
  E v=eval(cols[j],xv,x2,x3),c=E(1)<<j;
  while(v){int p=top(v);if(piv[p]){v^=piv[p];c^=comb[p];}else{piv[p]=v;comb[p]=c;out.rank++;break;}}
  if(!v)out.kernel.push_back(c);
 }
 E rhs=eval(base,xv,x2,x3);
 while(rhs){int p=top(rhs);if(!piv[p]){out.consistent=false;break;}rhs^=piv[p];out.part^=comb[p];}
 return out;
}
void printcell(unsigned x,const Cell&c){std::cout<<"{\"x\":"<<x<<",\"rank\":"<<c.rank<<",\"consistent\":"<<(c.consistent?"true":"false")<<",\"particular\":\""<<hex(c.part)<<"\",\"basis\":[";bool first=true;for(E b:c.kernel){if(!first)std::cout<<',';first=false;std::cout<<'"'<<hex(b)<<'"';}std::cout<<"]}";}
int main(){
 auto start=std::chrono::steady_clock::now();unsigned m;std::cin>>m;if(m>83)return 2;Poly base{};std::vector<Poly>cols(m);std::string str;
 for(E&v:base){std::cin>>str;v=readhex(str);if(!gf83::canonical(v))return 3;}
 for(auto&H:cols)for(E&v:H){std::cin>>str;v=readhex(str);if(!gf83::canonical(v))return 4;}
 for(unsigned i=0;i<64;i++){E a=(E(i+1)<<70)^E(719*i+17),b=(E(172*i+1)<<50)^E(i+79);a&=gf83::mask;b&=gf83::mask;if(gf83::mul(a,b)!=gf83::mul_portable(a,b))return 5;}
 std::array<unsigned,84>hist{};unsigned seen=0,compatible=0;bool complete=true;std::string reason="";std::vector<std::pair<unsigned,Cell>>checks;std::vector<std::pair<unsigned,E>>candidates;
 for(unsigned x=1;x<(1u<<18);x++){
  Cell c=solve(base,cols,x);seen++;hist[c.rank]++;if(x%4096==1||x==(1u<<18)-1)checks.emplace_back(x,c);
  if(c.consistent){compatible++;if(c.kernel.size()>8){complete=false;reason="nullity_cap";break;}
   for(unsigned t=0;t<(1u<<c.kernel.size());t++){E v=c.part;for(unsigned j=0;j<c.kernel.size();j++)if(t>>j&1)v^=c.kernel[j];if(candidates.size()>=8192){complete=false;reason="candidate_cap";break;}candidates.emplace_back(x,v);}if(!complete)break;
  }
  if(x%256==0 && std::chrono::duration<double>(std::chrono::steady_clock::now()-start).count()>55){complete=false;reason="deadline";break;}
 }
 double seconds=std::chrono::duration<double>(std::chrono::steady_clock::now()-start).count();if(seconds>58){complete=false;reason="reporting_deadline";}
 rusage usage{};getrusage(RUSAGE_SELF,&usage);
 std::cout<<"{\"complete\":"<<(complete?"true":"false")<<",\"reason\":\""<<reason<<"\",\"visited\":"<<seen<<",\"compatible_cells\":"<<compatible<<",\"seconds\":"<<seconds<<",\"peak_rss_kib\":"<<usage.ru_maxrss<<",\"rank_histogram\":{";bool first=true;
 for(unsigned i=0;i<84;i++)if(hist[i]){if(!first)std::cout<<',';first=false;std::cout<<'"'<<i<<"\":"<<hist[i];}
 std::cout<<"},\"checkpoints\":[";first=true;for(auto&v:checks){if(!first)std::cout<<',';first=false;printcell(v.first,v.second);}std::cout<<"],\"candidates\":[";first=true;
 for(auto&v:candidates){if(!first)std::cout<<',';first=false;std::cout<<"["<<v.first<<",\""<<hex(v.second)<<"\"]";}std::cout<<"]}"<<std::endl;
}
