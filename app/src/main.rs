use clap::Parser;
use fltk::*;
use serialport::new;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod pwm;
mod serial_ctrl;
mod ui;
mod util;

#[derive(Parser, Default, Debug)]
#[command(
    author = "Ivan Lumbano Vivar",
    version = "0.1.0",
    about = "Fast Dial based PWM control"
)]
struct Args {
    /// Serial link [Unix: /dev/tty[]|pts; Windows: COM[]]
    #[arg(short, long)]
    link: String,
    /// Serial link baud rate
    #[arg(short, long)]
    baud_rate: u32,
    /// Serial write handler thread ID [0-99]
    #[arg(short, long)]
    serial_write_thread_id: usize,
    /// PWM Full cycle period in ms
    #[arg(short, long)]
    cycle_period: u32,
    /// PWM tick period in us
    #[arg(short, long)]
    tick_period: u32,
    /// PWM ctrl handler thread ID [0-99]
    #[arg(short, long)]
    pwm_ctrl_thread_id: usize,
}

/// To be referenced Update PWM duty function that acts as an intermediate logic call function
///
/// It can be updated without involving UI internal handling.
///
/// Arguments:
///
/// * `duty` - Thread safe Mutexed reference to the target duty to be updated.
/// * `Val` - Duty target value.
///
fn update_duty(duty: Arc<Mutex<f32>>, val: f32) {
    *duty.lock().expect("Duty update failed") = val;
}

/// To be referenced Update PWM Bitmask function that acts as an intermediate logic call function
///
/// It can be updated without involving UI internal handling.
///
/// Arguments:
///
/// * `mask` - Thread safe Mutexed reference to the target bitmask to be updated.
/// * `Val` - bitmask target value.
fn update_mask(mask: Arc<Mutex<u8>>, val: u8) {
    *mask.lock().expect("Mask update failed") = val;
}

fn main() {
    //
    // Args parse (serial config & thread id handling).
    //

    let args = Args::parse();
    let (link_name, baud_rate, cycle_period, tick_period) = (
        args.link,
        args.baud_rate,
        args.cycle_period,
        args.tick_period,
    );
    let (serial_th, pwm_th) = (args.serial_write_thread_id, args.pwm_ctrl_thread_id);

    //
    // No Mutexed I/O direct stream Byte pointer (simulate I/O stream between threads).
    //
    // Since we want to simulate async serial stream there is no need for locking values
    // (reading intermediate writes are also expected).
    //
    // Also unsafe by definition since it can be used by threads that may outlive referenced value
    // owner thread (Not this case).
    //
    let end_flag = Arc::new(AtomicBool::new(false));

    let mut byte = 0_u8;
    let raw_ptr: util::raw_ptr::RawPtr<u8> = util::raw_ptr::RawPtr {
        ptr: &mut byte as *mut u8,
    };

    //
    // Mutexed pwm duty & mask control (GUI update & stdin).
    //

    let shared_mask = Arc::new(Mutex::new(0x00_u8));
    let shared_pwm = Arc::new(Mutex::new(0_f32));

    let (s_efl0, s_efl1) = (end_flag.clone(), end_flag.clone());

    let (s_pwm0, s_pwm1) = (shared_pwm.clone(), shared_pwm.clone());

    let (s_mask0, s_mask1) = (shared_mask.clone(), shared_mask.clone());

    //
    // Mutexed serial (safe control between threads)
    //

    let serial = Arc::new(Mutex::new(
        new(link_name, baud_rate)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open serial port"),
    ));

    //
    // Pinned Threads
    //
    // (Lock OS thread affinity exchange)
    //
    // If NOT locked, generated pwm would NOT be inside timmings generating
    // INVALID / UNEXPECTED simulated values
    //

    let writer_thread = thread::spawn(move || {
        util::thread_misc::pin_thread_to_core(serial_th);
        serial_ctrl::write_serial(serial.clone(), raw_ptr, s_efl0);
    });

    let update_thread = thread::spawn(move || {
        util::thread_misc::pin_thread_to_core(pwm_th);
        pwm::pwm_ctrl(
            raw_ptr,
            Duration::from_micros(cycle_period as u64),
            Duration::from_micros(tick_period as u64),
            s_pwm0,
            s_mask0,
            s_efl1,
        );
    });

    //
    // Share duty and mask references to UI handler functions
    //

    ui::ui::UI_UPDATE_DUTY_FN
        .set((update_duty, s_pwm1))
        .unwrap();
    ui::ui::UI_UPDATE_MASK_FN
        .set((update_mask, s_mask1))
        .unwrap();

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let _ = ui::ui::UserInterface::make_window();

    app.run().unwrap();

    end_flag.store(true, Ordering::SeqCst);

    writer_thread.join().expect("Writer thread crashed");
    update_thread.join().expect("Update thread crashed");
}
