#include "bounded.hpp"
#include "independent.hpp"
#include <iostream>
#include <filesystem>
#include <fstream>
using namespace vic;
static independent::Point convert(Point p){return {p.x,p.y,p.infinity};}
template<class F>static void refuses(F f){bool threw=false;try{f();}catch(const std::exception&){threw=true;}require(threw,"negative control did not fail");}
static json pw(Point p){return p.infinity?json(nullptr):json::array({std::to_string(p.x),std::to_string(p.y)});}
int main(int argc,char** argv){try{
    require(argc==1||argc==3,"tests [NEW_CONTROL_DIRECTORY PUBLIC_SOURCE_CASE]");
    std::ofstream journal;json control,config;
    // Deliberately small correctness fixture, not a solve on the 54-bit corpus.
    // p=10007 == 2 mod 3; y^2=x^3+7 has p+1=72*139 points.
    Curve e(10007,0,7);independent::Curve check(10007,0,7);Point P;
    for(U x=0;x<10007&&P.infinity;++x){U rhs=e.f.add(e.f.mul(e.f.sq(x),x),7),y=e.f.pow(rhs,2502);if(e.f.sq(y)==rhs)P=e.scalar(72,{x,y,false});}
    require(!P.infinity&&e.scalar(139,P).infinity,"fixture subgroup");
    std::array<Point,12> base{};for(int i=0;i<12;++i)base[i]=e.scalar(U(i+1),P);
    if(argc==3){require(std::filesystem::create_directory(argv[1]),"control directory must be new");json b=json::array();for(Point x:base)b.push_back(pw(x));
        control={{"schema","volcano.ic.public-instance/v1"},{"instance_id","development-positive-control-only"},{"p","10007"},{"a","0"},{"b","7"},{"r","139"},{"P",pw(P)},{"Q",pw(e.scalar(103,P))},{"factor_base",b}};
        config={{"schema","volcano.ic.bounded-config/v1"},{"radius","1"},{"memory_limit_bytes","1000000"},{"max_setup_trials","200"},{"deadline_seconds","10"}};
        std::ofstream(std::string(argv[1])+"/input.json")<<control.dump(2)<<'\n';std::ofstream(std::string(argv[1])+"/config.json")<<config.dump(2)<<'\n';journal.open(std::string(argv[1])+"/records.jsonl");require(bool(journal),"control journal open");journal<<json({{"kind","identity"},{"input",control},{"config",config}}).dump()<<'\n';}
    U group_checks=0;for(U i=0;i<139;++i){Point a=e.scalar(i,P);require(convert(a)==check.times(i,convert(P)),"scalar differential");for(U j=0;j<17;++j){Point b=e.scalar(j,P);require(convert(e.add(a,b))==check.plus(convert(a),convert(b)),"group differential");++group_checks;}}
    require(encoding({})!=encoding({0,0,false}),"infinity encoding collision");
    auto l=layout(1);require(l.side_entries==729&&l.slots==2048&&l.total_bytes==35684,"toy memory layout");
    auto big=layout(12);require(big.side_entries==244140625&&big.total_bytes==9566497092ULL,"full-size static memory calculation");
    refuses([]{layout(0);});refuses([]{layout(31);});refuses([&]{Decomposer no(e,base,1,1);});
    std::array<Point,6> half{};for(int i=0;i<6;++i)half[i]=base[i];
    U walk_checks=0;for(U origin:{0,1,728}){Walker w(e,half,1,{},origin);std::vector<Jacobian> states;for(U i=0;i<729;++i)states.push_back(w.take());auto points=e.normalize(states);for(U i=0;i<729;++i){U code=(origin+i)%729;Point expected;for(int j=0;j<6;++j){expected=e.add(expected,e.signed_scalar(int(code%3)-1,half[j]));code/=3;}require(points[i]==expected,"walker wrap/normalization");++walk_checks;}}
    Decomposer d(e,base,1,1000000);d.build();require(d.entries_built==729&&d.distinct_left_points==43,"duplicate representation preservation");
    RowSpace matrix(139);U state=0x711ac374ULL;
    auto choose=[&](U n){state=hash(state+0x9e3779b97f4a7c15ULL);return state%n;};
    U relation_checks=0;for(U trial=1;trial<=200&&matrix.rank()<12;++trial){U t=choose(138)+1;auto m=d.find(e.scalar(t,P),choose);require(m.found,"covered fixture did not decompose");
        require(evaluate(e,base,m.coefficient)==e.scalar(t,P),"native relation");independent::Point sum;for(int j=0;j<12;++j)sum=check.plus(sum,check.signed_times(m.coefficient[j],convert(base[j])));
        require(sum==check.times(t,convert(P)),"independent relation");bool increased=matrix.insert(m.coefficient,t);++relation_checks;
        if(journal.is_open())journal<<json({{"kind","setup"},{"trial",trial},{"t",std::to_string(t)},{"R",pw(e.scalar(t,P))},{"found",true},{"coefficients",m.coefficient},{"rank",matrix.rank()},{"rank_increased",increased}}).dump()<<'\n';}
    require(matrix.rank()==12,"fixture did not reach full rank");auto logs=matrix.logs();for(int i=0;i<12;++i)require(logs[i]==U(i+1),"independent expected factor-base logs");
    U targets=0;for(U k=1;k<139;++k){Point Q=e.scalar(k,P);auto m=d.find(Q,choose);require(m.found&&matrix.recover(m.coefficient)==k,"target recovery");require(check.times(matrix.recover(m.coefficient),convert(P))==convert(Q),"independent target equation");++targets;}
    if(journal.is_open()){auto m=d.find(e.scalar(103,P),choose);require(m.found,"journal control target");journal<<json({{"kind","factor_base_logs_private"},{"logs",logs}}).dump()<<'\n'<<json({{"kind","target_recovery_private"},{"coefficients",m.coefficient},{"k","103"}}).dump()<<'\n';journal.close();}
    if(argc==3){std::ifstream in(std::string(argv[1])+"/records.jsonl");std::vector<json> records;std::string line;while(std::getline(in,line))records.push_back(json::parse(line));
        auto emit=[&](const char* name,const std::vector<json>& v){std::ofstream out(std::string(argv[1])+"/"+name);for(const auto& row:v)out<<row.dump()<<'\n';require(bool(out),"negative fixture write");};
        auto bad=records;bad.back()["k"]="104";emit("bad_target.jsonl",bad);
        bad=records;bad[1]["rank"]=0;emit("bad_rank.jsonl",bad);
        bad=records;bad[1]["coefficients"][0]=13;emit("bad_bounds.jsonl",bad);
        bad=records;bad[0]["input"]["instance_id"]="another-instance";emit("bad_identity.jsonl",bad);
        bad=records;bad.pop_back();emit("missing_target.jsonl",bad);
        auto forbidden=control;forbidden["k"]="103";std::ofstream(std::string(argv[1])+"/forbidden_input.json")<<forbidden.dump(2)<<'\n';
        auto low=config;low["memory_limit_bytes"]="1";std::ofstream(std::string(argv[1])+"/low_memory_config.json")<<low.dump(2)<<'\n';
    }
    RowSpace bad(139);Coeff unit{};unit[4]=1;require(bad.insert(unit,3),"row-space insertion");require(!bad.insert(unit,3),"duplicate row");refuses([&]{bad.insert(unit,4);});refuses([&]{bad.recover(unit);});
    RowSpace coefficients(139);for(int i=0;i<12;++i){Coeff row{};row[i]=-140;coefficients.insert(row,U(139-(i+1)));}require(coefficients.logs()==logs,"signed coefficient reduction");
    Field f(139);f.mul(2,3);f.sq(2);require(f.count.mul==1&&f.count.square==1,"disjoint counter convention");
    U source_checks=0;
    if(argc==3){std::ifstream in(argv[2]);json pub;in>>pub;auto n=[](const json& j){return std::stoull(j.get<std::string>());};
        Curve src(n(pub.at("p")),n(pub.at("a")),n(pub.at("b")));independent::Curve other(n(pub.at("p")),n(pub.at("a")),n(pub.at("b")));auto parse=[&](const json& j){return Point{n(j[0]),n(j[1]),false};};
        std::vector<Point> points{parse(pub.at("P")),parse(pub.at("Q"))};for(const auto& b:pub.at("factor_base"))points.push_back(parse(b));
        for(Point x:points){require(src.on(x)&&other.on(convert(x))&&src.scalar(n(pub.at("r")),x).infinity&&other.times(n(pub.at("r")),convert(x)).infinity,"source subgroup validation");for(U k:{0,1,2,3,31,257,65537}){require(convert(src.scalar(k,x))==other.times(k,convert(x)),"source differential scalar");++source_checks;}}
        std::array<Point,6> hb{};for(int i=0;i<6;++i)hb[i]=points[i+2];Walker sw(src,hb,1,{},721);std::vector<Jacobian> batch;for(int i=0;i<31;++i)batch.push_back(sw.take());auto affine=src.normalize(batch);
        for(U i=0;i<31;++i){U code=(721+i)%729;independent::Point want;for(int j=0;j<6;++j){want=other.plus(want,other.signed_times(int(code%3)-1,convert(hb[j])));code/=3;}require(convert(affine[i])==want,"source-field batch/wrap differential");++source_checks;}
    }
    std::cout<<json({{"passed",true},{"scope","development known-answer fixture only; no full-width recovery or timing claim"},
        {"differential_group_pairs",group_checks},{"walker_states",walk_checks},{"independently_checked_setup_relations",relation_checks},
        {"rank",matrix.rank()},{"independently_checked_toy_targets",targets},{"duplicate_representations_retained",true},{"source_field_differential_scalar_checks",source_checks}}).dump()<<'\n';return 0;
}catch(const std::exception& e){std::cerr<<"TEST_ERROR "<<e.what()<<'\n';return 1;}}
