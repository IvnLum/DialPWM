use fltk::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

type UpdateDutyFnMutexTuple = (fn(Arc<Mutex<f32>>, f32) -> (), Arc<[Arc<Mutex<f32>>;8]>);

pub struct DialCtrl {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub scale: f32,
}
impl DialCtrl {
    fn new(value:f32, min:f32, max:f32, scale:f32) -> DialCtrl {
        DialCtrl {value:value, min:min, max:max, scale:scale}
    }
}

unsafe impl Sync for DialCtrl {}
impl Clone for DialCtrl {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for DialCtrl {}
unsafe impl Send for DialCtrl {}

pub static UI_UPDATE_DUTY_FN: OnceLock<UpdateDutyFnMutexTuple> = OnceLock::new();
static DIAL_TXT_REF: OnceLock<Arc<Mutex<Widget>>> = OnceLock::new();
static DIAL_TXT_REF_SET: AtomicBool = AtomicBool::new(false);

/// FLTK input field text parse as float, assigning it to a referenced Mutexed float
///
/// Arguments:
///
/// * `input` - Input value from FLTK input field
/// * `upd_float` - Mutexed reference to be updated multipling it with a multiplier
/// * `scale` - Direct multiplier specified by this function caller
///
fn input_parse_update_float(input: &mut Input, upd_float: &Mutex<f32>, scale: f32) {
    let text = input.value();

    match text.parse::<f32>() {
        Ok(number) => match number {
            x if (0.00..=100.00).contains(&x) => {
                *upd_float.lock().expect("Mutex update float Err") = number * scale
            }
            _ => println!("Number out of range (0.00, 100.00)"),
        },
        Err(_) => println!("Invalid input, please enter a valid number"),
    }
}

/// FLTK input field text parse as a bitmask, assigning it to a referenced Mutexed bitmask
///
/// Arguments:
///
/// * `input` - Input value from FLTK input field
/// * `upd_u8` - Mutexed reference to be updated
///
fn input_parse_update_mask(input: &mut Input, upd_u8: &Mutex<u8>) {
    let text = input.value();
    let result = u8::from_str_radix(&text, 2).map_err(|_| "Invalid binary input".to_string());

    match result {
        Ok(value) => *upd_u8.lock().expect("Mutex update bitmask Err") = value,
        Err(_) => println!("Invalid input, please enter a valid bitmask"),
    }
}

/// Merge referenced (function, mutexed_value) tuple, by executing it unwrapping the value itself #1
///
/// Arguments:
///
/// * ()
///
fn handle_ref_fn_call_0(idx: usize) {
    let (func, v) = UI_UPDATE_DUTY_FN
        .get()
        .expect("Duty update handler NOT set");
    func(v.clone()[idx].clone(), *CURRENT_DUTY.lock().expect("Mutex err"));
}
/*
/// Merge referenced (function, mutexed_value) tuple, by executing it unwrapping the value itself #2
///
/// Arguments:
///
/// * ()
///
fn handle_ref_fn_call_1() {
    let (func, v) = UI_UPDATE_MASK_FN
        .get()
        .expect("Mask update handler NOT set");
    func(v.clone(), *CURRENT_MASK.lock().expect("Mutex err"));
}*/

/// Update dial labeled current duty value
///
/// Arguments:
///
/// * ()
///
fn to_ui_update_last_duty() {
    if !DIAL_TXT_REF_SET.load(Ordering::SeqCst) {
        return;
    }
    let duty = *CURRENT_DUTY
        .lock()
        .expect("Buffered value Mutex lock Error")
        / SCALE;
    // Workaround (fltk text label not clearing buffer for shorter text changes)
    let out = if duty == 100.00 {
        format!("{:.2}%", duty)
    } else {
        format!("{:.2} %", duty)
    };

    if let Some(mut output) = Output::from_dyn_widget(
        &*DIAL_TXT_REF
            .get()
            .expect("Get Mutexed Widget Error")
            .lock()
            .expect("Mutexed Widget Error"),
    ) {
        output.set_label(&out);
    } else {
        println!("Error: Widget is not an Output.");
    }
}

/// Update Referenced duty value with the latest parameters values
///
/// Arguments:
///
/// * ()
///
///
fn update_duty_value() {
    let value = *DIAL_BUFFERED_VALUE
        .lock()
        .expect("Load buffered dial value Error");
    let a = *A_DUTY.lock().expect("Load A duty Error");
    let diff = *B_DUTY.lock().expect("Load B duty Error") - a;
    *CURRENT_DUTY.lock().expect("Err") = a + value * diff;

 //   to_ui_update_last_duty();
    handle_ref_fn_call_0(0);
}

/// Update A (left sided text from UI) duty callback function
///
/// Arguments:
///
/// * `input` - Input value from FLTK input field, to be parsed as float
///
fn from_ui_set_a_duty(input: &mut Input) {
    input_parse_update_float(input, &A_DUTY, SCALE);
    update_duty_value();
}

/// Update B (right sided text from UI) duty callback function
///
/// Arguments:
///
/// * `input` - Input value from FLTK input field, to be parsed as float
///
fn from_ui_set_b_duty(input: &mut Input) {
    input_parse_update_float(input, &B_DUTY, SCALE);
    update_duty_value();
}
/*
/// Update bitmask callback function
///
/// Arguments:
///
/// * `input` - Input value from FLTK input field
///
fn from_ui_set_mask(input: &mut Input) {
    input_parse_update_mask(input, &CURRENT_MASK);
}

/// Set bitmask callback function that effectively changes the current mask to the updated one
///
/// Arguments:
///
/// * `_` - Ignored button event content
///
///
fn from_ui_update_mask(_: &mut Button) {
    handle_ref_fn_call_1();
}*/

/// Update Dial callback function
///
/// Arguments:
///
/// * `dial` - Dial value from FLTK input field needed to obtain within range 0.00 - 1.00 target value
///
fn from_ui_on_dial_change(dial: &mut Dial) {
    let value = dial.value() as f32;
    *DIAL_BUFFERED_VALUE
        .lock()
        .expect("Load buffered dial value Error") = value;
/*
    if !DIAL_TXT_REF_SET.load(Ordering::SeqCst) {
        if let Some(parent) = dial.parent() {
            if let Some(child) = parent.child(6) {
                DIAL_TXT_REF
                    .set(Arc::new(Mutex::new(child.clone())))
                    .unwrap();
                DIAL_TXT_REF_SET.store(true, Ordering::SeqCst);
            } else {
                println!("Error: No widget found at index {}", 6);
            }
        } else {
            println!("Error: No referenced parent");
        }
    }*/

    update_duty_value();
}

fl2rust_macro::include_ui! {"src/ui/res/ui.fl"}
