// Independent Prime31 Dickson2 coverage census. Reference enumeration ONLY.
// g++ -std=c++17 -O3 run55_reference.cpp -o reference && ./reference [run50_adaptive4096.csv]
#include <algorithm>
#include <cassert>
#include <cstdint>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_set>
#include <vector>
using U=uint64_t;
constexpr U P=2147483647ULL,A=847514846ULL,B=2087444724ULL;
U mul(U a,U b){return a*b%P;}
U sub(U a,U b){return (a+P-b)%P;}
U power(U a,U n){U out=1;for(;n;n>>=1,a=mul(a,a))if(n&1)out=mul(out,a);return out;}
U sqrtfp(U a){U r=power(a,(P+1)/4);return mul(r,r)==a?r:P;}
struct Pt{U x,y;bool infinity=false;};
Pt add(Pt q,Pt r){
 if(q.infinity)return r;if(r.infinity)return q;
 if(q.x==r.x&&(q.y+r.y)%P==0)return{0,0,true};
 U num=q.x==r.x?(3*mul(q.x,q.x)+A)%P:sub(r.y,q.y);
 U den=q.x==r.x?(2*q.y)%P:sub(r.x,q.x);assert(den);
 U lam=mul(num,power(den,P-2));
 U x=sub(sub(mul(lam,lam),q.x),r.x);
 U y=sub(mul(lam,sub(q.x,x)),q.y);
 return{x,y,false};
}
int main(int argc,char**argv){
 assert(argc<=2);U seed=3;
 for(int k=0;k<10;k++)seed=sub(mul(seed,seed),2);
 std::vector<U> xs{seed};
 for(int k=0;k<10;k++){
  std::vector<U> next;
  for(U v:xs){U root=sqrtfp((v+2)%P);if(root<P){next.push_back(root);next.push_back(sub(0,root));}}
  std::sort(next.begin(),next.end());next.erase(std::unique(next.begin(),next.end()),next.end());xs=std::move(next);
 }
 assert(xs.size()==1024);std::vector<Pt> signed_points;
 for(U x:xs){
  U f=(mul(mul(x,x),x)+mul(A,x)+B)%P;U y=sqrtfp(f);
  if(y<P&&y!=0){
   y=std::min(y,P-y);signed_points.push_back({x,y,false});signed_points.push_back({x,sub(0,y),false});
  }
 }
 assert(signed_points.size()==1018);std::unordered_set<U>sums;sums.reserve(518671);
 size_t pairs=0,identity_records=0;
 for(size_t i=0;i<signed_points.size();i++)for(size_t j=i;j<signed_points.size();j++){
  Pt q=add(signed_points[i],signed_points[j]);
  U key=q.infinity?(1ULL<<63):((q.x<<31)|q.y);
  sums.insert(key);pairs++;if(q.infinity)identity_records++;
 }
 assert(pairs==518671&&identity_records==509&&sums.size()==518085);
 size_t hits=0,targets=0;std::vector<size_t>hit_indices;
 if(argc==2){
  std::ifstream in(argv[1]);assert(in.good());std::string line;std::getline(in,line);
  while(std::getline(in,line)){
   if(line.empty())continue;auto a=line.find(','),b=line.find(',',a+1),c=line.find(',',b+1);
   assert(a!=std::string::npos&&b!=std::string::npos&&c!=std::string::npos);
   U x=std::stoull(line.substr(a+1,b-a-1)),y=std::stoull(line.substr(b+1,c-b-1));
   assert(x<P&&y<P&&mul(y,y)==(mul(mul(x,x),x)+mul(A,x)+B)%P);
   if(sums.count((x<<31)|y)){hits++;hit_indices.push_back(targets);}targets++;
  }
 }
 printf("{\"base_points\":509,\"signed_points\":1018,\"unordered_pairs\":%zu,\"distinct_sums\":%zu,\"identity_records\":%zu,\"targets_checked\":%zu,\"hits\":%zu}\n",pairs,sums.size(),identity_records,targets,hits);
 for(size_t i:hit_indices)std::cerr<<"hit_index="<<i<<"\n";
}
