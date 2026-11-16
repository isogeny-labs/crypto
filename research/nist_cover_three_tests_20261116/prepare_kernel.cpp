// Guarded source derivation for recorded kernels; arithmetic originals stay immutable.
#include "json.hpp"
#include <filesystem>
#include <iostream>
namespace fs=std::filesystem;
std::string read_text(const fs::path& p){std::ifstream in(p);require(bool(in),"source input");return {std::istreambuf_iterator<char>(in),std::istreambuf_iterator<char>()};}
void write_text(const fs::path& p,const std::string& s){require(!fs::exists(p),"derived source refuses overwrite");std::ofstream out(p);require(bool(out),"derived source output");out<<s;}
void replace_once(std::string& text,const std::string& old,const std::string& replacement){auto p=text.find(old);require(p!=std::string::npos&&text.find(old,p+old.size())==std::string::npos,"unique original selector block");text.replace(p,old.size(),replacement);}
void identity_strings(std::string& s,int index){std::string old="kernel1",replacement="kernel"+std::to_string(index);size_t p=0;while((p=s.find(old,p))!=std::string::npos){s.replace(p,old.size(),replacement);p+=replacement.size();}}
int main(int argc,char** argv){try{
    require(argc==4,"usage: prepare_kernel original-study NEW-derived-study 2..8");fs::path original=argv[1],derived=argv[2];int index=std::stoi(argv[3]);require(index>=2&&index<=8,"recorded kernel range");require(fs::is_directory(derived),"created derived study");
    auto witness=original/"evidence/final-kernel-check/kernels.jsonl";std::ifstream rows(witness);require(bool(rows),"kernel rows input");std::string line,chosen;int matches=0;
    while(std::getline(rows,line)){Parser parser{line};auto row=parser.value();if(row.at("kernel").integer()==index){chosen=line;++matches;}}
    require(matches==1,"unique recorded selected kernel");write_text(derived/"selected-kernel.json",chosen+"\n");
    const std::string old="    std::ifstream in(root+\"/evidence/final-kernel-check/kernels.jsonl\");std::string line;\n    require(bool(std::getline(in,line)),\"first kernel input\");Parser parser{line};auto row=parser.value();\n    require(row.at(\"kernel\").integer()==1,\"fixed kernel1\");Basis coordinates;";
    const std::string replacement="    std::ifstream in(root+\"/evidence/final-kernel-check/kernels.jsonl\");std::string line;Json row;int matches=0;\n    while(std::getline(in,line)){Parser parser{line};auto candidate=parser.value();if(candidate.at(\"kernel\").integer()=="+std::to_string(index)+"){row=candidate;++matches;}}\n    require(matches==1,\"unique recorded kernel selector\");Basis coordinates;";
    int files=0;for(auto& item:fs::directory_iterator(original)){if(!item.is_regular_file())continue;auto ext=item.path().extension().string();if(ext!=".cpp"&&ext!=".hpp"&&ext!=".sh")continue;auto source=read_text(item.path());if(item.path().filename()=="weil3.cpp")replace_once(source,old,replacement);if(item.path().filename()=="export_candidate.cpp")replace_once(source,"\\\"kernel\\\":1","\\\"kernel\\\":"+std::to_string(index));identity_strings(source,index);auto output=derived/item.path().filename();write_text(output,source);if(ext==".sh")fs::permissions(output,fs::perms::owner_exec|fs::perms::group_exec|fs::perms::others_exec,fs::perm_options::add);++files;}
    std::cout<<"{\"scope\":\"recorded-kernel-source-derivation\",\"kernel\":"<<index<<",\"files\":"<<files<<",\"arithmetic_formula_changes\":false,\"standing\":\"producer-unverified\"}\n";
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
