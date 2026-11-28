// Native full-pipeline interface. Datum construction/validation stays visible.
#include "isogeny.hpp"
#include "pullback.hpp"
#include <flint/ulong_extras.h>
#include <array>
#include <fstream>
#include <iostream>
#include <optional>
#include <stdexcept>
#include <string>

namespace {
using namespace paper_ec;
Word mul(Word a, Word b, Word p) { return static_cast<Word>((__uint128_t(a) * b) % p); }
Word power(Word a, Word n, Word p) {
    Word out = 1; while (n) { if (n & 1) out = mul(out,a,p); a=mul(a,a,p); n>>=1; } return out;
}
void require(bool okay, const std::string& message) { if (!okay) throw std::invalid_argument(message); }
Datum read_datum(const std::string& path) {
    std::ifstream file(path); Datum d;
    require(bool(file >> d.curve.p >> d.curve.a >> d.curve.b >> d.beta >> d.order >> d.twist_order), "cannot read datum header");
    const auto p=d.curve.p;
    require(p>3 && p<(Word(1)<<63) && n_is_prime(p) && d.beta>0 && d.beta<p, "invalid datum field");
    const Word inv=power(d.beta,p-2,p),inv2=mul(inv,inv,p);
    d.twist={p,mul(d.curve.a,inv2,p),mul(d.curve.b,mul(inv,inv2,p),p)};
    for (auto* factors : {&d.factors,&d.twist_factors}) {
        std::size_t size=0; require(bool(file>>size)&&size<=64,"invalid factor count");
        for(std::size_t i=0;i<size;++i){PrimePower factor;require(bool(file>>factor.prime>>factor.exponent),"invalid factorization");factors->push_back(factor);}
    }
    for (auto* basis : {&d.basis,&d.twist_basis}) {
        const std::array<std::pair<Point*,Word*>,2> entries{{
            {&basis->first,&basis->first_order},{&basis->second,&basis->second_order}}};
        for (const auto& pair : entries) {
            unsigned infinity=0;
            require(bool(file>>infinity>>pair.first->x>>pair.first->y>>*pair.second)&&infinity<=1,"invalid basis record");
            pair.first->infinity=bool(infinity);
        }
    }
    std::string extra;require(!(file>>extra),"extra datum tokens");return d;
}
Coefficients read_polynomial(const std::string& path) {
    std::ifstream file(path);std::size_t degree=0;
    require(bool(file>>degree)&&degree<=1000000,"invalid polynomial degree or exceeds CLI capacity");
    Coefficients coefficients(degree+1);
    for(auto& value:coefficients)require(bool(file>>value),"missing polynomial coefficient");
    std::string extra;require(!(file>>extra),"extra polynomial tokens");return coefficients;
}
void counts(const Stats& s) {
    std::cout<<'{';bool first=true;for(const auto& entry:s.counts){if(!first)std::cout<<',';first=false;std::cout<<'"'<<entry.first<<"\":"<<entry.second;}std::cout<<'}';
}
void run(int argc,char** argv) {
    require(argc==4||argc==5,"usage: rootfinder pullback|trace|dense DATUM POLYNOMIAL [--two-isogeny]");
    const std::string backend=argv[1];
    require(backend=="pullback"||backend=="trace"||backend=="dense","unknown backend");
    require(argc==4||std::string(argv[4])=="--two-isogeny","unknown option");
    Stats setup,online;Datum datum=read_datum(argv[2]);const auto input=read_polynomial(argv[3]);
    std::optional<PreparedDatum> prepared;prepared.emplace(datum,&setup);
    if(argc==5){
        Point kernel;
        const std::array<std::pair<Point,Word>,2> generators{{
            {datum.basis.first,datum.basis.first_order},{datum.basis.second,datum.basis.second_order}}};
        for(const auto& item:generators)
            if(!(item.second&1)){kernel=point_mul(datum.curve,item.second/2,item.first,&setup);break;}
        require(!kernel.infinity,"no rational two-torsion kernel in supplied basis");
        const auto map=two_isogeny(datum.curve,kernel,&setup);
        // The mapped generators need not be a full codomain basis. Complete
        // a NEW datum explicitly, then re-embed the original polynomial.
        datum=construct_toy_datum(map.codomain,datum.beta,&setup);
        prepared.emplace(datum,&setup);
    }
    const auto roots=backend=="pullback" ? root_find_pullback(*prepared,input,&online)
        : root_find(*prepared,input,backend=="trace"?TranslationMethod::Trace:TranslationMethod::Dense,&online);
    std::cout<<"{\"schema\":\"paper-native-rootfinder/v1\",\"pcc_standing\":\"unverified\",\"backend\":\""<<backend
        <<"\",\"timings_collected\":false,\"isogeny_preprocessing\":"<<(argc==5?"true":"false")
        <<",\"auxiliary_curve\":{\"p\":"<<datum.curve.p<<",\"a\":"<<datum.curve.a<<",\"b\":"<<datum.curve.b
        <<",\"order\":"<<datum.order<<",\"twist_order\":"<<datum.twist_order<<"},\"roots\":[";
    for(std::size_t i=0;i<roots.size();++i){if(i)std::cout<<',';std::cout<<roots[i];}
    std::cout<<"],\"preprocessing_counts\":";counts(setup);
    std::cout<<",\"online_counts\":";counts(online);
    std::cout<<",\"scope\":\"Native root-finding with supplied datum; optional isogeny datum construction is exhaustive and capped at p257. No ECDLP timing ratio is asserted.\"}\n";
}
}
int main(int argc,char** argv) {
    try { run(argc,argv); return 0; }
    catch(const std::exception& error){std::cerr<<error.what()<<'\n';return 2;}
}
