# DialPWM

Generate and test PWM signals with no dedicated controller through UART & register buffering.
<p align="center">
  <img height=300 src="https://raw.githubusercontent.com/IvnLum/DialPWM/main/anim/N.png" />
</p>


## Direct PWM Generation using CPU
Using **CPU PWM** simulation, it aims to provide a fast way of testing hardware control relying on PWM signals using an USB-UART bridge and a register based buffer as a **passthrough**.

<br/>

### About signal generation precision
Targeted at **non**-RTOS based systems such as Windows, Linux (non-RT versions).

This software relies in the usage of **spin-locks**, **thread-pinning**, and a **non**-atomic shared output bit stream to be read unconditionally from target PWM-controlled HW's

> **Note:** Running it from a VM is discouraged due to the increase of cache misses, and physical-virtual cpu topological cache binding trickery it implies.
Tested with no luck, signals are completely inconsistent.

<br/>

## Building

Clone & Build with rust **cargo**
```bash
cd DialPWM/app
cargo build --release
```

## Testing
> DialPWM [Serial Link] [Baud Rate] [Serial Pinned Thread] [PWM duty period] [PWM tick period] [PWM generator Pinned Thread]
```bash
target/debug/DialPWM -l /dev/ttyUSB1 -b 460800 -s 0 -c 20000 -t 1 -p 1
```

<br/>

## Testbench

### Using physical registers to achieve incoming UART RX data buffering from USB-UART bridge.
Sequential logic brief explanation:

- Registers load **rx** until the **stop condition** is reached
- Detect **"full byte receive"** combinational function will wait for a falling edge clock signal to generate a rising edge one on the parallel load register.
- Main register clear is delayed to avoid parallel register copying cleared content.
  
<p align="center">
  <img height=900 src="https://raw.githubusercontent.com/IvnLum/DialPWM/main/anim/B.png" />
</p>

### Using HDL IP design equivalent (VHDL source included) targeted at Basys3

Design implemented IPs:

- Full UART tx/rx module (no parity)
- Output parallel bits as **std_vector(7 downto 0)** signal
  
<p align="center">
  <img height=850 src="https://raw.githubusercontent.com/IvnLum/DialPWM/main/anim/A.png" />
</p>
