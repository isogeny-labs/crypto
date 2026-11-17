#pragma once
#include "closed_transpose_arithmetic.hpp"
struct MTWitness {CFPoly factor;bool infinity=false;unsigned long parameter=0;MCycle residual;G image;std::vector<CTFiber> fibers;std::vector<std::string> rejections;};
Poly<F> mt_restrict(const EBPoly& f,const F& m,const F& t){Poly<F> x{F(0),F(1)},y{t,m},out;for(auto& [e,c]:f.terms){Poly<F> term{F(CFPoly{c})};for(int i=0;i<e[0];++i)term=term*x;for(int j=0;j<e[1];++j)term=term*y;out=out+term;}return out;}
G mt_point(const MMap& map,const CFPoly& factor,const CFPoly& qx,const CFPoly& qy,bool infinity,MTWitness& witness){
    auto saved=modulus,saved_source=GBSource::modulus;witness.factor=factor;witness.infinity=infinity;unsigned long d=unsigned(map.den.degree()),bound=(2*d+8)*(2*d+8)*(2*d+8)*(2*d+8)+32;require(prime>mpz_class(bound),"moving transpose line schedule below characteristic");
    for(unsigned long a=0;a<=bound;++a){try{
        modulus=factor;F z(CFPoly{Z(0),Z(1)}),tx=ct_eval(qx,z),ty=ct_eval(qy,z),m=infinity?ty:F(CFPoly{Z(mpz_class(a))}),t=infinity?F(CFPoly{Z(mpz_class(a))}):ty-m*tx;auto restriction=mt_restrict(map.f,m,t);Poly<F> residual;
        if(infinity){require(restriction.degree()==3,"moving infinity line transverse chart");residual=restriction.monic();}else{require(restriction.degree()==4,"moving finite line affine degree4");residual=restriction.exact(Poly<F>{-tx,F(1)}).monic();require(residual.degree()==3,"moving finite residual degree3");}
        GBSource::modulus=factor;std::vector<GBSource> uc;for(auto& c:residual.c)uc.emplace_back(c.a);MAlgebra algebra(factor.degree(),GBP(uc),GBP{GBSource(0),GBSource(1)},GBP{GBSource(t.a),GBSource(m.a)});auto cycle=algebra.cycle(map.f);modulus=saved;GBSource::modulus=saved_source;std::vector<CTFiber> fibers;G value=ct_closed(map,cycle.u,cycle.x,cycle.y,fibers);for(auto& f:fibers)require(f.u.degree()==9,"moving residual meets transpose boundary");witness.parameter=a;witness.residual=cycle;witness.fibers=std::move(fibers);witness.image=value;return value;
    }catch(const std::exception& e){modulus=saved;GBSource::modulus=saved_source;witness.rejections.push_back(std::to_string(a)+":"+e.what());}}
    throw std::runtime_error("moving transpose line schedule exhausted");
}
G mt_normalized(const MMap& map,const CFPoly& u,const CFPoly& x,const CFPoly& y,const G& aref,std::vector<MTWitness>& witnesses){
    require(cm_curve(map.f,u,x,y).degree()<0,"moving closed input quartic");std::vector<CFPoly> factors;ct_factor(u,factors);CFPoly replay{Z(1)};G total;for(auto& factor:factors){ct_irreducible(factor);replay=replay*factor;MTWitness w;auto a=mt_point(map,factor,x%factor,y%factor,false,w);total=total.add(aref.times(factor.degree()).add(a.neg()));witnesses.push_back(w);}require(replay==u.monic(),"moving input factor multiplicity replay");return total;
}
void mt_emit(const MTWitness& w){std::cout<<"{\"factor\":";cf_emit_poly(w.factor);std::cout<<",\"infinity\":"<<(w.infinity?"true":"false")<<",\"line_parameter\":"<<w.parameter<<",\"rejections\":[";for(size_t i=0;i<w.rejections.size();++i)std::cout<<(i?",":"")<<std::quoted(w.rejections[i]);std::cout<<"],\"residual_cycle\":";cm_emit_cycle(w.residual);std::cout<<",\"source_image\":";tr_emit_g(w.image);std::cout<<",\"transpose_factors\":[";for(size_t i=0;i<w.fibers.size();++i){if(i)std::cout<<',';ct_emit(w.fibers[i]);}std::cout<<"]}";}
