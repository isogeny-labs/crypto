#include "io.hpp"
#include <cstdint>
#include <iomanip>
#include <iostream>
static uint64_t number(const std::string& json,const std::string& name) {
    auto at=json.find("\""+name+"\":");require(at!=std::string::npos,"summary key");at+=name.size()+3;
    auto end=json.find_first_not_of("0123456789",at);require(end>at,"summary integer");
    return std::stoull(json.substr(at,end-at));
}
int main(int argc,char** argv) {
try {
    require(argc==3,"summarize VERIFIED_RESULTS NEW_OUTPUT");fs::path input=argv[1],out=argv[2];require(!fs::exists(out),"fresh summary output");
    auto s=read_file(input/"summary.json");auto fields=table(input/"fields.tsv",9);require(fields.size()==5,"field rows");
    auto tail=number(s,"tail_at_least_observed"),denom=number(s,"assignments"),score=number(s,"observed_score_twice");
    uint64_t wins=0,ties=0,losses=0;std::ostringstream macros,table_out;
    table_out << "\\begin{tabular}{rrrrrrr}\\toprule\nField bits & Published & Additional & Pairs & Higher & Tied & Lower\\\\\\midrule\n";
    for(size_t i=1;i<fields.size();++i) {
        const auto& f=fields[i];wins+=std::stoull(f[4]);ties+=std::stoull(f[5]);losses+=std::stoull(f[6]);
        table_out << f[0]<<" & "<<f[1]<<" & "<<f[2]<<" & "<<f[3]<<" & "<<f[4]<<" & "<<f[5]<<" & "<<f[6]<<"\\\\\n";
    }
    require(2*wins+ties==score&&wins+ties+losses==102,"verified pair accounting");
    table_out << "\\midrule Total & 8 & 42 & 102 & "<<wins<<" & "<<ties<<" & "<<losses<<"\\\\\\bottomrule\\end{tabular}\n";
    macros << "\\newcommand{\\TailNumerator}{"<<tail<<"}\n\\newcommand{\\TailDenominator}{"<<denom<<"}\n";
    macros << std::fixed<<std::setprecision(9)<<"\\newcommand{\\TailDecimal}{"<<static_cast<long double>(tail)/denom<<"}\n";
    macros << "\\newcommand{\\PairFraction}{"<<static_cast<long double>(score)/204<<"}\n";
    macros << "\\newcommand{\\HigherCount}{"<<wins<<"}\n\\newcommand{\\TieCount}{"<<ties<<"}\n\\newcommand{\\LowerCount}{"<<losses<<"}\n";
    fs::create_directories(out);put(out/"numbers.tex",macros.str());put(out/"fields.tex",table_out.str());
    std::cout << "Exact tail "<<tail<<'/'<<denom<<" = "<<std::fixed<<std::setprecision(9)<<static_cast<long double>(tail)/denom<<'\n';
    return 0;
}catch(const std::exception& e){std::cerr << "FAIL: "<<e.what()<<'\n';return 1;}
}
