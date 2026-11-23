// Complete target lifetime. Parent driver helpers and original controls remain unchanged.
#define main legacy_attempt_driver_main
#include "benchmark.cpp"
#undef main
#include "progression.hpp"
#include "progression-sized.hpp"
#ifdef SCALING_BASELINE
#include "canonical-flat.hpp"
using SizeMath=labelled_progression::Arithmetic;
constexpr const char* size_math_name="nmod";
#else
#include "canonical-field.hpp"
using SizeMath=labelled_progression::Arithmetic32;
constexpr const char* size_math_name="barrett";
#endif
namespace {
void extend_third(LabelledPools& p,std::size_t wanted,Word seed,acc::FastContext& ctx,acc::GenerationCounts& counts){
 std::set<Word> used;for(const auto&pool:p.pools)for(const auto&l:pool)used.insert(l.point.x);
 std::mt19937_64 random(seed^0x9e3779b97f4a7c15ULL);std::uniform_int_distribution<Word> draw(0,p.subgroup_order-1);
 while(p.pools[2].size()<wanted){
  const auto deficit=wanted-p.pools[2].size();std::vector<std::array<Word,2>> labels;labels.reserve(deficit);
  for(std::size_t i=0;i<deficit;++i){need(++counts.draws<1000000,"extension draw bound");labels.push_back({draw(random),draw(random)});}
  const auto points=ctx.combine(labels,true);
  for(std::size_t i=0;i<points.size();++i){const auto point=points[i];if(point.infinity){++counts.rejected_infinity;continue;}if(!used.insert(point.x).second){++counts.rejected_coordinate;continue;}p.pools[2].push_back({point,labels[i][0],labels[i][1]});}
 }
}
}
int main(int argc,char**argv){try{
 std::vector<std::string> args(argv+1,argv+argc);if(!args.empty()&&args[0]=="--check-only"){check_only=true;args.erase(args.begin());}
 need(args.size()==3,"target [--check-only] cold|reuse|asym2|asym4|asym8|grow FIXTURE BASE_SEED");
 const std::string kernel=std::getenv("TRACE_PAIR_KERNEL")?std::getenv("TRACE_PAIR_KERNEL"):"rust";need(kernel=="rust"||kernel=="nmod"||kernel=="barrett","pair kernel");
 const auto&mode=args[0];need(mode=="balanced"||mode=="quarter"||mode=="half"||mode=="one"||mode=="two","shape");
 need(args[1].find("validation")==std::string::npos,"secret input rejected");
 const auto began=reading();std::ifstream input(args[1]);
 const std::set<std::string> keys{"schema","kind","id","B","curve","public_seed"};
 Json j=Json::parse(input,[&](int depth,Json::parse_event_t event,Json& value){return !(event==Json::parse_event_t::key&&depth==1)||keys.count(value.get<std::string>())!=0;});
 need(j.at("schema")=="prime-pools/v1"&&j.at("kind")=="natural","schema");
 const auto&c=j.at("curve");LabelledPools base;base.target={c.at("p"),c.at("a"),c.at("b")};base.subgroup_order=c.at("r");base.generator={c.at("P")[0],c.at("P")[1],false};base.public_target={c.at("Q")[0],c.at("Q")[1],false};
 const auto B=j.at("B").get<std::size_t>();need(B==128||B==256||B==512||B==640,"registered size");std::size_t used=0;const Word seed=std::stoull(args[2],&used);need(used==args[2].size(),"malformed seed");
 const bool ap=true;const auto sizes=labelled_progression::shape_sizes(B,base.subgroup_order,mode);
 const auto parsed=reading();std::unique_ptr<canonical_pairs::Selector> held;
 if(mode!="cold")held=std::make_unique<canonical_pairs::Selector>(base,!ap);
 const auto setup=reading();LabelledPools p;Json trials=Json::array();IncidenceAnswer answer;double context_wall=setup.wall-parsed.wall,context_cpu=setup.cpu-parsed.cpu,generation_wall=0,generation_cpu=0,selection_wall=0,selection_cpu=0;
 for(unsigned attempt=0;attempt<4;++attempt){
  const auto before_context=reading();
  std::unique_ptr<canonical_pairs::Selector> fresh;if(mode=="cold")fresh=std::make_unique<canonical_pairs::Selector>(base,true);auto&selector=held?*held:*fresh;auto&ctx=selector.context();
  const Word attempt_seed=seed+Word(attempt)*2654435761ULL;acc::GenerationCounts g;const auto start=reading();context_wall+=start.wall-before_context.wall;context_cpu+=start.cpu-before_context.cpu;
  labelled_progression::Extra ap_counts;
  if(ap)p=labelled_progression::generate_sizes<SizeMath>(base,sizes,attempt_seed,ctx,g,ap_counts);
  else if(mode=="grow"&&attempt>0)extend_third(p,B*(attempt+1),attempt_seed,ctx,g);
  else{p=acc::generate(base,B,attempt_seed,acc::Generator::BatchedWindow,g,&ctx);std::size_t scale=mode=="asym2"?2:mode=="asym4"?4:mode=="asym8"?8:1;if(scale>1)extend_third(p,scale*B,attempt_seed,ctx,g);}
  const auto generated=reading();answer=kernel=="rust"?ctx.first_witness(p,true):selector.search(p,kernel=="barrett");
  if(answer.informative_relation)need(point_mul(p.target,answer.recovered_scalar,p.generator)==p.public_target,"public scalar replay failed");
  const auto selected=reading();generation_wall+=generated.wall-start.wall;generation_cpu+=generated.cpu-start.cpu;selection_wall+=selected.wall-generated.wall;selection_cpu+=selected.cpu-generated.cpu;
  if(check_only){
   for(const auto&pool:p.pools)for(const auto&l:pool)need(point_add(p.target,point_mul(p.target,l.alpha,p.generator),point_mul(p.target,l.beta,p.public_target))==l.point,"mixed-label construction mismatch");
   auto reference=frozen(p);need((reference.at("status")=="relation")==answer.informative_relation,"independent miss/hit mismatch");if(answer.informative_relation)need(reference.at("recovery").at("k")==answer.recovered_scalar,"independent scalar mismatch");
  }
  trials.push_back({{"attempt",attempt},{"seed",attempt_seed},{"sizes",{p.pools[0].size(),p.pools[1].size(),p.pools[2].size()}},{"draws",g.draws},{"rejected_infinity",g.rejected_infinity},{"rejected_coordinate",g.rejected_coordinate},{"ap_counts",{{"seed_pairs",ap_counts.seed_pairs},{"step_infinity",ap_counts.step_infinity},{"projective_additions",ap_counts.projective_additions},{"affine_batches",ap_counts.affine_batches}}},{"status",answer.informative_relation?"relation":"none"}});
  if(answer.informative_relation)break;
 }
 const auto finish=reading();struct rusage usage{};if(!check_only)getrusage(RUSAGE_SELF,&usage);
 Json row={{"schema","trace-target-lifetime/v1"},{"mode",mode},{"pair_kernel",kernel},{"generator_container","flat"},{"generation_field",size_math_name},{"fixture_id",j.at("id")},{"B",B},{"pool_sizes",sizes},{"nominal_product_density",double(__uint128_t(sizes[0])*sizes[1]*sizes[2])/double(base.subgroup_order)},{"seed",seed},{"status",answer.informative_relation?"relation":"none"},{"trials",trials},{"complete",elapsed(began,finish)},{"input_ingestion",elapsed(began,parsed)},{"after_input_ingestion",elapsed(parsed,finish)},{"initial_context",elapsed(parsed,setup)},{"context_total",{{"wall_seconds",context_wall},{"cpu_seconds",context_cpu}}},{"generation",{{"wall_seconds",generation_wall},{"cpu_seconds",generation_cpu}}},{"selection",{{"wall_seconds",selection_wall},{"cpu_seconds",selection_cpu}}},{"peak_rss_kib",usage.ru_maxrss},{"timings_collected",!check_only}};
 if(held){const auto counts=held->counts();row["fast_uninformative"]=counts.uninformative;}
 if(answer.informative_relation) { row["recovered_scalar"]=answer.recovered_scalar; }
 std::cout<<row.dump()<<'\n';
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
