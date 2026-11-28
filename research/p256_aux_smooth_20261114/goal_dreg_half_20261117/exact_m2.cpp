#define main frozen_auxiliary_main
#include "../aux_base.cpp"
#undef main
using IP=std::vector<Z>;
IP product(const IP& a,const IP& b){IP c(a.size()+b.size()-1);for(size_t i=0;i<a.size();++i)for(size_t j=0;j<b.size();++j)c[i+j]+=a[i]*b[j];return c;}
IP section(unsigned d){require(d>0,"degree0");return IP(d,Z(1));}
std::string ints(const IP& a){std::vector<std::string> v;for(const auto& x:a)v.push_back(dec(x));return array(v);}
Z total(const IP& a){Z s=0;for(const auto& x:a)s+=x;return s;}
void regularity(const IP& h,unsigned D){require(!h.empty() && h.back()>0 && D==h.size(),"incorrect exact Artinian ideal regularity");for(const auto& n:h)require(n>=0,"negative Hilbert dimension");}
int main(int argc,char** argv) {
    try {
        require(argc==3,"usage: exact_m2 CACHE FRESH_OUT");std::filesystem::path out=argv[2];require(!std::filesystem::exists(out),"fresh output");
        std::istringstream in(read(argv[1]));std::string tag;unsigned n,d1,d2,e,k,tuples,accepted;
        in>>tag>>n;require(tag=="POINTS" && n==288,"points");in>>tag>>d1>>d2;require(tag=="DENSE_DEGREES" && d1==3 && d2==96,"dense CI");
        in>>tag>>k;require(tag=="CHAIN_H" && k>0 && k<100,"chain numerator");IP h(k);for(auto& x:h){std::string s;in>>s;x=Z(s);}require(total(h)==n,"wrong point numerator");
        in>>tag>>e;require(tag=="GROUP_DEGREE" && e==4,"same raw group degree");in>>tag>>tuples>>accepted;require(tag=="REFERENCE" && tuples==n*n && accepted==0,"nonzerodivisor precondition fails");in>>tag;require(tag=="END" && !(in>>tag),"cache terminator");
        IP dense_input=product(section(d1),section(d2));dense_input=product(dense_input,dense_input);
        IP chain_input=product(h,h),dense_full=product(dense_input,section(e)),chain_full=product(chain_input,section(e));
        const unsigned Db=static_cast<unsigned>(dense_full.size()),Dc=static_cast<unsigned>(chain_full.size());regularity(dense_full,Db);regularity(chain_full,Dc);
        require(total(dense_full)==total(chain_full) && total(chain_full)==Z(n)*n*e,"Artinian length mismatch");require(2*Dc<=Db,"twofold target fails");
        size_t corruptions=0;try{regularity(chain_full,Dc+1);}catch(const std::exception&){++corruptions;}
        try{require(accepted+1==0,"nonzerodivisor precondition fails");}catch(const std::exception&){++corruptions;}
        require(corruptions==2,"corrupt control accepted");
        std::vector<unsigned> degrees{d1,d2,d1,d2,e};std::map<std::pair<unsigned,unsigned>,Z> betti{{{0,0},1}};
        for(unsigned d:degrees){auto old=betti;for(const auto& [ij,count]:old)betti[{ij.first+1,ij.second+d}]+=count;}
        unsigned qreg=0;std::vector<std::string> bs;for(const auto& [ij,count]:betti){qreg=std::max(qreg,ij.second-ij.first);bs.push_back(object({{"homological_degree",num(ij.first)},{"internal_degree",num(ij.second)},{"rank",dec(count)}}));}
        require(qreg+1==Db,"Koszul shifts disagree with Hilbert certificate");
        write(out/"exact-unplanted-m2.json",object({{"factor_base",jquote("FB1h16882188194e")},{"m","2"},
          {"kind",jquote("frozen unplanted")},{"definition",jquote("ideal-module CM regularity of actual generator homogenization")},
          {"dense_input_numerator",ints(dense_input)},{"chain_input_numerator",ints(chain_input)},
          {"dense_full_Artinian_Hilbert_polynomial",ints(dense_full)},{"chain_full_Artinian_Hilbert_polynomial",ints(chain_full)},
          {"dense_full_quotient_regularity",num(Db-1)},{"dense_full_ideal_regularity",num(Db)},
          {"chain_full_quotient_regularity",num(Dc-1)},{"chain_full_ideal_regularity",num(Dc)},
          {"Artinian_length_each",dec(total(chain_full))},{"dense_minimal_Koszul_shifts",array(bs)},
          {"field_points_per_input",num(n)},{"complete_tuples_checked",num(tuples)},{"accepted_tuples",num(accepted)},
          {"regular_element_degree",num(e)},{"corrupt_controls_rejected",num(corruptions)},
          {"twofold_numeric_target","true"},{"affine_variables_dense","4"},{"affine_variables_chain","18"},
          {"homogenizing_variables_each","1"},{"membership_equations_dense","4"},{"membership_equations_chain","102"},
          {"group_equations_each","1"},{"runtime_gain_claim","false"},
          {"scope",jquote("exact unplanted m2 algebraic derivation with native dependency checks; not independent signed/Lean admission")}}));
        std::cout<<"Exact unplanted m2 full ideal regularity "<<Db<<" -> "<<Dc<<"; unsigned certificate.\n";
    }catch(const std::exception& e){std::cerr<<"exact_m2: "<<e.what()<<'\n';return 1;}
}
