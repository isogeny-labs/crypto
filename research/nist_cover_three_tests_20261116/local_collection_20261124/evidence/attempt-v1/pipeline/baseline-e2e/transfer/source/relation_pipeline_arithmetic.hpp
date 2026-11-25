#pragma once
#include "public_classes_arithmetic.hpp"
#include <chrono>
#include <ctime>

struct RPStage {std::string name;long long wall_us,cpu_us;};
struct RPTimer {
    std::chrono::steady_clock::time_point wall=std::chrono::steady_clock::now();
    std::clock_t cpu=std::clock();
    std::vector<RPStage> stages;
    void mark(const std::string& name){
        auto now=std::chrono::steady_clock::now();auto ticks=std::clock();
        stages.push_back({name,std::chrono::duration_cast<std::chrono::microseconds>(now-wall).count(),
            static_cast<long long>(1000000.0*(ticks-cpu)/CLOCKS_PER_SEC)});
        wall=now;cpu=ticks;
    }
};
void rp_point(std::ostream& out,const EC& p){
    out<<"{\"infinity\":"<<(p.zero?"true":"false");
    if(!p.zero)out<<",\"x\":\""<<scalar(p.x).x<<"\",\"y\":\""<<scalar(p.y).x<<'"';
    out<<'}';
}
void rp_pool(std::ostream& out,const std::vector<BasePoint>& points,const std::vector<Fiber>& fibers){
    out<<"{\"points\":[";
    for(size_t i=0;i<points.size();++i){if(i)out<<',';out<<"{\"x\":\""<<points[i].x<<"\",\"y\":\""<<points[i].y.x<<"\"}";}
    out<<"],\"split_fibers\":[";
    for(size_t i=0;i<fibers.size();++i){if(i)out<<',';out<<"{\"x\":"<<fibers[i].x<<",\"point_indices\":[";
        for(size_t j=0;j<fibers[i].support.size();++j){out<<(j?",":"")<<fibers[i].support[j];}out<<"]}";}
    out<<"]}";
}
