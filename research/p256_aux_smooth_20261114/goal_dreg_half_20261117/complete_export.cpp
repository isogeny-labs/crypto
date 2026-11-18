// Build via build_complete.sh: include unchanged old reducer without its CLI.
#include "frozen_linear_prefix.inc"

Poly homogeneous_generator(const Poly& p) {
    Poly out; const unsigned degree=p.degree();
    for(const auto& [m,c]:p.terms) {
        Monomial n=m; unsigned d=0; for(const auto& [v,e]:m) { (void)v; d+=e; }
        require(!m.contains("H"),"reserved homogenization coordinate");
        if(degree>d) n["H"]=degree-d;
        out.terms[n]=c;
    }
    require((substitute(out,"H",Poly(1))-p).terms.empty(),"dehomogenization coefficient identity");
    return out;
}
std::string complete_arm(const Base& b,const std::string& arm,const Reduced& reduced) {
    std::string code="printf \"BEGIN_CELL "+arm+"\\n\";\n";
    std::string affine=reduced_program(b,2,arm,reduced);
    auto end=affine.rfind("quit;\n"); require(end!=std::string::npos,"affine exporter terminator");
    affine.erase(end); code+=affine;
    code+="T<";
    for(const auto& [name,value]:reduced.residual.witness) { (void)value; code+=name+","; }
    code+="H> := PolynomialRing(F,"+num(reduced.residual.witness.size()+1)+",\"grevlex\");\nJ := ideal<T | ";
    bool first=true;
    for(const auto& [name,p]:reduced.residual.equations) {
        (void)name; if(p.terms.empty()) continue;
        if(!first) code+=",";
        first=false; code+="T!("+magma_poly2(homogeneous_generator(p))+")";
    }
    require(!first,"empty homogeneous ideal");
    code+=" >;\nfor g in Generators(J) do assert IsHomogeneous(g); end for;\n";
    code+="M := QuotientModule(J);\nrq := Regularity(M);\n";
    code+="printf \"QUOTIENT_REGULARITY "+arm+" %o\\n\", rq;\n";
    code+="printf \"IDEAL_REGULARITY "+arm+" %o\\n\", rq+1;\n";
    code+="printf \"BETTI_TABLE "+arm+" %o\\n\", BettiTable(M);\n";
    code+="printf \"END_CELL "+arm+"\\n\";\n"; return code;
}
int main(int argc,char** argv) {
    try {
        require(argc==3 && std::string(argv[1])=="export","usage: complete_export export FRESH_DIR");
        std::filesystem::path out=argv[2]; require(!std::filesystem::exists(out),"fresh output required");
        for(unsigned d:{2U,4U}) {
            Base b=build(d); crosscheck(b); auto indices=planted_indices(b,2);
            auto domain=signed_points(b); Point target=replay(domain,indices);
            require(!target.infinity,"qualification target must be finite");
            std::string code="SetQuitOnError(true);\nAlarm(20);\nSetSeed(1);\nSetColumns(256);\nSetVerbose(\"Groebner\",0);\n";
            if(d==2) {
                code+="F := GF("+P.get_str()+");\nU<a,b,h> := PolynomialRing(F,3,\"grevlex\");\n";
                code+="K := ideal<U | a^2-h^2,b^3-h^3 >;\nr0 := Regularity(QuotientModule(K));\nassert r0 eq 3;\n";
                code+="printf \"CI_QUOTIENT_REGULARITY %o\\n\",r0;\n";
            }
            for(const std::string arm:{"dense","tower"}) {
                System original=relation_system(b,2,arm,target,indices); original.check();
                Reduced reduced=reduce_linear(original); check_reconstruction(original,reduced);
                code+=complete_arm(b,arm,reduced);
                write(out/("depth"+num(d)+"-"+arm+"-original.json"),system_json(b,2,arm,"planted",target,original));
                write(out/("depth"+num(d)+"-"+arm+"-residual.json"),system_json(b,2,arm,"planted",target,reduced.residual));
                write(out/("depth"+num(d)+"-"+arm+"-linear-certificates.json"),array(reduced.certificates));
            }
            code+="quit;\n"; require(code.size()+1<=51200,"public CAS input cap");
            write(out/("depth"+num(d)+".m"),code);
        }
        return 0;
    } catch(const std::exception& e) { std::cerr<<"complete_export: "<<e.what()<<'\n'; return 1; }
}
