use crate::{hal::HalError, hal_call, hal_call_s};

pub struct PWM {
    handle: i32,
}

impl PWM {
    pub fn new(port: u32) -> Result<Self, HalError> {
        unsafe {
            let handle = hal_call!(HAL_GetPort, port as i32);
            let handle = hal_call_s!(HAL_InitializePWMPort, handle, std::ptr::null())?;
            hal_call_s!(HAL_SetPWMDisabled, handle)?;
            hal_call_s!(HAL_SetPWMEliminateDeadband, handle, false as i32)?;

            Ok(PWM { handle })
        }
    }

    pub fn set_config(
        &mut self,
        max: f64,
        deadband_max: f64,
        center: f64,
        deadband_min: f64,
        min: f64,
    ) {
        unsafe {
            hal_call_s!(
                HAL_SetPWMConfig,
                self.handle,
                max,
                deadband_max,
                center,
                deadband_min,
                min
            )
            .expect("Failed to set PWM config");
        }
    }

    pub fn turn_off(&self) {
        unsafe {
            hal_call_s!(HAL_SetPWMDisabled, self.handle).expect("Failed to disable PWM port");
        }
    }

    pub fn set_speed(&self, speed: f64) {
        unsafe {
            hal_call_s!(HAL_SetPWMSpeed, self.handle, speed).expect("Failed to set PWM speed");
        }
    }

    pub fn get_speed(&self) -> f64 {
        unsafe { hal_call_s!(HAL_GetPWMSpeed, self.handle).expect("Failed to get PWM speed") }
    }
}

impl Drop for PWM {
    fn drop(&mut self) {
        unsafe {
            hal_call_s!(HAL_FreePWMPort, self.handle).expect("Failed to free pwm port");
        }
    }
}
