#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <string>
using U=unsigned __int128;
U readhex(const std::string&s){U v=0;for(char c:s){unsigned d=c<='9'?c-'0':c-'a'+10;assert(d<16);v=(v<<4)|d;}return v;}
int main(){std::set<std::pair<U,U>> all,empty;std::map<int,unsigned> total,certified;std::string ps,cs,ls;int f,k;unsigned n=0;while(std::cin>>ps>>f>>k>>cs>>ls){assert(f>=0&&f<3&&k>=0&&k<=8);U p=readhex(ps),c=readhex(cs),lam=readhex(ls);total[k]++;if(lam)certified[k]++;n++;for(unsigned i=0;i<(1u<<k);i++){U s=c;for(int j=0;j<k;j++)if((i>>j)&1)s^=U(1)<<(j+(f==0?0:f==1?1:65));all.insert({p,s});if(lam)empty.insert({p,s});}}assert(n==1800);std::cout<<"{\"configurations\":"<<n<<",\"distinct_p_s\":"<<all.size()<<",\"distinct_certified_empty_p_s\":"<<empty.size()<<",\"by_dimension\":[";bool first=true;for(auto row:total){if(!first)std::cout<<",";first=false;std::cout<<"{\"dimension\":"<<row.first<<",\"tested\":"<<row.second<<",\"certified\":"<<certified[row.first]<<"}";}std::cout<<"]}\n";}
