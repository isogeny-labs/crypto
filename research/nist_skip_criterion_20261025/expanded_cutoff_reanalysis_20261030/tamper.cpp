// Byte-integrity negative controls; no numerical curve computation.
#include <filesystem>
#include <fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
namespace fs=std::filesystem;
static void need(bool b,const std::string& s){if(!b)throw std::runtime_error(s);}
int main(int argc,char** argv){try{need(argc==4,"tamper KIND GOOD FRESH_BAD");std::string kind=argv[1],name;need(!fs::exists(argv[3]),"fresh negative-control directory");if(kind=="character")name="characters.tsv";else if(kind=="calibration")name="calibration.tsv";else if(kind=="summary")name="summary.json";else throw std::runtime_error("control kind");fs::copy(argv[2],argv[3],fs::copy_options::recursive);fs::path target=fs::path(argv[3])/name;std::ifstream in(target);need(bool(in),"control input");std::ostringstream stream;stream<<in.rdbuf();std::string s=stream.str();size_t pos=0;if(kind=="character"){pos=s.find("\t3\t");need(pos!=std::string::npos,"odd-prime row");pos+=3;size_t end=s.find('\t',pos);need(end!=std::string::npos,"character cell");s.replace(pos,end-pos,"9");}else if(kind=="calibration"){pos=s.find('\n');need(pos!=std::string::npos,"header");pos=s.find('\t',pos+1);need(pos!=std::string::npos&&pos+1<s.size(),"calibration cell");++pos;need(s[pos]=='0'||s[pos]=='1',"binary flag");s[pos]=s[pos]=='0'?'1':'0';}else{std::string key="\"strict_search_success_assignments\":";pos=s.find(key);need(pos!=std::string::npos,"summary key");pos+=key.size();need(pos<s.size()&&s[pos]>='0'&&s[pos]<='9',"summary digit");s[pos]=s[pos]=='9'?'8':'9';}std::ofstream out(target,std::ios::trunc);need(bool(out),"control output");out<<s;need(bool(out),"control write");std::cout<<"Changed one "<<kind<<" field; independent verification must refuse\n";return 0;}catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}}
