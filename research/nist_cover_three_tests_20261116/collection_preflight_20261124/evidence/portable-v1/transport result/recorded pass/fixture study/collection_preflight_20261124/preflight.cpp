#include "typed_json.hpp"
#include "pins.hpp"
#include <openssl/sha.h>
#include <algorithm>
#include <array>
#include <functional>
#include <iomanip>
#include <iostream>
#include <set>

const std::set<std::string> semantic_fields{
    "candidate_lines","factor_base_points","images","infinity_images","local_map_points",
    "pool","projected_base","projected_base_size","public_composition_companions",
    "rank_deficiency","rank_mod_r","relations","rows","scope","split_fibers",
    "target_KM_zero_checks","zero_projected_rows"};
const std::set<std::string> metadata_fields{"standing","stages","cost_scope"};
std::string bytes(const std::string& path) {
    std::ifstream in(path,std::ios::binary|std::ios::ate);
    require(bool(in),"cannot read "+path);
    const auto size=in.tellg();
    require(size>=0&&size<=32*1024*1024,"input size cap");
    std::string value(static_cast<size_t>(size),'\0');
    in.seekg(0);
    in.read(value.data(),static_cast<std::streamsize>(value.size()));
    require(in.gcount()==static_cast<std::streamsize>(value.size()),"complete input read");
    return value;
}
std::string digest(const std::string& value) {
    std::array<unsigned char,SHA256_DIGEST_LENGTH> result{};
    require(SHA256(reinterpret_cast<const unsigned char*>(value.data()),value.size(),result.data())!=nullptr,"SHA256");
    std::ostringstream out;
    for(unsigned char byte:result) out<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(byte);
    return out.str();
}
void exact_bytes(const std::string& value,const std::string& expected,const std::string& label) {
    require(digest(value)==expected,"exact input hash mismatch: "+label);
}
Json parse(const std::string& value) {
    Parser parser{value};
    auto result=parser.value();parser.ws();
    require(parser.p==parser.s.size(),"complete JSON parse");
    return result;
}
bool same(const Json& a,const Json& b) {
    if(a.kind!=b.kind||a.scalar!=b.scalar||a.arr.size()!=b.arr.size()||a.obj.size()!=b.obj.size()) return false;
    for(size_t i=0;i<a.arr.size();++i) if(!same(a.arr[i],b.arr[i])) return false;
    auto first=a.obj.begin(),second=b.obj.begin();
    for(;first!=a.obj.end();++first,++second)
        if(first->first!=second->first||!same(first->second,second->second)) return false;
    return true;
}
void baseline_equal(const Json& reference,const Json& actual) {
    require(reference.kind==JsonKind::object&&actual.kind==JsonKind::object,"baseline object types");
    require(reference.obj.size()==20&&actual.obj.size()==reference.obj.size(),"exact baseline root schema");
    for(const auto& entry:reference.obj) {
        require(actual.obj.count(entry.first)==1,"required baseline root field");
        require(semantic_fields.count(entry.first)||metadata_fields.count(entry.first),"declared baseline field");
    }
    for(const auto& name:semantic_fields)
        require(same(reference.at(name),actual.at(name)),"frozen baseline semantic drift: "+name);
}
void must_reject(const std::function<void()>& call) {
    bool rejected=false;
    try {call();} catch(const std::runtime_error&) {rejected=true;}
    require(rejected,"drift control accepted");
}
void self_test(const std::string& raw_reference,const Json& reference,const Json& actual,const std::string& input_root) {
    baseline_equal(reference,actual);
    Json metadata=actual;
    metadata.obj["standing"]=parse("\"untrusted metadata label\"");
    metadata.obj["stages"]=parse("[]");
    metadata.obj["cost_scope"]=parse("\"different process clocks\"");
    baseline_equal(reference,metadata);
    const std::vector<std::function<void(Json&)>> mutations{
        [](Json& j){j.obj.at("pool").obj.at("points").arr[0].obj.at("y").scalar="DRIFT";},
        [](Json& j){auto& a=j.obj.at("pool").obj.at("points").arr;std::swap(a[0],a[1]);},
        [](Json& j){j.obj.at("projected_base").arr[0].obj.at("x").scalar="DRIFT";},
        [](Json& j){j.obj.at("images").arr[1].obj.at("sign").scalar="0";},
        [](Json& j){j.obj.at("images").arr[1].obj.at("point").obj.at("y").scalar="DRIFT";},
        [](Json& j){j.obj.at("pool").obj.at("split_fibers").arr[0].obj.at("point_indices").arr[0].scalar="-1";},
        [](Json& j){j.obj.at("rows").arr[0].arr[0].arr[1].scalar="DRIFT";},
        [](Json& j){j.obj.at("public_composition_companions").arr[0].obj.at("norm").obj["unexpected"]=parse("null");},
        [](Json& j){j.obj.at("rank_mod_r").scalar="0";},
        [](Json& j){j.obj.erase("rows");},
        [](Json& j){j.obj["unexpected"]=parse("null");},
        [](Json& j){auto& sign=j.obj.at("images").arr[1].obj.at("sign");require(sign.kind==JsonKind::number,"numeric sign fixture");sign.kind=JsonKind::string;}
    };
    require(mutations.size()==12,"frozen semantic control count");
    for(const auto& mutate:mutations) {
        Json changed=actual;mutate(changed);
        must_reject([&]{baseline_equal(reference,changed);});
    }
    must_reject([&]{exact_bytes(raw_reference+"\n",reference_pin,"reference mutation");});
    for(const auto& pin:construction_pins) {
        const auto raw=bytes(input_root+"/"+pin.path);
        must_reject([&]{exact_bytes(raw+"\n",pin.sha256,pin.path);});
    }
    require(!same(parse("1"),parse("\"1\"")),"number/string types stay distinct");
    require(!same(parse("[]"),parse("{}")),"array/object types stay distinct");
}
int main(int argc,char** argv) {
    try {
        require(argc==6&&(std::string(argv[1])=="--check"||std::string(argv[1])=="--self-test"),
                "usage: preflight --check|--self-test REFERENCE.json ACTUAL.json INPUT-directory OUTPUT.json");
        require(!std::ifstream(argv[5]).good(),"output already exists");
        const auto raw_reference=bytes(argv[2]);
        exact_bytes(raw_reference,reference_pin,"fixed reference");
        const auto reference=parse(raw_reference),actual=parse(bytes(argv[3]));
        for(const auto& pin:construction_pins) exact_bytes(bytes(std::string(argv[4])+"/"+pin.path),pin.sha256,pin.path);
        baseline_equal(reference,actual);
        const bool testing=std::string(argv[1])=="--self-test";
        if(testing) self_test(raw_reference,reference,actual,argv[4]);
        std::ofstream out(argv[5]);require(bool(out),"new preflight output");
        out<<"{\"schema\":\"p192.baseline-preflight/v1\",\"standing\":\"producer-unverified\",\"scope\":\"frozen-input-consistency-only\",\"mode\":\""
            <<(testing?"self-test":"check")<<"\",\"semantic_fields_checked\":"<<semantic_fields.size()
            <<",\"exact_construction_inputs_checked\":3,\"reference_sha256\":\""<<reference_pin
            <<"\",\"positive_baseline_cases\":"<<(testing?2:1)<<",\"semantic_mutations_rejected\":"<<(testing?12:0)
            <<",\"exact_pin_mutations_rejected\":"<<(testing?4:0)<<",\"typed_parser_controls\":"<<(testing?2:0)
            <<",\"admission_validated\":false,\"new_curve_points\":0,\"curve_or_Jacobian_arithmetic_executed\":false}\n";
        require(bool(out),"complete preflight output");
    } catch(const std::exception& error) {std::cerr<<"REJECT "<<error.what()<<'\n';return 1;}
}
