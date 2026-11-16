// Complete projective length-three divisor encoding from a supplied split
// support multiset on a smooth plane quartic, using local jets. This is NOT
// inversion of an eight-coordinate Kummer image.
#include "ff.hpp"
#include <map>
#include <set>
using P3=std::array<F,3>;
using Series=std::vector<F>;
struct Term{std::array<int,3> e;long c;};
std::vector<Term> quartic(){return {{{4,0,0},1},{{0,4,0},1},{{0,0,4},1},{{0,1,3},8},{{0,2,2},24},{{0,3,1},32},{{0,4,0},16}};}
std::vector<std::array<int,3>> monomials(){std::vector<std::array<int,3>> m;for(int i=0;i<=3;i++)for(int j=0;j<=3-i;j++)m.push_back({i,j,3-i-j});return m;}
P3 normalize(P3 p){auto it=std::find_if(p.begin(),p.end(),[](const F&v){return bool(v);});require(it!=p.end(),"zero projective vector");F scale=*it;for(auto&v:p)v=v/scale;return p;}
std::string key(const P3&p){std::string k;for(auto&a:p)for(auto c:a.coeffs())k+=std::to_string(c)+",";return k;}
P3 frob(P3 p){for(auto&a:p)a=a.frob();return normalize(p);}
F evaluate(const P3&p){F v;for(auto&t:quartic())v+=F(t.c)*p[0].pow(t.e[0])*p[1].pow(t.e[1])*p[2].pow(t.e[2]);return v;}
F derivative(const P3&p,int j){F v;for(auto&t:quartic())if(t.e[j]){auto e=t.e;e[j]--;v+=F(t.c*t.e[j])*p[0].pow(e[0])*p[1].pow(e[1])*p[2].pow(e[2]);}return v;}
Series multiply(const Series&a,const Series&b){Series c(a.size());for(size_t i=0;i<a.size();i++)for(size_t j=0;i+j<a.size();j++)c[i+j]+=a[i]*b[j];return c;}
Series power(Series a,int n){Series b(a.size());b[0]=1;for(int i=0;i<n;i++)b=multiply(b,a);return b;}
Series monomial(const std::array<Series,3>&s,const std::array<int,3>&e){return multiply(multiply(power(s[0],e[0]),power(s[1],e[1])),power(s[2],e[2]));}
std::array<Series,3> local_series(P3 p,int multiplicity){
 require(!evaluate(p),"support point off the quartic");int pivot=0;while(!p[pivot])pivot++;F scale=p[pivot];for(auto&v:p)v=v/scale;
 int dependent=-1,independent=-1;for(int j=0;j<3;j++)if(j!=pivot&&derivative(p,j)){dependent=j;break;}
 require(dependent>=0,"singular point has no smooth local parameter");for(int j=0;j<3;j++)if(j!=pivot&&j!=dependent)independent=j;
 std::array<Series,3>s{Series(multiplicity),Series(multiplicity),Series(multiplicity)};for(int j=0;j<3;j++)s[j][0]=p[j];if(multiplicity>1)s[independent][1]=1;
 F unit=derivative(p,dependent);
 for(int n=1;n<multiplicity;n++){Series value(multiplicity);for(auto&t:quartic()){auto v=monomial(s,t.e);for(int j=0;j<multiplicity;j++)value[j]+=F(t.c)*v[j];}s[dependent][n]=-value[n]/unit;}
 for(int n=0;n<multiplicity;n++){F value;for(auto&t:quartic())value+=F(t.c)*monomial(s,t.e)[n];require(!value,"implicit local jet does not satisfy quartic");}return s;
}
uint64_t modpow(uint64_t a,uint64_t e){uint64_t b=1;while(e){if(e&1)b=b*a%FP;a=a*a%FP;e>>=1;}return b;}
struct Encoding{std::string canonical;json examples;int rank=0;};
Encoding encode(const std::vector<P3>&multiset){
 require(multiset.size()==3,"effective divisor must have degree three");std::map<std::string,std::pair<P3,int>> remaining;
 for(auto p:multiset){p=normalize(p);require(!evaluate(p),"point is not on curve");auto k=key(p);if(!remaining.count(k))remaining.emplace(k,std::make_pair(p,0));remaining[k].second++;}
 auto mons=monomials();std::vector<std::vector<uint64_t>>rows;json orbits=json::array();
 while(!remaining.empty()){
  auto item=remaining.begin()->second;P3 p=item.first;int mult=item.second;std::vector<P3>orbit;P3 q=p;
  do{require(orbit.size()<6,"support Frobenius orbit exceeds splitting field");orbit.push_back(q);q=frob(q);}while(key(q)!=key(p));
  require(orbit.size()*size_t(mult)<=3,"nonrational or oversized divisor support");
  for(auto&a:orbit){auto it=remaining.find(key(a));require(it!=remaining.end()&&it->second.second==mult,"support multiset is not Frobenius invariant");}
  for(auto&a:orbit)remaining.erase(key(a));
  auto series=local_series(p,mult);json point=json::array();for(auto&a:p)point.push_back(a.coeffs());
  orbits.push_back({{"point",point},{"degree",orbit.size()},{"multiplicity",mult}});
  std::vector<Series>values;for(auto&e:mons)values.push_back(monomial(series,e));
  for(int n=0;n<mult;n++)for(int a=0;a<fq_nmod_ctx_degree(FC);a++){std::vector<uint64_t>row(mons.size());for(size_t j=0;j<mons.size();j++)row[j]=values[j][n].coeffs()[a];rows.push_back(row);}
 }
 int rank=0;std::vector<int>pivots;for(size_t col=0;col<mons.size();col++){
  int row=rank;while(row<int(rows.size())&&!rows[row][col])row++;if(row==int(rows.size()))continue;std::swap(rows[row],rows[rank]);uint64_t inv=modpow(rows[rank][col],FP-2);
  for(auto&v:rows[rank])v=v*inv%FP;
  for(size_t k=0;k<rows.size();k++)if(int(k)!=rank){uint64_t c=rows[k][col];for(size_t j=0;j<mons.size();j++)rows[k][j]=(rows[k][j]+FP-c*rows[rank][j]%FP)%FP;}
  pivots.push_back(col);rank++;
 }
 require(rank==3,"cubic jet evaluation must have rank three");std::string canonical;for(int i=0;i<rank;i++)for(auto c:rows[i])canonical+=std::to_string(c)+",";
 // Each nullspace basis vector is checked against every original jet condition.
 int dimension=0;for(size_t col=0;col<mons.size();col++)if(std::find(pivots.begin(),pivots.end(),int(col))==pivots.end()){
  std::vector<uint64_t>v(mons.size());v[col]=1;for(int i=0;i<rank;i++)v[pivots[i]]=(FP-rows[i][col])%FP;
  for(auto&orb:orbits){P3 p;for(int j=0;j<3;j++){auto cc=orb["point"][j].get<std::vector<uint64_t>>();nmod_poly_zero(p[j].v);for(size_t k=0;k<cc.size();k++)nmod_poly_set_coeff_ui(p[j].v,k,cc[k]);}int m=orb["multiplicity"];auto ss=local_series(p,m);for(int k=0;k<m;k++){F value;for(size_t j=0;j<mons.size();j++)value+=F(v[j])*monomial(ss,mons[j])[k];require(!value,"nullspace cubic violates a local jet");}}
  dimension++;
 }
 require(dimension==7,"degree-three homogeneous monomial count"); // 10 cubics, rank 3.
 return{canonical,{{"support_orbits",orbits},{"degree_three_ideal_dimension",dimension},{"rref",std::vector<std::vector<uint64_t>>(rows.begin(),rows.begin()+rank)}},rank};
}
std::vector<F>subfield(int d){
 if(d==1){std::vector<F>v;for(uint64_t i=0;i<FP;i++)v.push_back(F(i));return v;}
 F s;for(int attempt=0;;attempt++){require(attempt<1000,"subfield generator budget");F a=F::random();s=F(0);for(int k=0;k<6/d;k++)s+=a.frob(d*k);if(s.frob()!=s&&s.frob(d)==s)break;}
 std::vector<F>values;uint64_t n=1;for(int i=0;i<d;i++)n*=FP;for(uint64_t i=0;i<n;i++){uint64_t code=i;F a,power(1);for(int j=0;j<d;j++){a+=F(code%FP)*power;power=power*s;code/=FP;}values.push_back(a);}return values;
}
std::vector<std::vector<P3>>closed_points(int d){
 auto field=subfield(d);std::map<std::string,std::vector<P3>>orbits;
 auto accept=[&](P3 P){P=normalize(P);P3 Q=P;std::vector<P3>orb;do{orb.push_back(Q);Q=frob(Q);}while(key(Q)!=key(P)&&orb.size()<7);require(orb.size()<=6,"orbit enumeration bound");if(int(orb.size())!=d)return;auto best=std::min_element(orb.begin(),orb.end(),[](const P3&a,const P3&b){return key(a)<key(b);});orbits[key(*best)]=orb;};
 for(auto&x:field){std::vector<F>c(5);for(auto&t:quartic())c[t.e[1]]+=F(t.c)*x.pow(t.e[0]);for(auto&y:roots(c))if(y.frob(d)==y)accept({x,y,F(1)});}
 std::vector<F>infinite(5);for(auto&t:quartic())if(t.e[2]==0)infinite[t.e[1]]+=F(t.c);for(auto&y:roots(infinite))if(y.frob(d)==y)accept({F(1),y,F(0)});P3 last{F(0),F(1),F(0)};if(!evaluate(last))accept(last);
 std::vector<std::vector<P3>>out;for(auto&v:orbits)out.push_back(v.second);return out;
}
int main(int argc,char**argv){try{
 require(argc==3,"usage: projective_divisor PRIME OUT");int p=std::stoi(argv[1]);require(p>=5&&p<=13,"exhaustive diagnostic prime bound");field_start(p,6);
 {auto rational=closed_points(1),quadratic=closed_points(2),cubic=closed_points(3);std::set<std::string>seen;uint64_t count=0,repeated=0,infinite=0;json examples=json::object();
 auto check=[&](const std::vector<P3>&points,const std::string&stratum){Encoding e=encode(points);require(seen.insert(e.canonical).second,"distinct effective divisors have identical cubic ideals");count++;bool rep=false,inf=false;for(size_t i=0;i<points.size();i++){inf|=!points[i][2];for(size_t j=0;j<i;j++)rep|=key(normalize(points[i]))==key(normalize(points[j]));}repeated+=rep;infinite+=inf;if(!examples.contains(stratum))examples[stratum]=e.examples;if(inf&&!examples.contains("infinity"))examples["infinity"]=e.examples;};
 for(size_t i=0;i<rational.size();i++)for(size_t j=i;j<rational.size();j++)for(size_t k=j;k<rational.size();k++)check({rational[i][0],rational[j][0],rational[k][0]},i==k?"triple_rational":i==j||j==k?"double_rational":"three_rational");
 for(auto&P:rational)for(auto&Q:quadratic)check({P[0],Q[0],Q[1]},"rational_plus_quadratic");
 for(auto&P:cubic)check(P,"cubic_closed_point");
 uint64_t n=rational.size();uint64_t expected=n*(n+1)*(n+2)/6+n*quadratic.size()+cubic.size();require(count==expected,"complete effective-divisor enumeration count");
 bool rejected=false;if(!cubic.empty())try{encode({cubic[0][0],cubic[0][0],cubic[0][1]});}catch(const std::exception&){rejected=true;}require(rejected,"nonrational support control");
 write_json(argv[2],{{"status","passed"},{"p",p},{"curve","X^4+Y^4+(Z+2Y)^4"},{"smoothness","invertible linear change of Fermat quartic; characteristic does not divide 4"},{"rational_points",rational.size()},{"degree_two_closed_points",quadratic.size()},{"degree_three_closed_points",cubic.size()},{"all_effective_degree_three_divisors",count},{"with_repeated_support",repeated},{"with_support_at_infinity",infinite},{"distinct_cubic_ideals",seen.size()},{"nonrational_support_rejected",rejected},{"examples",examples},{"scope","Exhaustive finite diagnostic of projective divisor recovery. General correctness uses smooth local parameters and cubic interpolation; no Kummer-image inversion or source transfer is implemented here."}});
 std::cout<<"projective divisor checks "<<count<<" p="<<p<<"\n";
 }field_stop();return 0;
 }catch(const std::exception&e){std::cerr<<"ERROR "<<e.what()<<'\n';return 2;}}
