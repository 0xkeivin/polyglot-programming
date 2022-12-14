#include <vector>
#include <stdio.h>

int main(){
    std::vector<int> a;
    std::vector<int> b = a;

    b.push_back(5);

    printf("a.size: %zu\n", a.size()); //0
    printf("b.size: %zu\n", b.size()); //1
}