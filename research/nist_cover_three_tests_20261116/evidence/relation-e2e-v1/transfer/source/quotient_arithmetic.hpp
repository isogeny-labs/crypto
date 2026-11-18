#pragma once
// Exact quotient-group image representation. This is not a plane-quartic model.
#include "kernel_arithmetic.hpp"
#include "module_arithmetic.hpp"
#include <memory>
#include <unordered_map>
#include <openssl/sha.h>
struct Orbit {G representative;std::string key;};
using Q=std::shared_ptr<const Orbit>;
struct Quotient {
    std::vector<G> kernel;
    std::unordered_map<std::string,Q> cache;
    Quotient(const std::vector<G>&basis,const Basis&coordinates,const M&a,const M&form){
        require(coordinates.size()==3&&rank(coordinates)==3,"quotient kernel rank");
        require(stable(coordinates,a)&&isotropic(coordinates,form),"quotient kernel stability/isotropy");
        std::vector<G> generators;for(auto row:coordinates)generators.push_back(combination(basis,std::vector<int>(row.begin(),row.end())));
        kernel={G{}};for(auto&g:generators){require(g.times(3)==G{},"quotient kernel killed by3");auto old=kernel;for(auto&k:old){kernel.push_back(k.add(g));kernel.push_back(k.add(g.neg()));}}
        std::set<std::string> keys;for(auto&k:kernel)keys.insert(gkey(k));require(keys.size()==27,"actual quotient kernel cardinality");
        for(auto&k:kernel){require(keys.count(gkey(gfrob(k))),"actual kernel Frobenius stability");for(auto&g:generators)require(keys.count(gkey(k.add(g))),"actual kernel generator closure");}
    }
    Q image(const G&d){d.check();auto key=gkey(d);auto found=cache.find(key);if(found!=cache.end())return found->second;
        std::vector<G> orbit;std::vector<std::string> keys;size_t least=0;
        for(auto&k:kernel){orbit.push_back(d.add(k));keys.push_back(gkey(orbit.back()));if(keys.back()<keys[least])least=keys.size()-1;}
        auto q=std::make_shared<Orbit>(Orbit{orbit[least],keys[least]});
        for(auto&s:keys){auto [it,inserted]=cache.emplace(s,q);require(inserted||it->second->key==q->key,"inconsistent coset cache");}return q;
    }
    bool equal(const Q&a,const Q&b)const{return a->key==b->key;}
    Q add(const Q&a,const Q&b){return image(a->representative.add(b->representative));}
    Q negate(const Q&a){return image(a->representative.neg());}
    Q multiply(const Q&a,const mpz_class&n){return image(a->representative.times(n));}
    G dual(const Q&a)const{return a->representative.times(3);}
    Q Fmap(const EC&p){return image(pull(p));}
    EC Tmap(const Q&a)const{return norm(dual(a));}
};
void initialize_cover(const Json&j){prime=mpz_class(j.at("source_model").at("p").scalar);modulus=base_poly(j.at("finite_field_modulus_coefficients"));F b(Poly<Z>{Z(mpz_class(j.at("source_model").at("b").scalar))});dd=F(7);cc=F(1024);P t{F(0),F(1)},one{F(1)},tt=t*t,tm=tt-one,td=tt+P{dd};P h=tm*(td*td*td-(td*tm*tm).scale(F(3))+(tm*tm*tm).scale(b));require(h.degree()==8&&h.eval(F(1))==F(0),"cover input");
    std::vector<F> q(8);for(int i=0;i<=8;++i){long choose=1;F cp(1);for(int k=1;k<=i;++k){choose=choose*(i-k+1)/k;cp=cp*cc;q[8-k]=q[8-k]+h.at(i)*F(choose)/cp;}}curve=P(q);require(curve.degree()==7&&curve.c.back()==F(1),"cover normalization");}
mpz_class public_scalar(const mpz_class&r){std::string label="nist-cover-three-tests/public-map-correctness/full-width-v1";unsigned char digest[32];SHA256(reinterpret_cast<const unsigned char*>(label.data()),label.size(),digest);mpz_class n;mpz_import(n.get_mpz_t(),32,1,1,1,0,digest);return mpz_class(n%r);}
template<class Fn> void rejects(Fn f,const std::string&label){bool rejected=false;try{f();}catch(const std::exception&){rejected=true;}require(rejected,"negative control accepted: "+label);}
