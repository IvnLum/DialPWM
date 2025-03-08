# DialPWM

### Generate and test PWM signals with no dedicated controller through UART & register buffering.
<p align="center">
  <img height=250 src="https://raw.githubusercontent.com/IvnLum/DialPWM/main/anim/DialPWM_rounded.png" />
</p>

<p align="center">
  <img height=300 src="https://raw.githubusercontent.com/IvnLum/DialPWM/main/anim/DialPWM_v0.1.5_rounded.png" />
</p>

> Totally independent 8 bit pwm control in version 0.1.5

<br/>

## Direct PWM Generation using CPU
Using **CPU PWM** simulation, it aims to provide a fast way of testing hardware control relying on PWM signals using an USB-UART bridge and a register based buffer as a **passthrough**.

<br/>

<p align="center">
  <img height=500 src="https://raw.githubusercontent.com/IvnLum/DialPWM/main/anim/servo.gif" alt="animated" />
</p>

> Testing common servo full sweep operation at 50hz PWM

<br/>

### About signal generation precision
Targeted at **non**-RTOS based systems such as Windows, Linux (non-RT versions).

This software relies in the usage of **spin-locks**, **thread-pinning**, and a **non**-atomic shared output bit stream to be read unconditionally from target PWM-controlled HW's

> [!NOTE]
> Running it from a VM is discouraged due to the increase of cache misses,
> and physical-virtual cpu topological cache binding trickery it implies.
> Tested with no luck, signals are completely inconsistent.

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

## Example Test benches

- ### Using PHYSICAL REGISTERS to achieve incoming UART RX data buffering from USB-UART bridge.
  Sequential logic brief explanation:

  - Registers load **rx** until the **stop condition** is reached
  - Detect **"full byte receive"** combinational function will wait for a falling edge clock signal to generate a rising edge one on the parallel load register.
  - Main register clear is delayed to avoid parallel register copying cleared content.
  
<p align="center">
  <img height=600 src="https://raw.githubusercontent.com/IvnLum/DialPWM/main/anim/registers.png" />
</p>

- ### Using HDL IP design equivalent (VHDL source included) targeted at Basys3
  Design implemented IPs:

  - Full UART tx/rx module (no parity)
  - Output parallel bits as **std_vector(7 downto 0)** signal
  
<p align="center">
  <img height=570 src="https://raw.githubusercontent.com/IvnLum/DialPWM/main/anim/basys3.png" />
</p>


## Used Test bench

- ### Used test bench for development / testing:
  - 1x Digilent's Basys3
  - 1x 2-wire GPU Brushless Fan
  - 1x 4-wire CPU Brushless Fan
  - 1x TIP120 (PWM Control for the 2 wire Fan)
  - 1x MG 996R Servo
  - 5v Power supply (Wall Charger)

<br/>

<p align="center">
  <img height=800 src="https://raw.githubusercontent.com/IvnLum/DialPWM/main/anim/testbench.png" />
</p>

<br/>

- ### Actual Setup:
  Nothing more, nothing less...
<br/>

<p align="center">
  <img height=475 src="https://raw.githubusercontent.com/IvnLum/DialPWM/main/anim/fan.gif" alt="animated" />
</p>

> Testing GPU (2-pin + TIP120)  & CPU 4-pin fans at 500hz PWM

