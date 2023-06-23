int main() {

    // C-style "for" loop:

    for(int x = 0; x < 42; ++x) {
        loop_body();
    }

    int x = 0;
    while(x < 42) {
        loop_body();
        ++x;
    }
    
    // real computer science for loop:

    for(x : things) {
        loop_body();
    }

    while(there are more things) {
        x = next_thing();
        loop_body();
    }

    // LDA 45,X

}