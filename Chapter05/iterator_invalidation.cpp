
#include <iostream>
#include <vector>

int main() { 
  
    std::vector <int> v{1, 5, 10, 15, 20}; 
  
    for (auto it=v.begin();it!=v.end();it++) 
        if ((*it) == 5) 
            v.push_back(-1); 
  
    for (auto it=v.begin();it!=v.end();it++) 
        std::cout << (*it) << " "; 
          
    return 0;     
}
