// File/format bridge from two pinned independent native mathematical receipts.
// Not read by the measured producer; no field/group/root solver is implemented here.
#include <nlohmann/json.hpp>
#include <openssl/sha.h>
#include <zlib.h>
#include <fstream>
#include <sstream>
#include <iomanip>
#include <iostream>
#include <stdexcept>
#include <filesystem>
using J=nlohmann::ordered_json;
void need(bool b,const char*s){if(!b)throw std::runtime_error(s);}
std::string read(const std::string&p){std::ifstream f(p,std::ios::binary);need(bool(f),"input");return{std::istreambuf_iterator<char>(f),{}};}
std::string sha(const std::string&s){unsigned char d[32];SHA256((const unsigned char*)s.data(),s.size(),d);std::ostringstream out;for(auto b:d)out<<std::hex<<std::setw(2)<<std::setfill('0')<<int(b);return out.str();}
int main(int argc,char**argv){try{
 need(argc==4,"references OLD OUTPUT HEAD; new cubic receipt on stdin");auto old=read(argv[1]);need(sha(old)=="9e48248cede5094609574f2ff78bf4f85901ae04a931bcd08755520f032ef95b","original source points receipt");std::string fresh{std::istreambuf_iterator<char>(std::cin),{}};need(sha(fresh)=="ff78ac4c4b2248fa571f6d141cdc839d1d017b4cdfd7c6cb3fff64367bea038c","complete prior independent cubic receipt");J previous=J::parse(old),b=J::parse(fresh),a=b;need(b["schema"]=="twist.cubic-complete.audit/v1"&&b["profiles"]==1365,"complete cubic scope");std::vector<std::pair<int,int>>points;for(const auto&m:previous["maps"])if(m[0]==3)points.push_back({m[1],m[2]});need(points.size()==31,"original source points");
 std::ostringstream out;out<<"S twist.counter-reference.v1 "<<"ff78ac4c4b2248fa571f6d141cdc839d1d017b4cdfd7c6cb3fff64367bea038c ff78ac4c4b2248fa571f6d141cdc839d1d017b4cdfd7c6cb3fff64367bea038c"<<'\n';int count=0;
 for(int method:{0,1}){const auto&doc=method==0?a:b;for(const auto&p:doc["profiles_data"]){int cap=p["cap"],route=p["route"],policy=p["policy"];if(cap!=(3))continue;need(route>=3&&route<=5&&policy>=0&&policy<455&&p["works"].size()==30,"exact profile membership");for(int qi=1;qi<31;qi++){const auto&w=p["works"][qi-1];need(w["target"]==qi,"point order");const auto&r=w["result"];out<<"E "<<method<<' '<<route<<' '<<policy<<' '<<qi<<' '<<points[qi].first<<' '<<points[qi].second;for(const char*k:{"scalar","kind","rank","setup_m","shift"})out<<' '<<r[k];for(const auto&x:r["logs"])out<<' '<<x;for(const auto&x:r["counts"])out<<' '<<x;out<<'\n';count++;}}}
 need(count==81900,"full reference row count");need(!std::filesystem::exists(argv[2]),"fresh reference only");gzFile f=gzopen(argv[2],"wb9");need(f,"gzip output");auto data=out.str();need(gzwrite(f,data.data(),data.size())==int(data.size())&&gzclose(f)==Z_OK,"complete reference output");return 0;
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}
