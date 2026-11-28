// Native adapter for the frozen core. Keeping the core unchanged also preserves
// its existing PCC source pin. Magma is an external, separately licensed solver.
#define main auxiliary_core_cli_main
#include "aux_base.cpp"
#undef main

std::string magma_poly(const Poly& p) {
    if(p.terms.empty()) return "R!0";
    std::string out;
    for(const auto& [monomial,coefficient]:p.terms) {
        if(!out.empty()) out+=" + ";
        out+=coefficient.get_str();
        for(const auto& [variable,exponent]:monomial) out+="*"+variable+"^"+num(exponent);
    }
    return out;
}
std::string header(const Base& b,unsigned m,const std::string& kind) {
    return "BEGIN_SOLUTIONS "+b.fb_id+" m="+num(m)+" kind="+kind;
}
std::string magma_program(const Base& b,unsigned m,const std::string& kind,const System& s) {
    std::string text="// Prospective, unexecuted CAS input. No measured regularity claim.\n";
    text+="// "+b.fb_id+" "+kind+" m="+num(m)+"; Fp rational points only.\n";
    text+="SetSeed(1);\nSetNthreads(1);\nSetColumns(256);\nF := GF("+P.get_str()+");\nR<";
    std::vector<std::string> variables;
    for(const auto& [name,value]:s.witness) { (void)value; variables.push_back(name); }
    for(size_t i=0;i<variables.size();++i) { if(i) text+=","; text+=variables[i]; }
    text+="> := PolynomialRing(F,"+num(variables.size())+",\"grevlex\");\nI := ideal<R |\n";
    for(size_t i=0;i<s.equations.size();++i) {
        text+="  // "+s.equations[i].first+"\n  "+magma_poly(s.equations[i].second);
        text+=i+1==s.equations.size()?"\n":" ,\n";
    }
    text+=">;\nSetVerbose(\"Groebner\",1);\nstarted := Cputime();\nG := GroebnerBasis(I);\n";
    text+="printf \"AFFINE_GB_MAX_DEGREE %o\\n\", Maximum([TotalDegree(g):g in G]);\n";
    text+="// This output degree is NOT degree of regularity or maximal F4 matrix degree.\n";
    text+="V := Variety(I);\nprintf \"CAS_CPU_SECONDS %o\\n\", Cputime(started);\n";
    text+="printf \""+header(b,m,kind)+"\\n\";\nfor solution in V do\n  printf \"REL\\n\";\n";
    for(unsigned j=0;j<m;++j) {
        auto xi=std::find(variables.begin(),variables.end(),"p"+num(j)+"_x");
        auto yi=std::find(variables.begin(),variables.end(),"p"+num(j)+"_y");
        require(xi!=variables.end() && yi!=variables.end(),"CAS point projection missing");
        text+="  printf \"P %o %o\\n\", Integers()!solution["+num(static_cast<size_t>(xi-variables.begin())+1)+"], Integers()!solution["+num(static_cast<size_t>(yi-variables.begin())+1)+"];\n";
    }
    text+="  printf \"END_REL\\n\";\nend for;\nprintf \"COUNT %o\\n\", #V;\nprintf \"END_SOLUTIONS\\n\";\nquit;\n";
    return text;
}
Z canonical_integer(const std::string& text) {
    require(!text.empty() && text.find_first_not_of("0123456789")==std::string::npos,"nondecimal solver coordinate/count");
    Z value(text); require(value.get_str()==text,"noncanonical solver integer"); return value;
}
std::string replay_cas(const Base& base,unsigned m,const std::string& kind,const std::string& log) {
    require(m>=2 && m<=7 && (kind=="planted" || kind=="unplanted"),"replay instance arguments");
    auto domain=signed_points(base);
    Point target=kind=="planted"?replay(domain,planted_indices(base,m)):natural_target(base,m);
    std::map<std::string,size_t> lookup;
    for(size_t i=0;i<domain.size();++i) lookup[point_key(domain[i])]=i;
    std::istringstream input(log); std::string line; bool started=false,inside=false,ended=false,counted=false;
    size_t reported=0; Z declared=-1; Tuple current; std::set<Tuple> tuples; std::set<std::string> rows;
    OpenSSLCheck openssl(TARGET);
    while(std::getline(input,line)) {
        if(!line.empty() && line.back()=='\r') line.pop_back();
        if(!started) { if(line==header(base,m,kind)) started=true; continue; }
        if(ended) { require(line.empty(),"trailing content after solver end marker"); continue; }
        if(line.empty()) continue;
        if(line=="REL") { require(!inside && !counted,"nested/late relation"); inside=true; current.clear(); }
        else if(line.starts_with("P ")) {
            require(inside && current.size()<m,"unexpected solver point");
            std::istringstream tokens(line); std::string tag,xs,ys,extra; tokens>>tag>>xs>>ys;
            require(!(tokens>>extra),"extra solver point fields");
            Z x=canonical_integer(xs), y=canonical_integer(ys); require(x<P && y<P,"coordinate outside Fp");
            Point q=point(x,y); require(TARGET.on(q),"solver point off target curve");
            auto found=lookup.find(point_key(q)); require(found!=lookup.end(),"solver point outside factor base");
            current.push_back(found->second);
        }
        else if(line=="END_REL") {
            require(inside && current.size()==m,"short solver relation");
            Point acc;
            for(size_t index:current) { Point next=TARGET.add(acc,domain[index]); openssl.add(acc,domain[index],next); acc=next; }
            require(acc==target,"solver relation failed exact target replay");
            tuples.insert(current); std::map<size_t,Z> row;
            for(size_t index:current) row.try_emplace(index/2,0);
            for(size_t index:current) row[index/2]+=index%2?-1:1;
            std::string key;
            for(const auto& [column,coefficient]:row) if(coefficient!=0) key+=num(column)+":"+coefficient.get_str()+";";
            if(!key.empty()) rows.insert(key); // zero projected rows are unusable
            ++reported; inside=false;
        }
        else if(line.starts_with("COUNT ")) {
            require(!inside && !counted,"nested/duplicate solver count"); declared=canonical_integer(line.substr(6)); counted=true;
        }
        else if(line=="END_SOLUTIONS") {
            require(!inside && counted && declared==Z(reported),"solver count mismatch/missing completion"); ended=true;
        }
        else throw std::runtime_error("unexpected content in solver solution block");
    }
    require(started && ended && !inside,"truncated/unbound solver output");
    return object({{"cas_completion_marker_seen","true"},{"factor_base",jquote(base.fb_id)},
      {"instance_kind",jquote(kind)},{"log_sha256",jquote(hash(log))},{"m",num(m)},
      {"projected_nonzero_rows",num(rows.size())},{"replayed_cas_solutions",num(reported)},
      {"schema",jquote("p256.cas_relation_replay/v1")},{"target",point_json(target)},
      {"unique_ordered_tuples",num(tuples.size())},{"verification_standing",jquote("producer-unverified; solver completeness not independently established")}});
}
std::string synthetic_log(const Base& b,unsigned m) {
    auto domain=signed_points(b); auto indices=planted_indices(b,m);
    std::string text=header(b,m,"planted")+"\nREL\n";
    for(size_t i:indices) text+="P "+domain[i].x.get_str()+" "+domain[i].y.get_str()+"\n";
    return text+"END_REL\nCOUNT 1\nEND_SOLUTIONS\n";
}
int main(int argc,char** argv) {
    try {
        require(argc>=3,"usage: magma_adapter export DEPTH OUT | replay DEPTH M KIND LOG | selftest OUT");
        std::string mode=argv[1];
        if(mode=="selftest") {
            require(argc==3,"adapter selftest arity"); Base b=build(2); std::vector<std::string> results;
            for(unsigned m=2;m<=7;++m) {
                std::string log=synthetic_log(b,m); results.push_back(replay_cas(b,m,"planted",log));
                rejects([&]{replay_cas(b,m,"unplanted",log);},"transplanted solver header");
                rejects([&]{replay_cas(b,m,"planted",log.substr(0,log.find("COUNT")));},"truncated CAS output");
                std::string bad=log; bad.replace(bad.find("COUNT 1"),7,"COUNT 2");
                rejects([&]{replay_cas(b,m,"planted",bad);},"solver count mismatch");
                bad=log; auto at=bad.find("P "); auto end=bad.find('\n',at); bad.replace(at,end-at,"P 0 0");
                rejects([&]{replay_cas(b,m,"planted",bad);},"off-curve solver point");
            }
            write(std::filesystem::path(argv[2])/"adapter-selftest.json",object({{"schema",jquote("p256.magma_adapter_selftest/v1")},
              {"synthetic_replays",array(results)},{"magma_executed","false"},{"verification_standing",jquote("producer-unverified")}}));
            std::cout<<"Synthetic CAS parser/replay tests pass; Magma was not executed.\n";
        }
        else if(mode=="export") {
            require(argc==4,"CAS export arity"); Z requested=canonical_integer(argv[2]);
            require(requested>=1 && requested<=8,"dense CAS exports limited to depth1..8");
            unsigned depth=static_cast<unsigned>(requested.get_ui()); Base b=build(depth);
            auto domain=signed_points(b); std::vector<std::string> manifest;
            for(unsigned m=2;m<=7;++m) for(const std::string kind:{"planted","unplanted"}) {
                auto indices=planted_indices(b,m);
                Point target=kind=="planted"?replay(domain,indices):natural_target(b,m);
                for(const std::string arm:{"dense","tower","paired","oriented"}) {
                    System s=relation_system(b,m,arm,target,indices); if(kind=="planted") s.check();
                    std::string program=magma_program(b,m,kind,s),name="m"+num(m)+"-"+arm+"-"+kind+".m";
                    write(std::filesystem::path(argv[3])/name,program);
                    manifest.push_back(object({{"path",jquote(name)},{"sha256",jquote(hash(program+"\n"))}}));
                }
            }
            write(std::filesystem::path(argv[3])/"manifest.json",object({{"artifacts",array(manifest)},
              {"factor_base",jquote(b.fb_id)},{"schema",jquote("p256.magma_export_manifest/v1")}}));
            std::cout<<b.fb_id<<" CAS scripts exported; no solver executed.\n";
        }
        else if(mode=="replay") {
            require(argc==7,"CAS replay arity"); Z depth=canonical_integer(argv[2]), arity=canonical_integer(argv[3]);
            require(depth>=1 && depth<=18 && arity>=2 && arity<=7,"CAS replay parameter bounds");
            require(std::string(argv[6])=="--producer-unverified","explicit standing label required");
            Base b=build(static_cast<unsigned>(depth.get_ui()));
            unsigned m=static_cast<unsigned>(arity.get_ui());
            std::cout<<replay_cas(b,m,argv[4],read(argv[5]))<<'\n';
        }
        else throw std::runtime_error("unknown adapter command");
        return 0;
    } catch(const std::exception& e) { std::cerr<<"magma_adapter: "<<e.what()<<'\n'; return 1; }
}
