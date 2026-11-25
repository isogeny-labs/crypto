#include "json.hpp"
#include <algorithm>
#include <array>
#include <iostream>
#include <tuple>

using Incidence=std::array<int,3>;
std::vector<Incidence> derive(const Json& original) {
    const auto& matrix=original.at("matrix").arr;
    require(matrix.size()==108,"complete original row domain");
    std::vector<Incidence> found;
    for(size_t row=0;row<matrix.size();++row) {
        require(matrix[row].arr.size()==429,"complete original column domain");
        size_t count=0;
        for(size_t col=225;col<429;++col) {
            const int c=matrix[row].arr[col].integer();
            require(c>=-1&&c<=1,"new-column coefficient domain");
            if(c) {found.push_back({int(row),int(col),c});++count;}
        }
        require(count==(row<6?0U:2U),"old rows or two actual new incidences");
    }
    std::sort(found.begin(),found.end(),[](const Incidence& a,const Incidence& b){return std::tie(a[1],a[0],a[2])<std::tie(b[1],b[0],b[2]);});
    require(found.size()==204,"complete incidence population");
    for(size_t i=0;i<found.size();++i) require(found[i][1]==int(i)+225,"every new column occurs exactly once");
    return found;
}
std::vector<Incidence> certificate(const Json& instrumented) {
    std::vector<Incidence> out;
    for(const auto& item:instrumented.at("private_columns").arr) {
        require(item.arr.size()==3,"incidence record shape");
        out.push_back({item.arr[0].integer(),item.arr[1].integer(),item.arr[2].integer()});
    }
    std::sort(out.begin(),out.end(),[](const Incidence& a,const Incidence& b){return std::tie(a[1],a[0],a[2])<std::tie(b[1],b[0],b[2]);});
    return out;
}
void check(const Json& original,const Json& instrumented) {require(derive(original)==certificate(instrumented),"certificate equals independently sorted original incidences");}
template<class Fn>void reject(Fn fn) {bool failed=false;try{fn();}catch(const std::exception&){failed=true;}require(failed,"mutation rejected");}
int run(int argc,char** argv) {
    require(argc==4,"usage: inspect ORIGINAL INSTRUMENTED FRESH_OUTPUT");
    require(!std::ifstream(argv[3]).good(),"fresh inspector output");
    const auto original=read_json(argv[1]),instrumented=read_json(argv[2]);check(original,instrumented);
    auto bad=instrumented;bad.obj.at("private_columns").arr[0].arr[0].scalar="0";reject([&]{check(original,bad);});
    bad=instrumented;bad.obj.at("private_columns").arr.pop_back();reject([&]{check(original,bad);});
    bad=instrumented;auto& coefficient=bad.obj.at("private_columns").arr[0].arr[2];coefficient.scalar=coefficient.integer()==1?"-1":"1";reject([&]{check(original,bad);});
    auto malformed=original;malformed.obj.at("matrix").arr[6].arr[225].scalar="2";reject([&]{derive(malformed);});
    long long sum=0;
    for(const auto& phase:instrumented.at("phase_us").obj) {const auto n=std::stoll(phase.second.scalar);require(n>=0,"nonnegative phase cost");sum+=n;}
    require(sum==std::stoll(instrumented.at("wall_us_before_final_serialization").scalar),"phase sum covers native clock interval");
    std::ofstream out(argv[3]);require(bool(out),"inspector output");
    out<<"{\"standing\":\"producer-unverified separate incidence method\",\"new_columns\":204,\"added_rows\":102,\"private_columns_per_added_row\":2,\"baseline_rows_with_zero_new_support\":6,\"all_certificates_match\":true,\"mutation_controls_rejected\":4,\"phase_interval_us\":"<<sum<<",\"phase_sum_matches\":true}\n";
    require(bool(out),"complete inspector result");return 0;
}
int main(int argc,char** argv) {try{return run(argc,argv);}catch(const std::exception& error){std::cerr<<"REJECT "<<error.what()<<'\n';return 1;}}
