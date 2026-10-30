#include "io.hpp"
#include <algorithm>
#include <iomanip>
#include <iostream>
#include <map>
struct Seed {
    std::string id,role,category;
    int bits,s,r,s5;
};
static char code(const std::string& s) {
    if (s == "published") return 'P';
    if (s == "retrospective_omission") return 'N';
    if (s == "order_too_wide_only") return 'W';
    require(s == "opposite_root_only","category");return 'O';
}
int main(int argc,char** argv) {
try {
    require(argc == 3,"summarize VERIFIED_RESULTS FRESH_OUTPUT");
    const fs::path input=argv[1],out=argv[2];require(!fs::exists(out),"fresh output");
    auto branches=table(input/"branches.tsv",14),cuts=table(input/"cutoffs.tsv",10),
         violations=table(input/"at691-violations.tsv",6);
    require(branches.size()==83 && cuts.size()==4902,"raw inventories");
    std::vector<Seed> seeds;
    for (size_t i=1;i<branches.size();++i) {
        const auto& c=branches[i];
        if(c[6]=="1") seeds.push_back({c[1],c[3],c[4],std::stoi(c[2]),std::stoi(c[7]),std::stoi(c[8]),std::stoi(c[10])});
    }
    require(seeds.size()==69,"selected seed inventory");
    std::ostringstream fields,ft,vt,ct,all,categories;
    fields << "bits\tpublished\tunpublished\tpub_min691\tpub_max691\tunpub_min691\tunpub_max691\tmargin691\tpositive_cutoffs\tviolators691\n";
    ft << "\\begin{tabular}{rrrrrrrr}\\toprule\nBits & Pub. & Unpub. & Pub. $S_{691}$ & Unpub. $S_{691}$ & Margin & Pos. cutoffs & Violators\\\\\\midrule\n";
    std::map<std::string,std::pair<int,int>> grouped;
    for (size_t field=0;field<6;++field) {
        int bits=std::vector<int>{192,224,239,256,384,521}[field];
        int np=0,nu=0,pmin=10000,pmax=-1,umin=10000,umax=-1,bad=0,positive=0;
        for(const auto& s:seeds)if(s.bits==bits) {
            if(s.role=="published") {++np;pmin=std::min(pmin,s.s);pmax=std::max(pmax,s.s);}
            else {++nu;umin=std::min(umin,s.s);umax=std::max(umax,s.s);}
        }
        for(const auto& s:seeds)if(s.bits==bits && s.role!="published")bad+=s.s<=pmax;
        if(nu)for(size_t i=1;i<cuts.size();++i)positive+=std::stoi(cuts[i][5+field])>0;
        fields << bits << '\t' << np << '\t' << nu << '\t' << pmin << '\t' << pmax << '\t';
        if(nu)fields << umin << '\t' << umax << '\t' << umin-pmax << '\t' << positive << '\t' << bad << '\n';
        else fields << "NA\tNA\tNA\tNA\tNA\n";
        ft << bits << " & " << np << " & " << nu << " & " << pmin << "--" << pmax << " & ";
        if(nu)ft << umin << "--" << umax << " & " << umin-pmax << " & " << positive << "/4901 & " << bad;
        else ft << "--- & --- & --- & ---";
        ft << "\\\\\n";
    }
    ft << "\\bottomrule\\end{tabular}\n";
    for(const auto& s:seeds)if(s.role!="published") {
        auto& g=grouped[s.category];++g.first;
        for(size_t i=1;i<violations.size();++i)if(violations[i][0]==s.id)++g.second;
    }
    categories << "category\tunpublished_positions\tviolators691\n";
    ct << "\\begin{tabular}{lrr}\\toprule\nRecorded category & Positions & Violators at691\\\\\\midrule\n";
    for(const auto& g:grouped) {
        categories << g.first << '\t' << g.second.first << '\t' << g.second.second << '\n';
        std::string label=g.first=="opposite_root_only" ? "Other-root-only" :
                          g.first=="order_too_wide_only" ? "Order-width-only" : "Named omissions";
        ct << label << " & " << g.second.first << " & " << g.second.second << "\\\\\n";
    }
    ct << "\\bottomrule\\end{tabular}\n";
    vt << "\\begin{tabular}{lcrrr}\\toprule\nSeed position & Category & $S_{691}$ & Pub. maximum & Difference\\\\\\midrule\n";
    for(size_t i=1;i<violations.size();++i) {
        const auto& v=violations[i];
        vt << v[0] << " & " << code(v[2]) << " & " << v[3] << " & " << v[4] << " & " << v[5] << "\\\\\n";
    }
    vt << "\\bottomrule\\end{tabular}\n";
    for(size_t half=0;half<2;++half) {
        if(half)all << "\\hfill\n";
        all << "\\begin{minipage}[t]{0.48\\linewidth}\\centering\n\\begin{tabular}{lcrrr}\\toprule\nSeed position & Cat. & $S_{691}$ & $R_{691}$ & $S_{5000}$\\\\\\midrule\n";
        size_t begin=half?35:0,end=half?seeds.size():35;
        for(size_t i=begin;i<end;++i) {
            const auto& s=seeds[i];
            all << s.id << " & " << code(s.category) << " & " << s.s << " & " << s.r << " & " << s.s5 << "\\\\\n";
        }
        all << "\\bottomrule\\end{tabular}\\end{minipage}\n";
    }
    fs::create_directories(out);
    put(out/"field-coverage.tsv",fields.str());put(out/"categories.tsv",categories.str());
    put(out/"fields.tex",ft.str());put(out/"categories.tex",ct.str());
    put(out/"violators.tex",vt.str());put(out/"all-seeds.tex",all.str());
    std::cout << "Summarized verified native records without changing the fixed criterion\n";
    return 0;
}catch(const std::exception& e){std::cerr << "FAIL: " << e.what() << '\n';return 1;}
}
