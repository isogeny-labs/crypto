// Development unit control, not a new Semaev census or an independent-host replay.
// Fixed F2[x,y] fixture matched to DegreeControl.lean.
#include <algorithm>
#include <cassert>
#include <cstdint>
#include <iostream>
#include <utility>
#include <vector>
using Mono=std::pair<int,int>;
using Poly=std::vector<Mono>;
using Row=std::uint32_t;
struct Space {
 std::vector<Mono> cols;
 std::vector<Row> pivots;
 explicit Space(int degree) {
  // High degrees first: the suffix is the degree-at-most-one block.
  for(int d=degree;d>=0;--d)for(int x=d;x>=0;--x)cols.emplace_back(x,d-x);
  pivots.resize(cols.size());
 }
 Row encode(const Poly& p,Mono m={0,0}) const {
  Row r=0;
  for(auto t:p){
   t.first+=m.first;t.second+=m.second;
   auto it=std::find(cols.begin(),cols.end(),t);
   assert(it!=cols.end());r^=Row(1)<<(it-cols.begin());
  }
  return r;
 }
 bool insert(Row r) {
  for(unsigned i=0;i<pivots.size();i++)if(r&(Row(1)<<i)){
   if(pivots[i])r^=pivots[i];
   else {pivots[i]=r;return true;}
  }
  return false;
 }
 Poly decode(Row r)const {
  Poly p;for(unsigned i=0;i<cols.size();i++)if(r&(Row(1)<<i))p.push_back(cols[i]);return p;
 }
 int low_dimension()const {
  int count=0;
  for(unsigned i=0;i<cols.size();i++)if(pivots[i]&&cols[i].first+cols[i].second<=1)count++;
  return count;
 }
 int rank()const {return std::count_if(pivots.begin(),pivots.end(),[](Row r){return r!=0;});}
};
int degree(const Poly& p){int d=0;for(auto m:p)d=std::max(d,m.first+m.second);return d;}
Space closure(const std::vector<Poly>& generators,int D){
 Space s(D);
 for(const auto& p:generators){
  for(int d=0;d+degree(p)<=D;d++)for(int x=0;x<=d;x++)s.insert(s.encode(p,{x,d-x}));
 }
 bool changed;
 do {
  changed=false;
  auto snapshot=s.pivots;
  for(Row r:snapshot)if(r){auto p=s.decode(r);if(degree(p)<D){
   changed|=s.insert(s.encode(p,{1,0}));changed|=s.insert(s.encode(p,{0,1}));
  }}
 }while(changed);
 return s;
}
int main(){
 const Poly e0{{2,0},{1,0}},e1{{0,2},{0,1}},f{{1,1},{1,0},{0,1}};
 const std::vector<Poly> fixture{e0,e1,f};
 auto d2=closure(fixture,2),d3=closure(fixture,3);
 assert(d2.rank()==3&&d2.low_dimension()==0);
 assert(d3.rank()==9&&d3.low_dimension()==2);
 auto linear=closure({{{1,0}},{{0,1}}},1);
 assert(linear.rank()==2&&linear.low_dimension()==2);
 std::cout<<"{\"fixture\":\"degree-three-control\",\"low2\":0,\"low3\":2,\"rank2\":3,\"rank3\":9,\"known_threshold\":3,\"linear_positive_control\":2}\n";
}
