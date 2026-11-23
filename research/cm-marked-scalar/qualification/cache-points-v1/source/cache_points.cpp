#include "json.hpp"
#include <gmpxx.h>
#include <iostream>
#include <set>
namespace replay {
using Int=mpz_class;
static void check(bool b,const char*s){if(!b)throw std::runtime_error(s);}
static Int integer(const Json&j){const auto s=j.str();check(!s.empty(),"decimal field");size_t start=s[0]=='-'?1:0;check(start<s.size(),"decimal sign");for(size_t i=start;i<s.size();++i)check(s[i]>='0'&&s[i]<='9',"decimal digits");Int n;check(mpz_set_str(n.get_mpz_t(),s.c_str(),10)==0,"GMP decimal parse");return n;}
struct Point {bool infinity=true;Int x=0,y=0;};
struct Curve {
 Int p,a,b,n,mu;unsigned ell;
 Int mod(Int v)const{v%=p;if(v<0)v+=p;return v;}
 Int inverse(const Int&v)const{Int out;check(mpz_invert(out.get_mpz_t(),v.get_mpz_t(),p.get_mpz_t())!=0,"nonzero field denominator");return out;}
 bool on_curve(const Point&P)const{return P.infinity||mod(P.y*P.y)==mod(P.x*P.x*P.x+a*P.x+b);}
 Point add(const Point&P,const Point&Q)const{
  if(P.infinity)return Q;
  if(Q.infinity)return P;
  if(P.x==Q.x&&mod(P.y+Q.y)==0)return {};
  Int slope;if(P.x==Q.x){check(P.y==Q.y,"same-x curve branch");slope=mod((3*P.x*P.x+a)*inverse(mod(2*P.y)));}else slope=mod((Q.y-P.y)*inverse(mod(Q.x-P.x)));
  Point R;R.infinity=false;R.x=mod(slope*slope-P.x-Q.x);R.y=mod(slope*(P.x-R.x)-P.y);return R;
 }
 Point multiply(Point P,Int scalar)const{if(scalar<0){scalar=-scalar;P.y=mod(-P.y);}Point out;while(scalar!=0){if(mpz_odd_p(scalar.get_mpz_t()))out=add(out,P);scalar/=2;if(scalar!=0)P=add(P,P);}return out;}
 std::string encode(const Point&P)const{if(P.infinity)return "00";size_t size=(mpz_sizeinbase(p.get_mpz_t(),2)+7)/8;std::string x=P.x.get_str(16);check(x.size()<=2*size,"encoded coordinate size");return std::string(mpz_odd_p(P.y.get_mpz_t())?"03":"02")+std::string(2*size-x.size(),'0')+x;}
};
static std::map<std::string,Curve> inputs(const std::string&directory){auto curves=read_json(directory+"/published-curves.json"),marks=read_json(directory+"/marked-relations.json");std::map<std::string,Curve> out;for(const auto&c:curves.at("rows").array){const auto name=c.at("name").str();const Json*marked=nullptr;for(const auto&m:marks.at("rows").array)if(m.at("name").str()==name){check(!marked,"duplicate marked identity");marked=&m;}check(marked,"marked input missing");check(out.emplace(name,Curve{integer(c.at("p")),integer(c.at("a")),integer(c.at("b")),integer(c.at("n")),integer(marked->at("mu")),unsigned(marked->at("degree").integer())}).second,"duplicate curve identity");}check(out.size()==5,"five exact curves");return out;}
static unsigned verify(const Json&data,const std::map<std::string,Curve>&curves){check(data.at("schema").str()=="cm-cache-cold-points/v2","point schema");std::set<std::tuple<std::string,std::string,unsigned>> seen;unsigned count=0;for(const auto&row:data.at("rows").array){const auto name=row.at("curve").str(),panel=row.at("panel").str();auto index=row.at("index").unsigned_integer();check(curves.count(name)&&(panel=="calibration"||panel=="heldout")&&index<(panel=="calibration"?4u:8u)&&seen.emplace(name,panel,unsigned(index)).second,"exact point matrix");const auto&C=curves.at(name);auto req=JsonParser(row.at("request").str()).read();check(req.at("name").str()==name&&integer(req.at("curve").at("p"))==C.p&&integer(req.at("curve").at("n"))==C.n&&integer(req.at("marked").at("mu"))==C.mu&&req.at("marked").at("degree").integer()==int(C.ell),"request parameter binding");check(req.at("point").at("infinity").kind==Json::Boolean&&req.at("point").at("infinity").value=="false","nonzero point request");Point P{false,integer(req.at("point").at("x")),integer(req.at("point").at("y"))};check(P.x>=0&&P.x<C.p&&P.y>=0&&P.y<C.p&&C.on_curve(P),"independent curve membership");check(C.multiply(P,C.n).infinity,"independent subgroup membership");auto Y=C.multiply(P,C.mu);check(C.on_curve(Y)&&C.encode(Y)==row.at("expected").str(),"independent GMP mu point action");++count;}check(count==60,"complete independent point replay");return count;}
}
int main(int argc,char**argv){try{using namespace replay;check(argc==3,"cache-points FIXTURES POINT_MANIFEST");auto curves=inputs(argv[1]);auto data=read_json(argv[2]);auto count=verify(data,curves);unsigned negative=0;for(unsigned mode=0;mode<3;++mode){auto bad=data;auto&row=bad.object.at("rows").array.at(0);if(mode==0){auto&encoded=row.object.at("expected").value;encoded[1]=encoded[1]=='2'?'3':'2';}else{auto req=JsonParser(row.at("request").str()).read();if(mode==1)req.object.at("point").object.at("x").value=Int(integer(req.at("point").at("x"))+1).get_str();else req.object.at("marked").object.at("mu").value=Int(integer(req.at("marked").at("mu"))+1).get_str();row.object.at("request").value=canonical_json(req);}bool rejected=false;try{verify(bad,curves);}catch(const std::exception&){rejected=true;}check(rejected,"independent point corruption accepted");++negative;}
 std::cout<<"{\"independent_GMP_coordinate_replays\":"<<count<<",\"subgroup_checks\":"<<count<<",\"altered_sign_coordinate_label_rejections\":"<<negative<<",\"OpenSSL_used\":false,\"timing_performed\":false}\n";return 0;
}catch(const std::exception&e){std::cerr<<"independent point replay refused: "<<e.what()<<'\n';return 1;}}
