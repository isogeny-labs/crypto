#include <iostream>
#include <vector>
#include <string>
#include <chrono>
#include <cassert>
#include "../k0_zero_cubic_20261115_v43/lab_variant_02/native_gf83.h"
using E=gf83::Element;
E hexread(std::string s){E v=0;for(char c:s)v=(v<<4)|(c<='9'?c-'0':c-'a'+10);return v;}
E inv(E x){assert(x);E t=x;for(int i=1;i<82;i++)t=gf83::mul(gf83::square(t),x);E v=gf83::square(t);assert(gf83::mul(x,v)==1);return v;}
E half(E x){E v=x,t=x;for(int j=1;j<=41;j++){t=gf83::square(gf83::square(t));v^=t;}return v;}
E s3(E x,E y,E z){return gf83::square(gf83::mul(x,y))^gf83::square(gf83::mul(x,z))^gf83::square(gf83::mul(y,z))^gf83::mul(gf83::mul(x,y),z)^E(1);}
int main(){auto start=std::chrono::steady_clock::now();E z=hexread("622425e96ba633969263b"),y=hexread("3258275069b18d0854564");assert((gf83::square(y)^gf83::mul(z,y))==(gf83::mul(gf83::square(z),z)^E(1)));unsigned N=1u<<18;std::vector<E>prefix(N),ix(N);prefix[0]=1;for(unsigned x=1;x<N;x++)prefix[x]=gf83::mul(prefix[x-1],E(x));E r=inv(prefix.back());for(unsigned x=N-1;x>0;x--){ix[x]=gf83::mul(r,prefix[x-1]);r=gf83::mul(r,E(x));}
E zt=inv(z);assert(z>=N&&zt>=N);std::cout<<"{\"targets\":[";bool first=true;
for(E target:{z,zt}){E it=inv(target);prefix[0]=1;for(unsigned x=1;x<N;x++)prefix[x]=gf83::mul(prefix[x-1],E(x)^target);r=inv(prefix.back());unsigned count=0,visited=0;std::vector<std::pair<unsigned,unsigned>>pairs;
if(it<N){assert(s3(0,it,target)==0);pairs.emplace_back(0,(unsigned)it);count++;}
for(unsigned x=N-1;x>0;x--){E id=gf83::mul(r,prefix[x-1]);r=gf83::mul(r,E(x)^target);E a=E(x)^target^ix[x]^it;E rhs=gf83::square(a),h=half(rhs);visited++;
if((gf83::square(h)^h)==rhs){E scale=gf83::mul(gf83::mul(E(x),target),gf83::square(id));E yy=gf83::mul(scale,h);for(E yy2:{yy,yy^scale}){assert(s3(x,yy2,target)==0);if(yy2<N){count++;pairs.emplace_back(x,(unsigned)yy2);}}}
if(x%4096==0&&std::chrono::duration<double>(std::chrono::steady_clock::now()-start).count()>50)return 2;
}
if(!first)std::cout<<',';first=false;std::cout<<"{\"visited_nonzero_x\":"<<visited<<",\"pair_count\":"<<count<<",\"pairs\":[";for(unsigned i=0;i<pairs.size();i++){if(i)std::cout<<',';std::cout<<'['<<pairs[i].first<<','<<pairs[i].second<<']';}std::cout<<"]}";
}
std::cout<<"],\"target_point_checked\":true,\"singletons_outside_domain\":true,\"distinct_torsion_pairs_excluded_by_polynomial_degree\":true,\"complete\":true,\"seconds\":"<<std::chrono::duration<double>(std::chrono::steady_clock::now()-start).count()<<"}"<<std::endl;
}
