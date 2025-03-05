# DialPWM

Generate and test PWM signals with no dedicated controller through UART & register buffering.


## Direct PWM Generation using CPU
Using **CPU PWM** simulation, it aims to provide a fast way of testing hardware control relying on PWM signals using simple registers based buffer as a **passthrough**.

<br/>

### About signal generation precision
Targeted at **non**-RTOS based systems such as Windows, Linux (non-RT versions).

This software relies in the usage of **spin-locks**, **thread-pinning**, and a **non**-atomic shared output bit stream to be read unconditionally from target PWM-controlled HW's

> **Note:** Running it from a VM is discouraged due to the increase of cache misses, and physical-virtual cpu topological cache binding trickery it implies.
Tested with no luck, signals are completely inconsistent.
