// buffer_overflow.c
 
int main() { 
     char buf[3]; 
     buf[0] = 'a';
     buf[1] = 'b';
     buf[2] = 'c';
     buf[3] = 'd';
}
