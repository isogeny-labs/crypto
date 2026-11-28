#include "halving.hpp"
int main(int argc,char**argv) {
    try {
        if(argc!=3)throw std::invalid_argument("usage: generated-cold BACKEND NEW_ROOT");
        const auto root=std::filesystem::absolute(argv[2]);
        halving::check(std::filesystem::create_directory(root),"reused generated cold root");
        std::ofstream events(root/"events.txt");
        halving::Sampler sampler(19,1234,true,events);const auto setup=sampler.generate();
        halving::receipt(setup,root);events.close();halving::check(bool(events),"cold setup events");
        auxiliary::GraphOracle oracle(argv[1],root/"target-x18-y1",setup.tower);
        const auto out=auxiliary::cold({18,1,false},5678,oracle,setup.tower);
        std::ofstream summary(root/"summary.tsv");
        summary<<"target_x\ttarget_y\trecovered_scalar\tattempts\tbackend_calls\tmisses\trank\n";
        summary<<"18\t1\t"<<out.scalar<<'\t'<<out.attempts<<'\t'<<out.calls<<'\t'<<out.misses<<'\t'<<out.rank<<'\n';
        summary.close();halving::check(bool(summary),"generated cold summary write");
        std::cout<<"PASS generated-setup cold scalar="<<out.scalar<<" trials="<<setup.trials
                 <<" attempts="<<out.attempts<<" calls="<<out.calls<<'\n';return 0;
    }catch(const std::exception&e){std::cerr<<"UNKNOWN: "<<e.what()<<'\n';return 2;}
}
