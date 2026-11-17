#include "json.hpp"
#include <boost/multiprecision/cpp_int.hpp>
#include <set>

using Integer=boost::multiprecision::cpp_int;
static void check(bool ok,const char* what){if(!ok)throw std::runtime_error(what);}
static Integer value(const Json& j){
 const auto s=j.str();check(!s.empty(),"empty integer");
 size_t i=s[0]=='-'?1:0;check(i<s.size(),"integer sign");Integer out=0;
 for(;i<s.size();++i){check(s[i]>='0'&&s[i]<='9',"decimal integer");out*=10;out+=s[i]-'0';}
 return s[0]=='-'?-out:out;
}
static Integer mod(Integer a,const Integer& m){a%=m;if(a<0)a+=m;return a;}
static void verify(const Json& curve,const Json& relation){
 check(curve.at("name").str()==relation.at("name").str(),"exact curve name");
 const Integer p=value(curve.at("p")),n=value(curve.at("n")),t=value(curve.at("t"));
 check(n==p+1-t&&n>0,"trace and group order");
 const Integer d=4*p-t*t;check(d>0,"positive norm form");
 const Integer a=value(relation.at("alpha_a")),b=value(relation.at("alpha_b"));
 const Integer s=a+b,mu=value(relation.at("mu"));
 check(mod(s,n)==mu&&mu>0&&mu<n,"rational scalar label");
 const Integer norm=a*a+t*a*b+p*b*b;
 check(norm==value(relation.at("norm"))&&norm>0,"saved norm");
 const Integer square=2*n*b+(t-2)*s;
 check(4*n*norm==square*square+d*s*s,"completed square identity");
 check(d*s*s<=4*n*norm,"finite search tail bound");
 const Integer ell=value(relation.at("degree")),rho=value(relation.at("orientation_root"));
 check(ell>2&&rho>0&&rho<ell&&rho!=1,"supported orientation");
 check(mod(rho*rho-t*rho+p,ell)==0,"Frobenius root");
 const Integer opposite=mod(t-rho,ell);
 check(opposite!=rho&&opposite!=0,"distinct opposite root");
 check(mod(s+(rho-1)*b,ell)==0,"selected kernel");
 check(mod(b,ell)!=0&&mod(s+(opposite-1)*b,ell)!=0,"opposite kernel retained");
 check(mod(norm,ell)==0&&norm/ell==value(relation.at("residual_degree")),"residual degree");
}
int main(int argc,char**argv){try{
 check(argc==2,"usage: lift-identity FIXTURE_DIR");
 const std::string dir=argv[1];const auto curves=read_json(dir+"/published-curves.json");
 const auto relations=read_json(dir+"/marked-relations.json");std::set<std::string> seen;unsigned fixtures=0,rejections=0;
 for(const auto& curve:curves.at("rows").array){
  const auto name=curve.at("name").str();check(seen.insert(name).second,"duplicate curve");
  const Json* selected=nullptr;for(const auto& candidate:relations.at("rows").array)if(candidate.at("name").str()==name){check(selected==nullptr,"duplicate relation");selected=&candidate;}
  check(selected!=nullptr,"missing relation");verify(curve,*selected);++fixtures;
  for(const char* key:{"norm","mu","orientation_root"}){
   Json changed=*selected;changed.object.at(key).kind=Json::String;
   changed.object.at(key).value=key==std::string("orientation_root")?"1":(value(changed.at(key))+1).convert_to<std::string>();
   bool rejected=false;try{verify(curve,changed);}catch(const std::exception&){rejected=true;}
   check(rejected,"corruption accepted");++rejections;
  }
 }
 check(fixtures==5&&relations.at("rows").array.size()==5,"exact five-curve panel");
 std::cout<<"{\"existing_certificate_replays\":"<<fixtures<<",\"mutations_rejected\":"<<rejections<<",\"integer_backend\":\"Boost.Multiprecision cpp_int\",\"minimizing_search_performed\":false,\"timing_performed\":false}\n";
 return 0;
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
