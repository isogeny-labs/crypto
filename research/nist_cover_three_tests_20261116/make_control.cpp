// Deterministic byte-level negative fixtures; preserves exact large integer tokens.
#include "json.hpp"
#include <regex>
#include <iostream>
int main(int argc,char**argv){try{require(argc==4,"usage: make_control kind input output");std::ifstream in(argv[2]);require(bool(in),"control input");std::stringstream ss;ss<<in.rdbuf();std::string s=ss.str(),kind=argv[1],pattern,replacement;
    if(kind=="count"){pattern="\"kernel_counts\"\\s*:\\s*\\[[^\\]]+\\]";replacement="\"kernel_counts\":[999,999,999]";}
    else if(kind=="frobenius"||kind=="form"){pattern="\""+std::string(kind=="form"?"E_Weil":"E_Frobenius")+"\"\\s*:\\s*\\[\\s*\\[[^\\]]*\\],\\s*\\[[^\\]]*\\]\\s*\\]";replacement="\""+std::string(kind=="form"?"E_Weil":"E_Frobenius")+"\":[[0,0],[0,0]]";}
    else if(kind=="pairing"){pattern="\"primitive_third_root_coefficients\"\\s*:\\s*\\[[^\\]]+\\]";replacement="\"primitive_third_root_coefficients\":[1]";}
    else if(kind=="point"){pattern="(\"basis_Mumford_coefficients\"\\s*:\\s*\\[\\s*\\[\\s*\\[\\s*\\[\\s*)[0-9]+";std::smatch m;require(std::regex_search(s,m,std::regex(pattern)),"point control target");s.replace(size_t(m.position()),size_t(m.length()),m[1].str()+"0");pattern.clear();}
    else if(kind=="curve"){pattern="\"b\"\\s*:\\s*\"[0-9]+\"";replacement="\"b\":\"0\"";}
    else if(kind=="missing"){s="{\"modules\":[]}";}
    else if(kind=="kernel"){s="{\"rref_basis\":[[1,0,0,0,0,0],[0,1,0,0,0,0],[0,0,1,0,0,0]]}\n";}
    else if(kind=="duplicate"){size_t m=s.find("\"modules\""),a=s.find('[',m),start=s.find('{',a);require(start!=std::string::npos,"duplicate target");int depth=0;size_t end=start;for(;end<s.size();++end){if(s[end]=='{')++depth;if(s[end]=='}'&&--depth==0)break;}require(end<s.size(),"duplicate balance");s.insert(start,s.substr(start,end-start+1)+",");}
    else throw std::runtime_error("unknown control");
    if(!pattern.empty()){std::regex r(pattern);require(std::regex_search(s,r),"control pattern missing");s=std::regex_replace(s,r,replacement,std::regex_constants::format_first_only);}
    std::ofstream out(argv[3]);require(bool(out),"control output");out<<s;
}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
