#include <map>
#include <array>
#include <vector>
// Format bridge only. Numerical receipts come from the independently executed native math auditor.
#include <nlohmann/json.hpp>
#include <openssl/sha.h>
#include <zlib.h>
#include <fstream>
#include <sstream>
#include <iomanip>
#include <iostream>
#include <filesystem>
using J=nlohmann::ordered_json;
void need(bool b,const char*s){if(!b)throw std::runtime_error(s);}
std::string read(const std::string&p){std::ifstream f(p,std::ios::binary);need(bool(f),"input");return{std::istreambuf_iterator<char>(f),{}};}
std::string sha(const std::string&s){unsigned char d[32];SHA256((const unsigned char*)s.data(),s.size(),d);std::ostringstream out;for(auto b:d)out<<std::hex<<std::setw(2)<<std::setfill('0')<<int(b);return out.str();}
int main(int argc,char**argv){try{need(argc==4,"reference original-point-receipt output head; three native math JSONs on stdin");auto raw=read(argv[1]);need(sha(raw)=="9e48248cede5094609574f2ff78bf4f85901ae04a931bcd08755520f032ef95b","original source binding");J docs[3];for(auto&d:docs)std::cin>>d;need(docs[0]["schema"]=="twist.first-row.audit/v1"&&docs[0]["head"]=="0a181f00465046b119ad33b932b012decb81f983"&&docs[1]["schema"]=="twist.setup-rank.audit/v1"&&docs[1]["head"]=="86449333fac19b62ada66e2e63980b10d3253b90"&&docs[2]["schema"]=="twist.two-base.audit/v1"&&docs[2]["head"]==argv[3],"exact historical math/source plus fresh B2 head");
J previous=J::parse(raw);std::vector<std::pair<int,int>>points;for(const auto&m:previous["maps"])if(m[0]==3)points.push_back({m[1],m[2]});need(points.size()==31,"original point order");std::ostringstream out;out<<"S twist.counter-reference.two-base.v1\n";int count=0;
for(int method=0;method<3;method++){need(docs[method]["profiles"]==(method==2?315:1365),"full declared mathematical population");for(const auto&p:docs[method]["profiles_data"]){int cap=p["cap"],route=p["route"],policy=p["policy"];need(cap==(method==2?2:3)&&route>=3&&route<=5&&policy>=0&&policy<(method==2?105:455)&&p["works"].size()==30,"exact base cardinality/cap/domain");for(int qi=1;qi<31;qi++){const auto&w=p["works"][qi-1];need(w["target"]==qi,"actual point order");const auto&r=w["result"];need(r["logs"].size()==(method==2?2:3),"actual public-log dimension");out<<"E "<<method<<' '<<route<<' '<<policy<<' '<<qi<<' '<<points[qi].first<<' '<<points[qi].second;for(const char*k:{"scalar","kind","rank","setup_m","shift"})out<<' '<<r[k];for(const auto&x:r["logs"])out<<' '<<x;if(method==2)out<<" 0";for(const auto&x:r["counts"])out<<' '<<x;out<<'\n';count++;}}}
need(count==91350,"all B3+B2 reference rows, zero padding is FORMAT only, not a third B2 unknown");need(!std::filesystem::exists(argv[2]),"fresh output");auto data=out.str();gzFile f=gzopen(argv[2],"wb9");need(f&&gzwrite(f,data.data(),data.size())==int(data.size())&&gzclose(f)==Z_OK,"complete gzip reference");return 0;
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}
