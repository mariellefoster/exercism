#include "armstrong_numbers.h"


bool is_armstrong_number(int candidate)
{
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
