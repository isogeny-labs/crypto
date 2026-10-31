#include "cm8-transfer-prior.hpp"
namespace dual13 {
using namespace cm8;
Point dual(Point R) {
    if (R.infinity() || R.x == 7) return {};
    I v=mod(12*R.x-6,13), w=ipow(v,11,13);
    return {mod(10*(v+5+8*w)+10,13),mod(12*R.y*(1-8*w*w),13)};
}
Json requests(I scalar,I forward,I backward,I comparisons,I lookup=0) {
    return obj({{"scalar",num(scalar)},{"phi",num(forward)},{"dual",num(backward)},
                {"comparisons",num(comparisons)},{"cache_lookup",num(lookup)}});
}
Point verify(const Curve&E,const Curve&T,Point R,Point V) {
    require(T.contains(R)&&T.scalar(7,R).infinity(),"inverse domain r7");
    require(E.contains(V)&&E.scalar(7,V).infinity()&&phi(V)==R,"inverse boundary");
    return V;
}
}
int main(int argc,char**argv) {try {
    using namespace dual13;
    require(argc==3,"cm8-dual ROOT FRESH_DIRECTORY");
    fs::path root=argv[1],out=argv[2];require(!fs::exists(out),"fresh output");fs::create_directories(out);
    auto ref=out/"reference",cand=out/"candidate";fs::create_directories(cand);
    std::vector<std::string> aa{"cm8-reference",root.string(),ref.string()};
    std::vector<char*> av;for(auto&x:aa)av.push_back(x.data());
    require(cm8_reference_main(3,av.data())==0,"reference completion");
    Field k(13);Curve E(k,1,4),T(k,4,6);Point P{0,2},U=phi(P);
    auto ep=E.points(),tp=T.points();require(ep.size()==14&&tp.size()==14,"rational populations");
    auto em=arr(),tm=arr(),forward_pairs=arr(),dual_pairs=arr(),inv=arr();
    for(auto A:ep) {
        require(T.contains(phi(A))&&dual(phi(A))==E.scalar(2,A),"dual forward composition");
        em.items.push_back(obj({{"point",pj(A)},{"phi",pj(phi(A))},{"dual_phi",pj(dual(phi(A)))},{"double",pj(E.scalar(2,A))}}));
        for(auto B:ep){auto S=E.add(A,B);require(phi(S)==T.add(phi(A),phi(B)),"forward addition");
            forward_pairs.items.push_back(obj({{"a",pj(A)},{"b",pj(B)},{"sum",pj(S)},{"image_sum",pj(phi(S))},{"sum_images",pj(T.add(phi(A),phi(B)))}}));}
    }
    for(auto R:tp) {
        require(E.contains(dual(R))&&phi(dual(R))==T.scalar(2,R),"forward dual composition");
        bool eligible=T.scalar(7,R).infinity();Point V=eligible?E.scalar(4,dual(R)):Point{};
        if(eligible)verify(E,T,R,V);
        tm.items.push_back(obj({{"point",pj(R)},{"dual",pj(dual(R))},{"phi_dual",pj(phi(dual(R)))},{"double",pj(T.scalar(2,R))},{"inverse_accepted",truth(eligible)},{"inverse",pj(V)}}));
        for(auto S:tp){auto A=T.add(R,S);require(dual(A)==E.add(dual(R),dual(S)),"dual addition");
            dual_pairs.items.push_back(obj({{"a",pj(R)},{"b",pj(S)},{"sum",pj(A)},{"image_sum",pj(dual(A))},{"sum_images",pj(E.add(dual(R),dual(S)))}}));}
    }
    for(I n=0;n<7;++n){auto A=E.scalar(n,P),R=T.scalar(n,U);require(E.scalar(4,dual(R))==A,"subgroup inverse");
        inv.items.push_back(obj({{"n",num(n)},{"source",pj(A)},{"target",pj(R)},{"inverse",pj(E.scalar(4,dual(R)))}}));}
    std::vector<std::pair<Point,Point>> cache;auto cache_json=arr();
    for(I i=0;i<7;++i){auto A=E.scalar(i,P);cache.emplace_back(phi(A),A);cache_json.items.push_back(obj({{"i",num(i)},{"source",pj(A)},{"image",pj(phi(A))}}));}
    auto pullbacks=arr(),pins=arr();
    for(auto entry:{std::pair{"main",E},std::pair{"twist",T}}){bool main=std::string(entry.first)=="main";Point G=main?P:U;auto xs=subgroup_xs(entry.second,G,7);I policy=0;
        for(I i=0;i<3;++i)for(I j=i+1;j<3;++j){auto base=representatives(entry.second,{xs[i],xs[j]});auto name=std::string(entry.first)+std::to_string(policy);auto armdata=cm8::arm(E,entry.second,P,G,base,!main);std::vector<Point>mapped;
            for(I col=0;col<2;++col){auto B=base[col];if(main){mapped.push_back(phi(B));continue;}
                auto trials=arr();Point V{};I hit=-1;for(I z=0;z<7;++z){auto A=E.scalar(z,P);bool match=phi(A)==B;trials.items.push_back(obj({{"i",num(z)},{"source",pj(A)},{"image",pj(phi(A))},{"match",truth(match)}}));if(match){V=A;hit=z;break;}}
                require(hit>=0,"reference inverse exists");Point C{};I count=0;for(auto&[R,A]:cache){++count;if(R==B){C=A;break;}}
                auto D=dual(B),W=E.scalar(4,D);verify(E,T,B,V);verify(E,T,B,C);verify(E,T,B,W);require(V==C&&V==W,"same base pullback");mapped.push_back(W);
                pullbacks.items.push_back(obj({{"policy",num(policy)},{"column",num(col)},{"target",pj(B)},{"trials",trials},{"reference",pj(V)},{"cached",pj(C)},{"dual",pj(D)},{"candidate",pj(W)},
                    {"reference_requests",requests(hit+1,hit+1,0,hit+1)},{"cached_requests",requests(0,0,0,count,1)},{"candidate_requests",requests(1,0,1,0)},
                    {"boundary_requests_per_route",obj({{"scalar",num(2)},{"phi",num(1)},{"curve_checks",num(2)},{"comparisons",num(3)}})}}));
            }
            auto other=std::string(main?"matched":"pullback")+std::to_string(policy);
            auto otherdata=cm8::arm(E,main?T:E,P,main?U:P,mapped,main);
            for(auto item:{std::pair{name,armdata},std::pair{other,otherdata}}){auto bytes=dump(item.second)+'\n';require(bytes==read(ref/(item.first+".json")),"complete arm byte parity");std::ofstream(cand/(item.first+".json"))<<bytes;pins.items.push_back(obj({{"name",str(item.first)},{"sha256",str(digest(bytes))}}));}++policy;
        }
    }
    std::ofstream(cand/"index.json")<<read(ref/"index.json");
    const std::string s="research/quadratic_twist_evidence_20261021/";auto sources=arr();
    for(auto p:{s+"native/cm8-dual.cpp",s+"native/cm8-transfer-prior.hpp",s+"native/cm8-transfer.cpp",s+"native/cm8-dual-audit/main.rs",s+"CM8_DUAL_20261029_PROTOCOL.md",s+"lean-cm8/CM8Transfer.lean"})sources.items.push_back(obj({{"path",str(p)},{"sha256",str(digest(read(root/p)))}}));
    auto data=obj({{"schema",str("quadratic-twist-cm8-dual/v1")},{"source_points",em},{"target_points",tm},{"forward_addition",forward_pairs},{"dual_addition",dual_pairs},{"subgroup",inv},{"pullbacks",pullbacks},{"cache",cache_json},
        {"cache_setup_requests",requests(7,7,0,0)},{"multiplier",num(4)},{"source_recoveries",num(72)},{"arms",pins},{"sources",sources}});
    auto bytes=dump(data)+'\n';std::ofstream(out/"dual.json")<<bytes;
    std::ofstream(out/"dual.sha256")<<digest(bytes)<<"  dual.json\n";
    std::cout<<"14+14 points;196+196 addition pairs;12 arms/72 recoveries byte-identical;6 explicit pullbacks\n";
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
