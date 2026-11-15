#pragma once
#define QUOTIENT_EXTRACT_NO_MAIN
#include "../shared_norm_algebraic_20261115/quotient_extract.cpp"
#include <cstdlib>
namespace mca {
using canonical::U;using canonical::Monomial;using canonical::Polynomial;using canonical::System;
constexpr std::size_t row_cap=20000,entry_cap=2000000,step_cap=1000000,basis_cap=128,cert_cap=1048576;
inline void require(bool ok,const std::string&s){if(!ok)throw std::runtime_error(s);}
inline Polynomial multiple(const Polynomial&f,const Monomial&m,U c=1){
    Polynomial term(f.n,f.p);term.insert(m,c);return f.times(term);
}
inline Polynomial normal_form(Polynomial f,const std::vector<Polynomial>&basis,std::size_t&steps){
    Polynomial remainder(f.n,f.p);
    while(!f.terms.empty()){
        require(++steps<=step_cap,"normal-form step cap");const auto lm=quotient::lead(f);bool reduced=false;
        for(const auto&g:basis){if(g.terms.empty())continue;const auto gm=quotient::lead(g);
            if(!quotient::divides(gm,lm))continue;
            Monomial multiplier(lm.size());for(std::size_t i=0;i<lm.size();++i)multiplier[i]=lm[i]-gm[i];
            const U c=f.terms.at(lm)*quotient::power(g.terms.at(gm),f.p-2,f.p)%f.p;
            f=f.plus(multiple(g,multiplier,c),true);reduced=true;break;
        }
        if(!reduced){remainder.insert(lm,f.terms.at(lm));f.terms.erase(lm);}
    }
    return remainder;
}
inline Polynomial s_polynomial(const Polynomial&f,const Polynomial&g){
    const auto fm=quotient::lead(f),gm=quotient::lead(g);Monomial a(f.n),b(f.n);
    for(std::size_t i=0;i<f.n;++i){const unsigned e=std::max(fm[i],gm[i]);a[i]=e-fm[i];b[i]=e-gm[i];}
    return multiple(f,a,quotient::power(f.terms.at(fm),f.p-2,f.p))
        .plus(multiple(g,b,quotient::power(g.terms.at(gm),f.p-2,f.p)),true);
}
inline void write_basis(const std::filesystem::path&path,const System&s,const std::vector<Polynomial>&g){
    std::ofstream out(path);require(bool(out),"basis output open");out<<"#field characteristic: "<<s.p<<"\n#variable order: ";
    for(std::size_t i=0;i<s.names.size();++i){if(i)out<<',';out<<s.names[i];}
    out<<"\n#monomial order: graded reverse lexicographical\n[";
    for(std::size_t i=0;i<g.size();++i){if(i)out<<",\n";out<<g[i].serialize(s.names,false);}out<<"]:\n";
    out.close();require(bool(out),"basis write failed");
}
struct RowDefinition{std::size_t equation;Monomial multiplier;};
} // namespace mca
