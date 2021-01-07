#include "armstrong_numbers.h"
#include <math.h>
#include <stdlib.h>

bool is_armstrong_number(int candidate)
{
    int digits = floor (log10 (abs (candidate))) + 1;
    int comparison = 0;
    int cand_copy = candidate;
    while(cand_copy > 1){
        int curr_dig = candidate % 10;
        comparison += pow(curr_dig, digits);
        cand_copy /= 10;
    }
    return comparison == candidate;

    if ( candidate == 0 )
    {
        return true;
    }
    if ( candidate >=0 && candidate <=9 )
    {
        return true;
    }
    // Need something to measure the digits, and also go through each digit
    // maybe something recursive that does mod by 10?
    return false;
}
