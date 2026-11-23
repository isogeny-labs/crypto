#include "canonical-blocked.hpp"
#include "progression-reserved.hpp"
#include <nlohmann/json.hpp>
#include <filesystem>
#include <fstream>
#include <iostream>
using namespace paper_ec;using J=nlohmann::json;
void need(bool b,const char*s){if(!b)throw std::runtime_error(s);}
int main(int argc,char**argv){try{
 need(argc==2,"test-hot NEW_PUBLIC_DIR");canonical_pairs::BlockedBloom bloom(100000);std::vector<Word> keys;for(Word i=0;i<100000;++i){const Word k=canonical_pairs::mix(i);keys.push_back(k);bloom.add(k);}for(Word k:keys)need(bloom.possible(k),"blocked Bloom false negative");
 Word points=0,cases=0,targets=0,rejections=0;for(const auto&e:std::filesystem::directory_iterator(argv[1])){std::ifstream in(e.path());J f;in>>f;const auto&c=f.at("curve");LabelledPools base;base.target={c.at("p"),c.at("a"),c.at("b")};base.subgroup_order=c.at("r");base.generator={c.at("P")[0],c.at("P")[1],false};base.public_target={c.at("Q")[0],c.at("Q")[1],false};paper_ec::accelerated::FastContext ctx(base,false);++targets;
  for(const std::string shape:{"balanced","quarter"}){const auto sizes=labelled_progression::shape_sizes(f.at("B"),base.subgroup_order,shape);auto retained=base;for(unsigned a=0;a<4;++a){const Word seed=f.at("public_seed").get<Word>()+Word(a)*2654435761ULL;paper_ec::accelerated::GenerationCounts ga,gb;labelled_progression::Extra ea,eb;const auto original=labelled_progression::generate_sizes<labelled_progression::Arithmetic32>(base,sizes,seed,ctx,ga,ea);retained=labelled_progression::generate_sizes_reserved<labelled_progression::Arithmetic32>(std::move(retained),sizes,seed,ctx,gb,eb);need(ga.draws==gb.draws&&ga.rejected_coordinate==gb.rejected_coordinate&&ga.rejected_infinity==gb.rejected_infinity,"generation counts changed");need(ea.seed_pairs==eb.seed_pairs&&ea.step_infinity==eb.step_infinity&&ea.projective_additions==eb.projective_additions&&ea.affine_batches==eb.affine_batches,"AP counts changed");rejections+=ga.rejected_coordinate+ga.rejected_infinity;
   for(unsigned h=0;h<3;++h){need(original.pools[h].size()==retained.pools[h].size(),"pool size");for(std::size_t i=0;i<original.pools[h].size();++i){const auto&x=original.pools[h][i];const auto&y=retained.pools[h][i];need(x.point==y.point&&x.alpha==y.alpha&&x.beta==y.beta,"pool reuse changed exact point/label/order");++points;}}++cases;
  }}
 }
 need(targets==192,"target count");std::cout<<J({{"status","passed"},{"targets",targets},{"generator_cases",cases},{"exact_point_label_order_comparisons",points},{"rejections",rejections},{"blocked_filter_inserted_keys",keys.size()},{"scope","exact generator stream/count equivalence across four attempts and both shapes, including retained-capacity reuse; no-false-negative blocked-filter insertion check"}}).dump()<<'\n';
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
