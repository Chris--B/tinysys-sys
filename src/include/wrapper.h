#include "sdk.h"

// ### These macros are manually defined in Rust, so these checks help us tell when they change.

#if !defined(MAKECOLORRGB12)
    #error "MAKECOLORRGB12" is no longer defined"
#endif

#if !defined(MCONTROL_TYPE)
    #error "MCONTROL_TYPE" is no longer defined"
#endif

#if !defined(MCONTROL_DMODE)
    #error "MCONTROL_DMODE" is no longer defined"
#endif

#if !defined(MCONTROL_MASKMAX)
    #error "MCONTROL_MASKMAX" is no longer defined"
#endif

