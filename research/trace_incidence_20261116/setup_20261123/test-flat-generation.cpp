#include "progression-flat.hpp"
#include <nlohmann/json.hpp>
#include <filesystem>
#include <fstream>
#include <iostream>
using J=nlohmann::json;using namespace labelled_progression;
int main(int argc,char**argv){try{
 require(argc==2,"test-flat-generation PUBLIC_DIR");std::size_t targets=0,cases=0,points=0,rejected=0;
 for(const auto&e:std::filesystem::directory_iterator(argv[1])){std::ifstream in(e.path());J j;in>>j;const auto&c=j.at("curve");LabelledPools base;base.target={c.at("p"),c.at("a"),c.at("b")};base.subgroup_order=c.at("r");base.generator={c.at("P")[0],c.at("P")[1],false};base.public_target={c.at("Q")[0],c.at("Q")[1],false};const std::size_t B=j.at("B");acc::FastContext ctx(base,false);++targets;
  for(std::size_t scale:{2,4,8})for(unsigned attempt=0;attempt<4;++attempt){const Word seed=j.at("public_seed").get<Word>()+Word(attempt)*2654435761ULL;acc::GenerationCounts ga,gb;Extra xa,xb;const auto a=generate(base,B,scale,seed,ctx,ga,xa),b=generate_flat(base,B,scale,seed,ctx,gb,xb);
   require(ga.draws==gb.draws&&ga.rejected_coordinate==gb.rejected_coordinate&&ga.rejected_infinity==gb.rejected_infinity,"generation counts");require(xa.seed_pairs==xb.seed_pairs&&xa.step_infinity==xb.step_infinity&&xa.projective_additions==xb.projective_additions&&xa.affine_batches==xb.affine_batches,"AP counts");rejected+=ga.rejected_coordinate+ga.rejected_infinity;
   for(unsigned pool=0;pool<3;++pool){require(a.pools[pool].size()==b.pools[pool].size(),"pool length");for(std::size_t i=0;i<a.pools[pool].size();++i){const auto&u=a.pools[pool][i];const auto&v=b.pools[pool][i];require(u.point==v.point&&u.alpha==v.alpha&&u.beta==v.beta,"changed point/label/order");require(point_add(base.target,point_mul(base.target,v.alpha,base.generator),point_mul(base.target,v.beta,base.public_target))==v.point,"independent mixed label");++points;}}++cases;
  }
 }
 require(targets==480,"target count");std::cout<<J({{"status","passed"},{"targets",targets},{"cases",cases},{"points",points},{"rejections",rejected},{"scope","every old/flat point, label, order, count equal; independent affine mixed-label replay including rejection stream"}}).dump()<<'\n';
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
