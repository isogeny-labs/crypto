#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
namespace fs=std::filesystem;
static void need(bool ok,const std::string& msg){if(!ok)throw std::runtime_error(msg);}
static std::string read(const fs::path& p){std::ifstream f(p);need(bool(f),"missing "+p.string());return {std::istreambuf_iterator<char>(f),{}};}
static std::vector<std::string> split(const std::string& s){std::istringstream f(s);std::vector<std::string> v;std::string x;while(std::getline(f,x,'\t'))v.push_back(x);return v;}
static int observed_return(const std::string& bytes,const std::string& root,const std::string& method){
 const std::string prefix="CONFIG long=4 double=8 charbits=8 NBITS=26 DOUBLE_WORDS=low-high\nINPUT ";
 if(bytes.rfind(prefix,0)!=0)return -1;
 size_t pos=prefix.size();std::string reported;bool line_digit=false,terminated=false;
 while(pos<bytes.size()){
  const char ch=bytes[pos++];
  if(ch>='0'&&ch<='9'){reported+=ch;line_digit=true;if(reported.size()>root.size())return -1;}
  else if(ch=='\\'){
   if(!line_digit||pos>=bytes.size()||bytes[pos++]!='\n'||pos>=bytes.size()||bytes[pos]<'0'||bytes[pos]>'9')return -1;
   line_digit=false;
  }else if(ch=='\n'){terminated=true;break;}else return -1;
 }
 if(!terminated||!line_digit||reported!=root)return -1;
 const std::string tail=bytes.substr(pos),result="RESULT "+method+" ";
 if(tail==result+"1\n")return 1;
 if(tail==result+"0\n")return 0;
 return -1;
}
static void parser_controls(){
 const std::string p="CONFIG long=4 double=8 charbits=8 NBITS=26 DOUBLE_WORDS=low-high\nINPUT ";
 const std::string wrapped=p+"1234\\\n567890\nRESULT probprime 1\n";
 need(observed_return(wrapped,"1234567890","probprime")==1,"wrapped positive");
 need(observed_return(p+"1234567890\nRESULT probprime 0\n","1234567890","probprime")==0,"zero return grammar");
 need(observed_return(p+"1234567890\nRESULT ordinary 1\n","1234567890","ordinary")==1,"unwrapped positive");
 for(const auto& bad:std::vector<std::string>{wrapped+"RESULT probprime 1\n",p+"1234\\X567890\nRESULT probprime 1\n",p+"1234\\\n\\\n567890\nRESULT probprime 1\n",p+"1234567890\nRESULT probprime 2\n",p+"1234567890\nRESULT ordinary 1\n",p+"1234567890\nRESULT probprime 1"})
  need(observed_return(bad,"1234567890","probprime")==-1,"malformed control accepted");
 need(observed_return(wrapped,"9234567890","probprime")==-1,"wrong root accepted");
 std::cout<<"Three positive grammar cases and seven malformed/root-bound controls passed.\n";
}
int main(int argc,char**argv){try{
 if(argc==2&&std::string(argv[1])=="selftest"){parser_controls();return 0;}
 need(argc==3,"usage: summarize RUN OUT");const fs::path run=argv[1],out=argv[2];
 std::istringstream input(read(run/"cohort.tsv"));std::string line;need(bool(std::getline(input,line)),"empty cohort");
 need(line=="id\tlabel\tbits\trole\torder_decimal","cohort schema");std::vector<std::vector<std::string>> rows;std::set<std::string> ids,labels;long published=0,skipped=0;
 while(std::getline(input,line)){auto r=split(line);need(r.size()==5&&ids.insert(r[0]).second&&labels.insert(r[1]).second,"cohort identity");need(r[0].rfind("curve-",0)==0,"case path");need(r[0].find('/')==std::string::npos,"invalid case path");need(r[3]=="published"||r[3]=="skipped","role");published+=r[3]=="published";skipped+=r[3]=="skipped";rows.push_back(r);}
 need(rows.size()==21&&published==10&&skipped==11,"complete cohort required");
 std::ofstream table(out/"cohort.tsv"),summary(out/"summary.json");need(bool(table)&&bool(summary),"output open");
 table<<"version\tmethod\tlabel\tbits\trole\tclassification\n";
 summary<<"{\"kind\":\"native-freelip-fixed-cohort/v1\",\"distinct_orders\":21,\"planned_calls\":84,\"configurations\":[";bool first=true;
 for(const std::string version:{"1.0","1.1"})for(const std::string method:{"probprime","ordinary"}){
  std::map<std::string,long> accepted,rejected,unknown;
  for(const auto&r:rows){const fs::path d=run/("v"+version+"-"+method)/r[0];need(read(d/"input")==r[4]+"\n","input binding");std::string classification="INDETERMINATE";
   if(fs::exists(d/"process.status")){std::istringstream status(read(d/"process.status"));int code;std::string extra;
    if((status>>code)&&!(status>>extra)&&code==0&&fs::exists(d/"stdout")){
     const int result=observed_return(read(d/"stdout"),r[4],method);
     if(result==1)classification="ACCEPTED";else if(result==0)classification="REJECTED_SOFTWARE";
    }
   }
   if(classification=="ACCEPTED")++accepted[r[3]];else if(classification=="REJECTED_SOFTWARE")++rejected[r[3]];else ++unknown[r[3]];
   table<<version<<'\t'<<method<<'\t'<<r[1]<<'\t'<<r[2]<<'\t'<<r[3]<<'\t'<<classification<<'\n';
  }
  if(!first) { summary<<','; }
  first=false;summary<<"{\"version\":\""<<version<<"\",\"method\":\""<<method<<"\",\"roles\":{";bool first_role=true;
  for(const std::string role:{"published","skipped"}){if(!first_role)summary<<',';first_role=false;summary<<'"'<<role<<"\":{\"accepted\":"<<accepted[role]<<",\"rejected_software\":"<<rejected[role]<<",\"indeterminate\":"<<unknown[role]<<'}';}summary<<"}}";
 }
 summary<<"],\"historical_use_established\":false,\"measured_dlp\":false}\n";std::cout<<"All84 configured observations retained;21 distinct orders, unknowns separate.\n";
 return 0;
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
