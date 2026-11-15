// Fixed first end-to-end integration control; production recovery is reused.
#define NORM_CORE_NO_MAIN
#include "../norm_core_elimination_20261115/core.cpp"
int main(int argc,char**argv){
    try{
        if(argc!=3)throw std::invalid_argument("usage: cold PC_BACKEND NEW_ROOT");
        const auto root=std::filesystem::absolute(argv[2]);
        if(!std::filesystem::create_directory(root))throw std::runtime_error("reused cold path");
        core::Oracle oracle(argv[1],root/"target-x0-y6");
        const auto out=algebraic::cold_solve({0,6,false},1234,oracle);
        std::ofstream summary(root/"summary.tsv");
        summary<<"target_x\ttarget_y\trecovered_scalar\tattempts\tbackend_calls\tmisses\trank\n";
        summary<<"0\t6\t"<<out.scalar<<'\t'<<out.attempts<<'\t'<<out.calls<<'\t'<<out.misses<<'\t'<<out.rank<<'\n';
        summary.close();if(!summary)throw std::runtime_error("cold summary write failed");
        std::cout<<"PASS first cold point-only recovery; scalar="<<out.scalar<<" calls="<<out.calls<<'\n';return 0;
    }catch(const std::exception&e){std::cerr<<"UNKNOWN: "<<e.what()<<'\n';return 2;}
}
