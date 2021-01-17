#include "armstrong_numbers.h"
#include <math.h>

bool is_armstrong_number(int candidate)
{
    int digits = floor (log10 (abs (candidate))) + 1;
    int comparison = 0;
    int cand_copy = candidate;
    while(cand_copy > 0){
        int curr_dig = cand_copy % 10;
        comparison += pow(curr_dig, digits);
        cand_copy /= 10;
    }
    return comparison == candidate;
}