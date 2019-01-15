// c_macros.c

#include <stdio.h> 

#define SWITCH(a, b) { temp = b; b = a; a = temp; } 

int main() { 
    int x=1; 
    int y=2; 
    int temp = 3; 

    SWITCH(x, y); 
    printf("x is now %d. y is now %d. temp is now %d\n", x, y, temp); 
}
