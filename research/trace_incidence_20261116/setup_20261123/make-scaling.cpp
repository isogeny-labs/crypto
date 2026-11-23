// Given-public-DLP construction only; no solver receives the private scalar file.
#include <nlohmann/json.hpp>
#include "../native/api.hpp"
#include <flint/ulong_extras.h>
#include <flint/nmod.h>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <random>
#undef ulong
#include <pari/pari.h>
#undef ulong
using J=nlohmann::json;using namespace paper_ec;
void need(bool b,const char*s){if(!b)throw std::runtime_error(s);}
int main(int argc,char**argv){try{
 need(argc==4,"make-scaling NEW_PUBLIC_DIR NEW_PRIVATE_VALIDATION NEW_CONSTRUCTION_REPORT");need(!std::filesystem::exists(argv[1])&&!std::filesystem::exists(argv[2])&&!std::filesystem::exists(argv[3]),"output exists");std::filesystem::create_directories(argv[1]);pari_init(128000000,1000000);std::mt19937_64 random(0x5343414c45333101ULL);J validation=J::array(),curves=J::array();
 for(const Word B:std::vector<Word>{128,256,512,640}){
  const Word p=B==640?2147483647ULL:n_nextprime(8*B*B*B-1,1);nmod_t mod;nmod_init(&mod,p);
  for(unsigned ci=0;ci<3;++ci){Word a=0,b=0,r=0,attempts=0;Point g;
   while(true){++attempts;if(B==640&&ci==0&&attempts==1){a=847514846;b=2087444724;}else{a=random()%p;b=random()%p;}
    if(nmod_add(nmod_mul(4,nmod_mul(a,nmod_mul(a,a,mod),mod),mod),nmod_mul(27,nmod_mul(b,b,mod),mod),mod)==0)continue;
    const pari_sp checkpoint=avma;GEN order=Fp_ellcard(utoipos(a),utoipos(b),utoipos(p));r=itou(order);avma=checkpoint;
    if(r==p||r<=3||!n_is_prime(r))continue;
    for(Word x=1;x<p;++x){const Word rhs=nmod_add(nmod_add(nmod_mul(nmod_mul(x,x,mod),x,mod),nmod_mul(a,x,mod),mod),b,mod);const Word y=n_sqrtmod(rhs,p);if(y&&nmod_mul(y,y,mod)==rhs){g={x,y,false};break;}}
    need(!g.infinity&&on_curve({p,a,b},g)&&point_mul({p,a,b},r,g).infinity,"prime-order generator check");break;
   }
   curves.push_back({{"B",B},{"curve_index",ci},{"p",p},{"a",a},{"b",b},{"r",r},{"P",{g.x,g.y}},{"curve_candidates",attempts},{"r_over_B_cubed",double(r)/double(B*B*B)},{"ordinary",r!=p+1},{"source","PARI Fp_ellcard plus FLINT prime test and independent affine order check"}});
   for(unsigned target=0;target<16;++target){const Word k=std::uniform_int_distribution<Word>(1,r-1)(random);const Point q=point_mul({p,a,b},k,g);need(!q.infinity&&on_curve({p,a,b},q),"target point");const std::string id="scaling-b"+std::to_string(B)+"-c"+std::to_string(ci)+"-t"+std::to_string(target);J f={{"schema","prime-pools/v1"},{"kind","natural"},{"id",id},{"B",B},{"curve",{{"p",p},{"a",a},{"b",b},{"r",r},{"P",{g.x,g.y}},{"Q",{q.x,q.y}}}},{"public_seed",random()%1000000000ULL}};std::ofstream out(std::filesystem::path(argv[1])/(id+".json"));out<<f.dump()<<'\n';validation.push_back({{"id",id},{"scalar",k}});}
  }
 }
 std::ofstream priv(argv[2]);priv<<validation.dump(2)<<'\n';J report={{"schema","trace-scaling-construction/v1"},{"targets",validation.size()},{"curves",curves},{"input_construction","Given public DLPs common to every arm; excluded equally from solver timing. No algorithmic auxiliary curve or free mixed labels."},{"private_validation_separate",true}};std::ofstream out(argv[3]);out<<report.dump(2)<<'\n';std::cout<<J({{"status","constructed"},{"targets",validation.size()},{"curves",curves.size()}}).dump()<<'\n';pari_close();
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
