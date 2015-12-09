#include <math.h>
#include "stdio.h"
#include "stdbool.h"
#include "string.h"
#include "stdlib.h"

int main(int argc, char **argv)
{
    int n = atoi(argv[1]);
    bool numbers[n];
    memset(numbers, true, sizeof(numbers));
    
    int i;
    for (i = 2; i < sqrt(n); i++) {
        if (numbers[i] == true) {
            int j, cnt;
            for (j = i*i; j <= n; j += i){
                numbers[j] = false;
            }
        }
    }

    int mirpCount = 0;
    int j;
    for (i = 2; i < n; i++) {
        if ( numbers[i] == true) {
            int reversed = 0;
            j = i; 
            while (j != 0) {
                reversed = (reversed * 10) + (j  % 10);
                j = j / 10;
            }
            if ( reversed != i ) { // Not a palindrome and a prime
                if ( reversed < n && numbers[reversed] == true) {
                    mirpCount++;
                }
            }

        }
    }
    printf("Mirp count : %d\n", mirpCount);
}

