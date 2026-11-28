#pragma once
#include "opaque_km_boundary_arithmetic.hpp"
#include "normalization_arithmetic.hpp"
#include <fstream>
EBPoly model_curve(const Json& model) {
    EBPoly f;for(const auto& row:model.at("terms").arr) {
        const auto& e=row.at("exponents").arr;f.add({e[0].integer(),e[1].integer()},Z(mpz_class(row.at("coefficient").scalar)));
    }return f;
}
struct Context {
    Json model,relations,formal,actual;
    QuarticKM km;EBPoly f;CMAdapter adapter;
    explicit Context(char** argv):model(read_json(argv[2])),relations(read_json(argv[3])),formal(read_json(argv[4])),actual(read_json(argv[5])),km(model,relations.at("pool")),f(model_curve(model)),adapter(km,f,formal) {}
};
Class class_input(const Json& rows,QuarticKM& km) {
    for(const auto& row:rows.arr)for(const auto& entry:row.arr) { mpz_class n(entry.scalar);require(n>=0&&n<prime,"canonical supplied class coefficient"); }
    return gb_class(rows,km);
}
MCycle stored_cycle(const Json& row) {
    MCycle c;c.dimension=row.at("dimension").integer();c.coordinate=row.at("coordinate_lambda").integer();
    c.u=pc_read_poly(row.at("support"));c.x=pc_read_poly(row.at("x"));c.y=pc_read_poly(row.at("y"));return c;
}
void point_json(const BasePoint& p) { std::cout<<"{\"x\":\""<<p.x<<"\",\"y\":\""<<p.y.x<<"\"}"; }
struct OutputFile {
    std::ofstream out;std::streambuf* previous;
    explicit OutputFile(const std::string& path):out(path),previous(std::cout.rdbuf()) { require(bool(out),"create fixture file");std::cout.rdbuf(out.rdbuf()); }
    ~OutputFile() { std::cout.rdbuf(previous); }
};
template<class Expectation> void fixture(const std::string& dir,const std::string& id,const Class& value,Expectation expected) {
    OutputFile output(dir+"/"+id+".json");std::cout<<"{\"case_id\":\""<<id<<"\",\"class_space\":";pc_emit_class(value);std::cout<<",\"expectation\":";expected();std::cout<<"}\n";
}
