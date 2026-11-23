// Development resource check of already-known inputs; no point-trait analysis.
#include "../../native/common.hpp"
#include <fstream>
#include <iostream>
#include <set>
int main(int argc,char** argv) {
    require(argc==2,"prime-inputs COHORT");
    pari_init(128000000,2000000);
    std::ifstream in(argv[1]);require(bool(in),"input");
    std::string line;std::set<long> sizes;long rows=0;
    while(std::getline(in,line)) {
        if(line.empty()||line[0]=='#')continue;
        std::istringstream stream(line);std::vector<std::string> r;std::string s;
        while(std::getline(stream,s,'\t'))r.push_back(s);
        require(r.size()==9,"row width");pari_sp mark=avma;
        std::cerr<<"prime input "<<r[0]<<'\n'<<std::flush;
        require(isprime(integer(r[5]))==1&&isprime(integer(r[7]))==1,"known input primality");
        std::cout<<r[0]<<"\t1\t1\n"<<std::flush;
        sizes.insert(std::stol(r[1]));++rows;avma=mark;
    }
    require(rows==21&&sizes.size()==6,"all21 inputs and six field sizes required");
    std::cerr<<"all21 known prime-input pairs checked\n";
    pari_close();
}
