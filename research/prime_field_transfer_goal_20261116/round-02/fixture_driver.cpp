// Isolate unchanged correctness fixtures; never change the evaluator's inputs.
#define main aggregate_fixture_main
#include "local_torsion.cpp"
#undef main
int main(int argc,char** argv) {
    std::cout.setf(std::ios::unitbuf);
    try {
        check(argc==2,"choose p17, p257, or secp256k1");
        const std::string which=argv[1];
        if (which=="p17") fixture("p17-r19",Model(17,19,{0,0,0,2,2}),{false,5,1},true);
        else if (which=="p257") fixture("p257-r83",Model(257,83,{0,0,0,23,11}),{false,7,1},true);
        else if (which=="secp256k1") {
            Model model(Z("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",16),
                        Z("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",16),
                        {0,0,0,0,7});
            Point generator{false,
                Z("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",16),
                Z("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",16)};
            fixture("secp256k1",model,generator,false);
        } else check(false,"unknown fixture");
        return 0;
    } catch (const std::exception& e) { std::cerr<<e.what()<<'\n';return 1; }
}
