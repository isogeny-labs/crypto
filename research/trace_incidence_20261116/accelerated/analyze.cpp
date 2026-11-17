// Reuse the frozen native charge/censoring analyzer without changing its bytes.
// Additional checks cover complete support and exact generation accounting.
#define main frozen_analysis_main
#include "../runpod/analyze.cpp"
#undef main
#include <sstream>
int main(int argc,char** argv) {
    try {
        if(argc!=2)return 2;
        std::ifstream input(argv[1]);if(!input)return 2;
        using Key=std::tuple<std::string,unsigned,unsigned>;
        std::map<Key,J> supports,generation;
        unsigned support_checks=0,generation_checks=0;
        while(true) {
            input>>std::ws;if(input.peek()==std::char_traits<char>::eof())break;
            J row;try{input>>row;}catch(const J::parse_error&){if(input.eof())break;throw;}
            if(row.value("experiment","")!="incidence"||row.value("status","")=="failed")continue;
            const Key key={row.at("source_fixture"),row.at("round"),row.at("attempt")};
            if(row.value("full_support",false)) {
                const auto& coeff=row.at("gcd_coefficients");
                if(supports.count(key)){if(supports.at(key)!=coeff)throw std::runtime_error("complete GCD support disagreement");++support_checks;}
                else supports[key]=coeff;
            }
            const auto& g=row.at("generation");
            if(generation.count(key)){if(generation.at(key)!=g)throw std::runtime_error("generation/rejection accounting disagreement");++generation_checks;}
            else generation[key]=g;
        }
        std::ostringstream captured;auto* previous=std::cout.rdbuf(captured.rdbuf());int status;
        try{status=frozen_analysis_main(argc,argv);}catch(...){std::cout.rdbuf(previous);throw;}
        std::cout.rdbuf(previous);if(status)return status;
        auto output=J::parse(captured.str());output["extension_schema"]="trace-accelerated-summary/v1";
        output["cross_mechanism_full_support_checks"]=support_checks;
        output["cross_generator_rejection_accounting_checks"]=generation_checks;
        output["support_scope"]="First-witness arms do not report a GCD. Full-support and implicit-norm arms compare every coefficient. Paired timing comparisons charge all preceding misses; four misses remain censored.";
        std::cout<<output.dump(2)<<'\n';return 0;
    }catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}
}
