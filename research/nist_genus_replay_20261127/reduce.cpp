#include "json.hpp"
#include <fstream>
#include <iostream>
#include <sstream>
#include <map>
#include <array>
#include <limits>
std::vector<std::string> csv(std::string s){if(!s.empty()&&s.back()=='\r')s.pop_back();std::stringstream ss(s);std::vector<std::string>v;while(std::getline(ss,s,','))v.push_back(s);return v;}
int value(const std::string&path,const std::string&key){std::ifstream f(path);require(bool(f),"missing "+path);std::string s;int count=0,result=0;while(std::getline(f,s))if(s.rfind(key+"=",0)==0){result=std::stoi(s.substr(key.size()+1));++count;}require(count==1&&result>0,"missing/duplicate degree in "+path);return result;}
int main(int argc,char**argv)try{
 require(argc==4,"usage: reduce models.json original-scores.csv cohort-directory");Json models=read_json(argv[1]);std::ifstream scores(argv[2]);require(bool(scores),"scores input");std::map<std::pair<std::string,std::string>,std::array<int,9>>expected;std::string s;std::getline(scores,s);while(std::getline(scores,s)){auto r=csv(s);require(r.size()>=13,"score row");std::array<int,9>a{};for(int i=0;i<9;++i)a[i]=std::stoi(r[4+i]);require(expected.emplace(std::make_pair(r[0],r[1]),a).second,"duplicate original score");}
 std::cout<<"family,candidate,status,d0,d1,d2,d3,d4,d5,d6,d7,S,exact_original_score_match\n";
 int cases=0,terms=0;std::map<std::string,std::pair<int,int>>range;
 for(auto&r:models.at("models").arr){std::string family=r.at("family").scalar,candidate=r.at("candidate").scalar,status=r.at("status").scalar;auto key=std::make_pair(family,candidate);require(expected.count(key),"missing original row");std::cout<<family<<','<<candidate<<','<<status;int sum=0;bool match=true;
  for(int d=0;d<8;++d){std::string path=std::string(argv[3])+"/"+family+"-"+candidate+"-d"+std::to_string(d)+"/";
   for(auto name:{"reconstruct","check"})require(read_json(path+name+".execution.json").at("exit").integer()==0,"failed native worker");
   int m=value(path+"check.stdout","derived_full_cover_three_torsion_field_degree");sum+=m;std::cout<<','<<m;match=match&&(m==expected.at(key)[d]);++terms;
  }
  match=match&&(sum==expected.at(key)[8]);std::cout<<','<<sum<<','<<(match?"true":"false")<<'\n';require(match,"original score mismatch");
  if(!range.count(family))range[family]={0,std::numeric_limits<int>::max()};if(status=="published")range[family].first=std::max(range[family].first,sum);else{require(status=="eligible_skip","unknown label");range[family].second=std::min(range[family].second,sum);}++cases;
 }
 require(cases==21&&terms==168,"coverage");std::cerr<<"{\"curves\":"<<cases<<",\"terms\":"<<terms<<",\"ranges\":[";bool first=true;for(auto&[field,r]:range){if(!first)std::cerr<<',';first=false;std::cerr<<"{\"field\":\""<<field<<"\",\"max_published\":"<<r.first<<",\"min_omitted\":";if(r.second==std::numeric_limits<int>::max())std::cerr<<"null";else std::cerr<<r.second;std::cerr<<'}';}std::cerr<<"]}\n";return 0;
}catch(std::exception&e){std::cerr<<e.what()<<'\n';return 1;}
