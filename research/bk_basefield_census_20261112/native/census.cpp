#include "../../theorem_census_20261109/native/finite.hpp"
#include <array>
#include <fstream>
#include <iostream>
#include <map>
#include <numeric>
#include <set>
#include <string>

using U=std::uint64_t;
bool prime(int p){if(p<2)return false;for(int d=2;d*d<=p;d++)if(p%d==0)return false;return true;}
std::vector<int> prime_factors(int n){std::vector<int>r;for(int d=2;d*d<=n;d++)if(n%d==0){r.push_back(d);while(n%d==0)n/=d;}if(n>1)r.push_back(n);return r;}
int embedding(int p,int n){if(std::gcd(p,n)!=1)return 0;int x=1;for(int k=1;k<=n;k++){x=x*p%n;if(x==1)return k;}throw std::runtime_error("unit order missing");}
struct Stats {
 U models=0,representatives=0,prime_cases=0,prime_to_p=0,characteristic_cases=0;
 U qualified=0,qualified_mismatches=0,omitted_condition_failures=0,omitted_r2_failures=0;
 U necessity_mismatches=0,ss_models=0,ss_prime_cases=0,ss_equal_two_failures=0;
 U ss_prime_bound_failures=0,ss_divisor_bound_failures=0,root2_checks=0,root3_checks=0;
 U order_checks=0,point_rows=0;int max_ss_prime_degree=0,max_ss_divisor_degree=0;
 std::map<int,U> full_by_prime;
 void print(const std::string&kind,int p=0)const{
  std::cout<<"{\"kind\":\""<<kind<<"\",\"p\":"<<p<<",\"models\":"<<models<<",\"representatives\":"<<representatives
   <<",\"prime_cases\":"<<prime_cases<<",\"prime_to_p\":"<<prime_to_p<<",\"characteristic_cases\":"<<characteristic_cases
   <<",\"qualified\":"<<qualified<<",\"qualified_mismatches\":"<<qualified_mismatches
   <<",\"omitted_condition_failures\":"<<omitted_condition_failures<<",\"omitted_r2_failures\":"<<omitted_r2_failures
   <<",\"necessity_mismatches\":"<<necessity_mismatches<<",\"ss_models\":"<<ss_models
   <<",\"ss_prime_cases\":"<<ss_prime_cases<<",\"ss_equal_two_failures\":"<<ss_equal_two_failures
   <<",\"ss_prime_bound_failures\":"<<ss_prime_bound_failures<<",\"ss_divisor_bound_failures\":"<<ss_divisor_bound_failures
   <<",\"root2_checks\":"<<root2_checks<<",\"root3_checks\":"<<root3_checks<<",\"order_checks\":"<<order_checks
   <<",\"point_rows\":"<<point_rows<<",\"max_ss_prime_degree\":"<<max_ss_prime_degree
   <<",\"max_ss_divisor_degree\":"<<max_ss_divisor_degree<<",\"full_by_prime\":{";
  bool first=true;for(auto [r,n]:full_by_prime){if(!first)std::cout<<',';first=false;std::cout<<'"'<<r<<"\":"<<n;}
  std::cout<<"}}\n";
 }
 Stats& operator+=(const Stats&s){
  models+=s.models;representatives+=s.representatives;prime_cases+=s.prime_cases;prime_to_p+=s.prime_to_p;characteristic_cases+=s.characteristic_cases;
  qualified+=s.qualified;qualified_mismatches+=s.qualified_mismatches;omitted_condition_failures+=s.omitted_condition_failures;omitted_r2_failures+=s.omitted_r2_failures;
  necessity_mismatches+=s.necessity_mismatches;ss_models+=s.ss_models;ss_prime_cases+=s.ss_prime_cases;ss_equal_two_failures+=s.ss_equal_two_failures;
  ss_prime_bound_failures+=s.ss_prime_bound_failures;ss_divisor_bound_failures+=s.ss_divisor_bound_failures;root2_checks+=s.root2_checks;root3_checks+=s.root3_checks;
  order_checks+=s.order_checks;point_rows+=s.point_rows;max_ss_prime_degree=std::max(max_ss_prime_degree,s.max_ss_prime_degree);max_ss_divisor_degree=std::max(max_ss_divisor_degree,s.max_ss_divisor_degree);
  for(auto [r,n]:s.full_by_prime)full_by_prime[r]+=n;
  return *this;
 }
};
std::vector<Point> short_points(const Curve&c,const std::vector<std::vector<int>>&roots){
 const Field&f=c.f;std::vector<Point>ps{{}};
 for(int x=0;x<f.p;x++){
  int v=f.add(f.add(f.power(x,3),f.mul(c.a4,x)),c.a6);
  for(int y:roots[v]){Point p{x,y,false};if(!c.on(p))throw std::runtime_error("point off curve");ps.push_back(p);}
 }
 U independent=1;
 for(int x=0;x<f.p;x++){
  int v=f.add(f.add(f.power(x,3),f.mul(c.a4,x)),c.a6);
  independent+=v==0?1:(f.power(v,(f.p-1)/2)==1?2:0);
 }
 if(independent!=ps.size())throw std::runtime_error("Euler/root-table point count mismatch");
 return ps;
}
int root_oracle(const Curve&c,const std::vector<std::vector<int>>&roots,int r,bool corrupt=false){
 const Field&f=c.f;int n=1;
 for(int x=0;x<f.p;x++){
  int g=f.add(f.add(f.power(x,3),f.mul(c.a4,x)),c.a6);
  if(r==2){if(g==0)n++;continue;}
  int v=f.sub(f.add(f.add(f.mul(f.c(3),f.power(x,4)),f.mul(f.mul(f.c(6),c.a4),f.power(x,2))),f.mul(f.mul(f.c(corrupt?11:12),c.a6),x)),f.mul(c.a4,c.a4));
  if(v==0)n+=int(roots[g].size());
 }
 return n;
}
std::map<int,int> inspect(const Curve&c,U weight,Stats&stats,const std::vector<std::vector<int>>&roots,bool emit=true){
 const Field&f=c.f;const int p=f.p;bool short_model=p>=5;
 std::vector<Point>ps=short_model?short_points(c,roots):c.points();int N=int(ps.size()),trace=p+1-N;
 bool ss=trace%p==0;auto factors=prime_factors(N);std::set<int>tests(factors.begin(),factors.end());
 if(short_model){tests.insert(2);tests.insert(3);}std::map<int,int>killed;
 for(int r:tests){int n=0;for(Point P:ps)if(c.mul(r,P).inf)n++;killed[r]=n;}
 for(Point P:ps){if(!c.mul(N,P).inf)throw std::runtime_error("computed cardinal does not kill point");stats.order_checks++;}
 if(short_model){
  if(killed.at(2)!=root_oracle(c,roots,2)||killed.at(3)!=root_oracle(c,roots,3))throw std::runtime_error("group/polynomial torsion mismatch");
  stats.root2_checks++;stats.root3_checks++;
 }
 stats.models+=weight;stats.representatives++;stats.point_rows+=ps.size();if(ss)stats.ss_models+=weight;
 int divisor_max=0;std::vector<std::pair<int,int>>divisors;
 if(ss)for(int d=2;d<=N;d++)if(N%d==0&&std::gcd(p,d)==1){int k=embedding(p,d);divisors.emplace_back(d,k);divisor_max=std::max(divisor_max,k);if(k>(p>3?2:6))stats.ss_divisor_bound_failures+=weight;}
 if(ss)stats.max_ss_divisor_degree=std::max(stats.max_ss_divisor_degree,divisor_max);
 if(emit)std::cout<<"{\"kind\":\"model\",\"p\":"<<p<<",\"a\":["<<c.a1<<','<<c.a2<<','<<c.a3<<','<<c.a4<<','<<c.a6<<"],\"orbit_size\":"<<weight<<",\"N\":"<<N<<",\"trace\":"<<trace<<",\"source_supersingular\":"<<ss<<",\"divisor_max_embedding_degree\":"<<divisor_max<<",\"factors\":[";
 bool first=true;
 for(int r:factors){
  bool coprime=r!=p,unit=(p-1)%r==0,full=killed.at(r)==r*r,qualified=coprime&&!unit,unrestricted=coprime&&(full!=unit),r2=N%(r*r)==0;int k=coprime?embedding(p,r):0;
  stats.prime_cases+=weight;
  if(coprime)stats.prime_to_p+=weight;else stats.characteristic_cases+=weight;
  if(qualified){stats.qualified+=weight;if(full)stats.qualified_mismatches+=weight;}
  if(unrestricted){stats.omitted_condition_failures+=weight;if(r2)stats.omitted_r2_failures+=weight;}
  if(coprime&&full&&!unit)stats.necessity_mismatches+=weight;
  if(coprime&&full)stats.full_by_prime[r]+=weight;
  if(ss&&coprime){stats.ss_prime_cases+=weight;if(k!=2)stats.ss_equal_two_failures+=weight;if(k>(p>3?2:6))stats.ss_prime_bound_failures+=weight;stats.max_ss_prime_degree=std::max(stats.max_ss_prime_degree,k);}
  if(emit){if(!first)std::cout<<',';first=false;std::cout<<"{\"r\":"<<r<<",\"killed\":"<<killed.at(r)<<",\"prime_to_p\":"<<coprime<<",\"r_div_p_minus_one\":"<<unit<<",\"r2_div_N\":"<<r2<<",\"full_size\":"<<full<<",\"qualified_k1_mismatch\":"<<(qualified&&full)<<",\"unrestricted_k1_mismatch\":"<<unrestricted<<",\"embedding_degree\":"<<k<<'}';}
 }
 if(emit){
  std::cout<<"],\"ss_divisors\":[";bool first_divisor=true;
  for(auto [d,k]:divisors){if(!first_divisor)std::cout<<',';first_divisor=false;std::cout<<"{\"d\":"<<d<<",\"embedding_degree\":"<<k<<'}';}
  std::cout<<"],\"r2_killed\":"<<(killed.count(2)?killed.at(2):0)<<",\"r3_killed\":"<<(killed.count(3)?killed.at(3):0)<<"}\n";
 }
 return killed;
}
void controls(){
 struct Fixture{int p,a,b,N,r,killed;};std::array<Fixture,4>fixtures{{{5,4,0,8,2,4},{7,0,2,9,3,9},{7,3,2,9,3,3},{19,1,6,18,3,3}}};
 for(auto z:fixtures){Field f(z.p,1,z.p);field_controls(f);Curve c(f,0,0,0,z.a,z.b);std::vector<std::vector<int>>roots(z.p);for(int y=0;y<z.p;y++)roots[f.mul(y,y)].push_back(y);Stats s;auto counts=inspect(c,1,s,roots);if(int(short_points(c,roots).size())!=z.N||counts.at(z.r)!=z.killed)throw std::runtime_error("known control missed");}
 Field f(7,1,7);Curve c(f,0,0,0,0,2);std::vector<std::vector<int>>roots(7);for(int y=0;y<7;y++)roots[f.mul(y,y)].push_back(y);int correct=root_oracle(c,roots,3),wrong=root_oracle(c,roots,3,true);if(correct!=9||wrong==correct)throw std::runtime_error("negative control undetected");
 std::cout<<"{\"kind\":\"controls\",\"known_fixtures\":4,\"known_pass\":1,\"corrupt_oracle_detected\":1,\"correct\":"<<correct<<",\"corrupt\":"<<wrong<<"}\n";
}
int main(int argc,char**argv){try{
 if(argc==2&&std::string(argv[1])=="--controls"){controls();return 0;}
 if(argc!=3||std::string(argv[1])!="--limit")throw std::runtime_error("usage: census --controls | --limit N (exclusive, N<=500)");
 int limit=std::stoi(argv[2]);if(limit<5||limit>500)throw std::runtime_error("limit outside frozen supported range");Stats total;
 std::cout<<"{\"kind\":\"scope\",\"exclusive_prime_limit\":"<<limit<<",\"extension_degree\":1,\"all_general_models_in_char2_3\":1,\"all_short_models_in_char_ge5\":1,\"weighted_scaling_orbits\":1}\n";
 for(int p=2;p<limit;p++)if(prime(p)){
  Field f(p,1,p);field_controls(f);std::vector<std::vector<int>>roots(p);for(int y=0;y<p;y++)roots[f.mul(y,y)].push_back(y);Stats stats;
  if(p<5){int bound=1;for(int i=0;i<5;i++)bound*=p;for(int j=0;j<bound;j++){int n=j;std::array<int,5>a{};for(int&i:a){i=n%p;n/=p;}Curve c(f,a[0],a[1],a[2],a[3],a[4]);if(c.discriminant())inspect(c,1,stats,roots);}if(stats.models!=U(bound-bound/p))throw std::runtime_error("general model population mismatch");}
  else{
   std::vector<int>owner(p*p,-1);int next=0,singular=0;
   for(int a=0;a<p;a++)for(int b=0;b<p;b++){
    int key=a*p+b;if(owner[key]!=-1)continue;Curve c(f,0,0,0,a,b);if(!c.discriminant()){owner[key]=-2;singular++;continue;}
    int id=next++;U weight=0;
    for(int u=1;u<p;u++){int aa=f.mul(f.power(u,4),a),bb=f.mul(f.power(u,6),b),idx=aa*p+bb;if(idx<key)throw std::runtime_error("orbit representative not minimal");if(owner[idx]==-1){owner[idx]=id;weight++;}else if(owner[idx]!=id)throw std::runtime_error("overlapping scaling orbits");}
    if(!weight)throw std::runtime_error("empty scaling orbit");
    inspect(c,weight,stats,roots);
   }
   if(singular!=p||stats.models!=U(p*(p-1))||std::find(owner.begin(),owner.end(),-1)!=owner.end())throw std::runtime_error("short model population mismatch");
  }
  stats.print("field",p);std::cout.flush();total+=stats;
 }
 total.print("total");return 0;
 }catch(const std::exception&e){std::cerr<<"census error: "<<e.what()<<'\n';return 1;}}
