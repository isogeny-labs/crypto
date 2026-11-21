// Pure serialization adapter for the already-frozen complete residual systems.
#include "frozen_complete_prefix.inc"
std::string cocoa_polynomial(const Poly& p,const std::map<std::string,unsigned>& indices) {
    if(p.terms.empty()) return "0";
    std::string text;
    for(const auto& [m,c]:p.terms) {
        if(!text.empty()) text+=" + ";
        text+=c.get_str();
        for(const auto& [name,e]:m) {
            std::string v=name=="H"?"h":"z["+num(indices.at(name))+"]";
            text+="*"+v+"^"+num(e);
        }
    }
    return text;
}
int main(int argc,char** argv) {
    try {
        require(argc==4 && std::string(argv[1])=="export","usage: cocoa_export export FROZEN_INPUTS FRESH_OUTPUT");
        std::filesystem::path frozen=argv[2],out=argv[3];
        require(!std::filesystem::exists(out),"fresh output required");
        std::vector<std::string> cells;
        for(unsigned d:{2U,4U}) {
            Base b=build(d); crosscheck(b); auto indices=planted_indices(b,2);
            Point target=replay(signed_points(b),indices); require(!target.infinity,"finite target");
            for(const std::string arm:{"dense","tower"}) {
                System original=relation_system(b,2,arm,target,indices); original.check();
                Reduced reduced=reduce_linear(original); check_reconstruction(original,reduced);
                std::string name="depth"+num(d)+"-"+arm;
                const auto original_json=system_json(b,2,arm,"planted",target,original)+"\n";
                const auto residual_json=system_json(b,2,arm,"planted",target,reduced.residual)+"\n";
                require(read(frozen/(name+"-original.json"))==original_json,"frozen original bytes differ");
                require(read(frozen/(name+"-residual.json"))==residual_json,"frozen residual bytes differ");
                std::map<std::string,unsigned> vars; Object names;
                for(const auto& [v,value]:reduced.residual.witness) {
                    (void)value; vars[v]=static_cast<unsigned>(vars.size()+1); names[v]=num(vars[v]);
                }
                std::string code="-- Exact frozen generator homogenization; no timing claim.\n";
                code+="p := "+P.get_str()+";\nFp := NewZZmod(p);\nassert(Characteristic(Fp)=p);\nassert(IsField(Fp));\nreseed(1);\n";
                code+="use R ::= Fp[z[1.."+num(vars.size())+"],h];\nI := ideal([\n";
                bool first=true;
                for(const auto& [eq,p]:reduced.residual.equations) {
                    (void)eq; if(p.terms.empty()) continue;
                    if(!first) code+=",\n";
                    first=false; code+=cocoa_polynomial(homogeneous_generator(p),vars);
                }
                require(!first,"zero ideal unexpected");
                code+="\n]);\nprintln \"BEGIN_COMPLETE_REGULARITY "+name+"\";\n";
                code+="ri := reg(I);\nrq := reg(R/I);\nassert(ri=rq+1);\n";
                code+="println \"IDEAL_REGULARITY "+name+" \",ri;\n";
                code+="println \"QUOTIENT_REGULARITY "+name+" \",rq;\nPrintBettiDiagram(I);\n";
                code+="println \"END_COMPLETE_REGULARITY "+name+"\";\n";
                write(out/(name+".cocoa5"),code);
                cells.push_back(object({{"name",jquote(name)},{"factor_base",jquote(b.fb_id)},
                  {"original_sha256",jquote(hash(original_json))},{"residual_sha256",jquote(hash(residual_json))},
                  {"program_sha256",jquote(hash(code+"\n"))},{"coordinate_indices",object(names)},
                  {"homogenizing_coordinate",jquote("h")},{"ideal_regularity","null"}}));
            }
        }
        write(out/"cells.json",array(cells)); return 0;
    } catch(const std::exception& e) { std::cerr<<"cocoa_export: "<<e.what()<<'\n'; return 1; }
}
