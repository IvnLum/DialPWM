use fltk::prelude::*;
use std::sync::{Arc, Mutex, OnceLock};

pub struct DialCtrl {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub extern_fn: fn(Arc<Mutex<f32>>, f32) -> (),
}

impl DialCtrl {
    fn update_extern(&self, duty: Arc<Mutex<f32>>) {
        let func = self.extern_fn;
        let diff = self.max - self.min;
        func(duty.clone(), self.min + self.value * diff);
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

pub static OUT_SCALE: f32 = 1e-2_f32;
pub static PWM: OnceLock<Arc<[Arc<Mutex<f32>>; 8]>> = OnceLock::new();
pub static DIAL: OnceLock<Arc<[Arc<Mutex<DialCtrl>>; 8]>> = OnceLock::new();

/// FLTK input field text parse as float, assigning it to a referenced Mutexed float
///
/// Arguments:
///
/// * `input` - Input value from FLTK input field
/// * `upd_float` - Mutexed reference to be updated multipling it with a multiplier
/// * `scale` - Direct multiplier specified by this function caller
///
fn input_parse_update_float(input: &mut Input, upd_float: &mut f32, scale: f32) {
    let text = input.value();

    match text.parse::<f32>() {
        Ok(number) => match number {
            x if (0.00..=100.00).contains(&x) => *upd_float = number * scale,
            _ => println!("Number out of range (0.00, 100.00)"),
        },
        Err(_) => println!("Invalid input, please enter a valid number"),
    }
}

/// Update Referenced duty value with the latest parameters values
///
/// Arguments:
///
/// * ()
///
///
fn update_duty_value(idx: usize) {
    DIAL.get().expect("a")[idx]
        .lock()
        .expect("Err")
        .update_extern(PWM.get().expect("")[idx].clone());
}

/// Update A (left sided text from UI) duty callback function
///
/// Arguments:
///
/// * `input` - Input value from FLTK input field, to be parsed as float
///
fn from_ui_set_a_duty(input: &mut Input, idx: usize) {
    input_parse_update_float(
        input,
        &mut (DIAL.get().expect("")[idx].lock().expect("aa").min),
        1e-2_f32,
    );
    update_duty_value(idx);
}

/// Update B (right sided text from UI) duty callback function
///
/// Arguments:
///
/// * `input` - Input value from FLTK input field, to be parsed as float
///
fn from_ui_set_b_duty(input: &mut Input, idx: usize) {
    input_parse_update_float(
        input,
        &mut (DIAL.get().expect("")[idx].lock().expect("aa").max),
        1e-2_f32,
    );
    update_duty_value(idx);
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
fn from_ui_on_dial_change(dial: &mut Dial, idx: usize) {
    let value = dial.value() as f32;
    let current: DialCtrl = *DIAL.get().expect("Arr Error")[idx].lock().expect("");
    DIAL.get().expect("aa")[idx].lock().expect("").value = value;

    let mut duty = current.min + value * (current.max - current.min);
    duty /= OUT_SCALE;

    // Workaround (fltk text label not clearing buffer for shorter text changes)
    let out = if duty < 100.00 {
        format!("{:.2} %", duty)
    } else {
        format!("{:.2}%", duty)
    };

    if let Some(parent) = dial.parent() {
        if let Some(child) = parent.child(25 + idx as i32) {
            if let Some(mut output) = Output::from_dyn_widget(&child) {
                output.set_label(&out);
            } else {
                println!("Error: Widget is not an Output.");
            }
        } else {
            println!("Error: No widget found at index {}", 25 + idx as i32);
        }
    } else {
        println!("Error: No referenced parent");
    }

    update_duty_value(idx);
}

// FLTK widgets do NOT pass more than a single self arg, no index

fn from_ui_on_dial_change_0(dial: &mut Dial) {
    from_ui_on_dial_change(dial, 0);
}
fn from_ui_on_dial_change_1(dial: &mut Dial) {
    from_ui_on_dial_change(dial, 1);
}
fn from_ui_on_dial_change_2(dial: &mut Dial) {
    from_ui_on_dial_change(dial, 2);
}
fn from_ui_on_dial_change_3(dial: &mut Dial) {
    from_ui_on_dial_change(dial, 3);
}
fn from_ui_on_dial_change_4(dial: &mut Dial) {
    from_ui_on_dial_change(dial, 4);
}
fn from_ui_on_dial_change_5(dial: &mut Dial) {
    from_ui_on_dial_change(dial, 5);
}
fn from_ui_on_dial_change_6(dial: &mut Dial) {
    from_ui_on_dial_change(dial, 6);
}
fn from_ui_on_dial_change_7(dial: &mut Dial) {
    from_ui_on_dial_change(dial, 7);
}

fn from_ui_set_b_duty_0(input: &mut Input) {
    from_ui_set_b_duty(input, 0);
}
fn from_ui_set_b_duty_1(input: &mut Input) {
    from_ui_set_b_duty(input, 1);
}
fn from_ui_set_b_duty_2(input: &mut Input) {
    from_ui_set_b_duty(input, 2);
}
fn from_ui_set_b_duty_3(input: &mut Input) {
    from_ui_set_b_duty(input, 3);
}
fn from_ui_set_b_duty_4(input: &mut Input) {
    from_ui_set_b_duty(input, 4);
}
fn from_ui_set_b_duty_5(input: &mut Input) {
    from_ui_set_b_duty(input, 5);
}
fn from_ui_set_b_duty_6(input: &mut Input) {
    from_ui_set_b_duty(input, 6);
}
fn from_ui_set_b_duty_7(input: &mut Input) {
    from_ui_set_b_duty(input, 7);
}

fn from_ui_set_a_duty_0(input: &mut Input) {
    from_ui_set_a_duty(input, 0);
}
fn from_ui_set_a_duty_1(input: &mut Input) {
    from_ui_set_a_duty(input, 1);
}
fn from_ui_set_a_duty_2(input: &mut Input) {
    from_ui_set_a_duty(input, 2);
}
fn from_ui_set_a_duty_3(input: &mut Input) {
    from_ui_set_a_duty(input, 3);
}
fn from_ui_set_a_duty_4(input: &mut Input) {
    from_ui_set_a_duty(input, 4);
}
fn from_ui_set_a_duty_5(input: &mut Input) {
    from_ui_set_a_duty(input, 5);
}
fn from_ui_set_a_duty_6(input: &mut Input) {
    from_ui_set_a_duty(input, 6);
}
fn from_ui_set_a_duty_7(input: &mut Input) {
    from_ui_set_a_duty(input, 7);
}

fl2rust_macro::include_ui! {"src/app_ui/res/ui.fl"}
