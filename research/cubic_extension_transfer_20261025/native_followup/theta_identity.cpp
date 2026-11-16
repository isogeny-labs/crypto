// Exact rational-function calculation, rather than fitting finite signs.
#define main chart_audit_reference_main
#include "proof_audit.cpp"
#undef main
json sum(const std::vector<json>&v){return{{"op","add"},{"args",v}};}
json sub(json a,json b){return add({a,mul({con(-1),b})});}
json from_poly(const Poly&p,const std::vector<std::string>&variables){std::vector<json>terms;for(auto&[e,c]:p)if(c){std::vector<json>factors{con(c)};for(size_t i=0;i<e.size();i++)for(int k=0;k<e[i];k++)factors.push_back(var(variables[i]));terms.push_back({{"op","mul"},{"args",factors}});}return sum(terms);}
int main(int argc,char**argv){try{
 if(argc!=2)throw std::runtime_error("usage: theta_identity RESULTS_DIR");
 std::string d=argv[1],source="THETA_ACTION.md; universal polynomial calculation in theta_identity.cpp";auto s=var("s"),v=var("v"),x=var("x"),y=var("y");auto u=mul({con(3),power(s,2)}),m=mul({con(3),s});
 auto a=sub(mul({con(6),s,v}),mul({con(27),power(s,4)}));auto b=add({power(v,2),mul({con(-18),power(s,3),v}),mul({con(54),power(s,6)})});
 std::array<std::array<json,3>,3>N{{
  {sub(mul({m,u}),v),sub(mul({m,power(u,2)}),mul({con(3),u,v})),sub(mul({con(3),power(v,2)}),mul({u,m,v}))},
  {mul({con(-1),m}),sub(mul({con(2),v}),mul({m,u})),mul({m,v})},
  {con(-1),mul({con(-1),u}),mul({con(-1),v})}}};
 std::vector<std::string>variables{"s","v"};std::vector<json>lhs,rhs;
 for(int i=0;i<3;i++)for(int j=0;j<3;j++){std::string w="w"+std::to_string(3*i+j);variables.push_back(w);std::vector<json>entry;for(int k=0;k<3;k++)for(int l=0;l<3;l++)entry.push_back(mul({N[i][k],N[k][l],N[l][j]}));lhs.push_back(mul({var(w),sum(entry)}));rhs.push_back(mul({var(w),i==j?power(mul({con(2),v}),3):con(0)}));}
 auto order=issue("universal normalized three-torsion lift has order three",variables,32,sum(lhs),sum(rhs),source);write(d+"/theta-order.identity.json",order);
 auto delta=sub(x,u);auto nx=sub(power(sub(y,v),2),mul({add({x,u}),power(delta,2)}));auto ny=sub(mul({sub(y,v),sub(mul({u,power(delta,2)}),nx)}),mul({v,power(delta,3)}));auto section=add({v,mul({con(-1),m,u}),mul({m,x}),y});
 std::vector<json>differences;std::array<json,3>point{con(1),x,y};
 for(int j=0;j<3;j++){std::vector<json>right;for(int i=0;i<3;i++)right.push_back(mul({point[i],N[i][j],power(delta,3)}));json left=j==0?mul({con(-1),section,power(delta,3)}):j==1?mul({con(-1),section,nx,delta}):mul({con(-1),section,ny});differences.push_back(mul({var("t"+std::to_string(j)),sub(left,sum(right))}));}
 variables={"s","v","x","y","t0","t1","t2"};json curve=sub(power(y,2),add({power(x,3),mul({a,x}),b}));json target=sum(differences);auto remainder=exact(target,variables),G=exact(curve,variables);Poly quotient;
 for(int count=0;;count++){
  if(count>10000)throw std::runtime_error("polynomial division budget");
  auto it=remainder.end();for(auto k=remainder.begin();k!=remainder.end();++k)if(k->second&&k->first[3]>=2&&(it==remainder.end()||k->first[3]>it->first[3]))it=k;
  if(it==remainder.end())break;
  auto exponent=it->first;int64_t coefficient=it->second;exponent[3]-=2;quotient[exponent]+=coefficient;
  for(auto&[e,c]:G){auto mon=exponent;for(size_t j=0;j<mon.size();j++)mon[j]+=e[j];remainder[mon]-=coefficient*c;}
 }
 for(auto&[e,c]:remainder)if(c)throw std::runtime_error("translation identity has nonzero curve remainder");
 auto translation=issue("universal three-torsion lift equals elliptic translation",variables,64,target,mul({curve,from_poly(quotient,variables)}),source);write(d+"/theta-translation.identity.json",translation);
 auto disc=issue("marked-three-torsion family discriminant factor",{"s","v"},12,add({mul({con(4),power(a,3)}),mul({con(27),power(b,2)})}),mul({con(27),power(v,3),sub(v,mul({con(4),power(s,3)}))}),source);write(d+"/theta-discriminant.identity.json",disc);
 write(d+"/theta-symbolic-audit.json",{{"status","passed"},{"coefficient_domain","integers; denominators cleared; nonzero v; characteristic not 2 or 3"},{"certificate_ids",{order["id"],translation["id"],disc["id"]}},{"exact_polynomial_checks",3},{"translation_remainder_zero",true},{"scope","Universal explicit three-torsion translation lift. This does not compute the twelve modular obstruction divisors or prove extra-step coverage."}});std::cout<<"symbolic theta identities passed\n";
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}}
