#include <iostream>
#include <string>
#include <vector>

extern "C" {
    char* koda_full_process(const char* input);
    void koda_free_string(char* p);
}

class Koda {
public:
    static std::string parse(const std::string& input) {
        char* result = koda_full_process(input.c_str());
        std::string converted(result);
        koda_free_string(result);
        return converted;
    }
};

int main() {
    std::string data = "version: !git rev-parse HEAD";
    std::cout << Koda::parse(data) << std::endl;
    return 0;
}
