// Native adverse-transcript writer. Does not participate in the replayer.
#include <gmpxx.h>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
static void need(bool b,const char*s){if(!b)throw std::runtime_error(s);}
static std::vector<std::string> split(const std::string&s,char c){std::istringstream in(s);std::vector<std::string>v;std::string x;while(std::getline(in,x,c))v.push_back(x);return v;}
static std::string join(const std::vector<std::string>&v,char c){std::string s;for(const auto&x:v){if(!s.empty())s+=c;s+=x;}return s;}
int main(int argc,char**argv){
    try{
        need(argc==4,"usage: mutate SOURCE_ROOT NEW_ROOT KIND");
        const std::filesystem::path source(argv[1]),dest(argv[2]);const std::string kind(argv[3]);
        need(!std::filesystem::exists(dest),"reused adverse directory");
        std::ifstream in(source/"events.txt");need(bool(in),"missing source transcript");std::vector<std::string>lines;std::string s;
        while(std::getline(in,s))lines.push_back(s);
        need(!lines.empty(),"empty source transcript");
        const auto header=split(lines[0],' ');need(header.size()==4,"source header");const mpz_class p(header[1]);bool changed=false;
        for(std::string&line:lines){
            auto row=split(line,' ');
            if(kind=="root"&&row.size()==7&&row[0]=="Q"&&row[3]!="0"){
                auto roots=split(row[4],',');need(!roots.empty(),"source roots");roots.pop_back();row[3]=std::to_string(roots.size());row[4]=roots.empty()?"-":join(roots,',');changed=true;
            }else if(kind=="curve"&&row.size()==12&&row[0]=="S"){
                mpz_class a(row[6]);a=(a+1)%p;row[6]=a.get_str();changed=true;
            }else if(kind=="numerator"&&row.size()==2&&row[0]=="N"){
                auto coeff=split(row[1],',');mpz_class a(coeff[0]);a=(a+1)%p;coeff[0]=a.get_str();row[1]=join(coeff,',');changed=true;
            }else if(kind=="rng"&&row.size()==7&&row[0]=="D"){
                auto words=split(row[4],',');const auto raw=std::stoull(words[0]);words[0]=std::to_string(raw^1ULL);row[4]=join(words,',');changed=true;
            }
            if(changed){line=join(row,' ');break;}
        }
        need(changed,"requested mutation missing");need(std::filesystem::create_directory(dest),"adverse directory creation");
        std::ofstream out(dest/"events.txt");for(const auto&line:lines)out<<line<<'\n';out.close();need(bool(out),"adverse output write");
        need(std::filesystem::copy_file(source/"setup.tsv",dest/"setup.tsv"),"adverse receipt copy");std::cout<<"WROTE "<<kind<<" adverse transcript\n";return 0;
    }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}
}
