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
    return false;
}
