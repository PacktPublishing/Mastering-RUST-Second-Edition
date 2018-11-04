// both_true_false.c

int main(void) {
    bool var;
    if (var) {
        fputs("var is true!\n");
    }
    if (!var) {
        fputs("var is false!\n");
    }
    return 0;
}