use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::util::raw_ptr::RawPtr;
use crate::util::thread_misc::spin_sleep;

/// (PWM generate loop with SAMPLING) function to be executed from a dedicated pinned thread
///
/// Arguments:
///
/// * `byte` - Raw-pointer with thread borrowing traits as an output target.
/// * `cycle_period` - The full cycle period or 1/freq,  ie: 50hz -> 1/50hz (20ms).
/// * `tick_period` - (Sample period) The minimum in-cycle iteration unit period or cycle_period/ticks ie: 20ms/4e3 (5us).
/// * `duty` - Mutexed Target duty within 0.00 - 1.00 range.
/// * `mask` - Mutexed Bitmask that allows multiple channel control (For now from same generated PWM).
///
pub fn pwm_ctrl(
    byte: RawPtr<u8>,
    cycle_period: Duration,
    tick_period: Duration,
    duty: Arc<[Arc<Mutex<f32>>; 8]>,
    end: Arc<AtomicBool>,
) {
    let ticks: u32 = (cycle_period.as_nanos() / tick_period.as_nanos()) as u32;
    let mut i: u32 = ticks;
    let mut duty_sync_ticks: [u32; 8] = [0; 8];// = array_init(|_| 0_u32);
    let mut now: std::time::Instant;
    let mut blank: u8;

    loop {
        now = std::time::Instant::now();

        if i == ticks {
            if end.load(Ordering::SeqCst) {
                //
                // Active flag TRUE then break loop
                //
                break;
            }

            //
            // Reached cycle end, restart i value (0), update mutexed values;
            //
            i = 0;
            for (j, d) in (*duty).iter().enumerate() {
                duty_sync_ticks[j] = (*d.lock().expect("Mutex duty copy error") * ticks as f32).round() as u32;
            }
        }

        //
        // PWM Output copy bitmask unsafely (raw pointer dereference)
        //

        unsafe {
            blank = 0_u8;
            for (j, dt) in duty_sync_ticks.iter().enumerate() {
                blank |= if i < *dt {
                    (1<<j) as u8
                } else {
                    0x00_u8
                };
            }
            *byte.ptr = blank;
        }

        i += 1;

        //
        // Sleep (tick_period - [ABOVE CODE EXECUTION]_period)
        //
        if now.elapsed() < tick_period {
            spin_sleep(tick_period.saturating_sub(now.elapsed()));
        }
    }
    println!("Successfully ended pwm_ctrl thread main loop task");
}
