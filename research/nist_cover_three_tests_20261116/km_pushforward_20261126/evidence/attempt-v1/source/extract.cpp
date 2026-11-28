#include "context.hpp"
int main(int argc,char** argv) { try {
    using Clock=std::chrono::steady_clock;auto last=Clock::now();std::vector<std::pair<std::string,long long>> phases;
    auto mark=[&](const std::string& name) { auto now=Clock::now();auto us=std::chrono::duration_cast<std::chrono::microseconds>(now-last).count();phases.emplace_back(name,us);last=now;std::cerr<<name<<" passed wall_us="<<us<<'\n'; };
    require(argc==7,"usage: extract cover model relations formal actual class.json");initialize_cover(read_json(argv[1]));Level2Input level;Context c(argv);mark("fixed_context_setup");
    auto supplied=read_json(argv[6]);Class input=class_input(supplied.at("class_space"),c.km);mark("external_class_coordinates");
    auto normalized=normalize(c.km,input);require(c.km.equal(input,normalized.finite),"opaque normalization preserves input KM class");mark("infinity_normalization_and_KM_equality");
    KMDecoder decoder(c.km,c.adapter,c.f,c.actual);mark("RR_decoder_setup");
    auto result=decoder.decode(normalized.finite,false);require(c.km.equal(result.encoded,input),"decoded cycle reencodes original external class");mark("RR_decode_and_original_class_equality");
    std::cout<<"{\"scope\":\"external-KM-class-to-finite-degree3-cycle\",\"valid_RR_input_required\":true,\"generating_scalar_or_support_read\":false,\"removed_infinity_multiplicity\":"<<normalized.removed<<",\"original_KM_class_reencoded\":true,\"decoded\":";kd_emit(result);
    std::cout<<",\"reference_points\":[";for(size_t i=0;i<c.adapter.reference.size();++i) { if(i)std::cout<<',';point_json(c.km.points[size_t(c.adapter.reference[i])]); }
    std::cout<<"],\"target_addflips\":"<<c.km.flips<<",\"signed_independent_verdict\":false,\"phase_us\":{";
    for(size_t i=0;i<phases.size();++i) { if(i)std::cout<<',';std::cout<<'"'<<phases[i].first<<"\":"<<phases[i].second; }std::cout<<"}}\n";
} catch(const std::exception& e) { std::cerr<<"REJECT "<<e.what()<<'\n';return 1; } }
