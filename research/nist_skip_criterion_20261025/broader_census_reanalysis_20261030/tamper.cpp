#include "io.hpp"
#include <iostream>
int main(int argc,char** argv) {
try {
    require(argc == 4,"tamper KIND INPUT NEW_COPY");
    std::string kind=argv[1],file; size_t row=1,column=0;
    if (kind == "character") { file="characters.tsv";row=2;column=2; }
    else if (kind == "profile") { file="profiles.tsv";column=5; }
    else if (kind == "cutoff") { file="cutoffs.tsv";column=3; }
    else throw std::runtime_error("unknown corruption");
    fs::path out=argv[3]; require(!fs::exists(out),"fresh corruption copy");
    fs::copy(argv[2],out,fs::copy_options::recursive);
    std::istringstream input(read_file(out/file));std::ostringstream changed;std::string line;
    size_t at=0;bool done=false;
    while (std::getline(input,line)) {
        if (at++ == row) {
            auto c=cells(line);require(column<c.size(),"corruption column");
            c[column]=std::to_string(std::stoi(c[column])+1);tsv(changed,c);done=true;
        } else changed << line << '\n';
    }
    require(done,"corruption target");put(out/file,changed.str());
    std::cout << "Prepared one deliberate corruption: " << kind << '\n';return 0;
} catch (const std::exception& error) { std::cerr << "FAIL: " << error.what() << '\n';return 1; }
}
