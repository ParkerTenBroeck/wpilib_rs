use std::{error::Error, fmt::Display};

use crate::{hal::errors::HalError, hal_call, hal_call_s};

pub struct PWM {
    handle: i32,
}

#[derive(Debug)]
pub struct PWMError {
    err: HalError,
}

impl Error for PWMError {}

impl Display for PWMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<HalError> for PWMError {
    fn from(err: HalError) -> Self {
        Self { err }
    }
}

pub type PWMResult<T> = Result<T, PWMError>;

impl PWM {
    pub fn new(port: u32) -> PWMResult<Self> {
        unsafe {
            let handle = hal_call!(HAL_GetPort, port as i32);
            let handle = hal_call_s!(HAL_InitializePWMPort, handle, std::ptr::null())?;
            // create the handle so that if the following methods fail
            // our pwm handle will be properly dropped.
            let mut pwm = PWM { handle };
            pwm.disable()?;
            pwm.set_eliminate_deadband(true)?;

            // hal_call_s!(HAL_Report, crate::bindings::HALUsageReporting_tResourceType_kResourceType_PWM)

            Ok(pwm)
        }
    }

    pub fn set_config(
        &mut self,
        max: f64,
        deadband_max: f64,
        center: f64,
        deadband_min: f64,
        min: f64,
    ) -> PWMResult<()> {
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
            .map_err(|e| e.into())
        }
    }

    pub fn set_eliminate_deadband(&mut self, eliminate_deadband: bool) -> PWMResult<()> {
        unsafe {
            hal_call_s!(
                HAL_SetPWMEliminateDeadband,
                self.handle,
                eliminate_deadband as i32
            )
            .map_err(|e| e.into())
        }
    }

    pub fn get_eliminate_deadband(&mut self) -> PWMResult<bool> {
        unsafe {
            hal_call_s!(HAL_GetPWMEliminateDeadband, self.handle)
                .map(|val| val == 1)
                .map_err(|e| e.into())
        }
    }

    pub fn disable(&self) -> PWMResult<()> {
        unsafe { hal_call_s!(HAL_SetPWMDisabled, self.handle).map_err(|e| e.into()) }
    }

    pub fn set_position(&self, position: f64) -> PWMResult<()> {
        unsafe { hal_call_s!(HAL_SetPWMPosition, self.handle, position).map_err(|e| e.into()) }
    }

    pub fn get_position(&self) -> PWMResult<f64> {
        unsafe { hal_call_s!(HAL_GetPWMPosition, self.handle).map_err(|e| e.into()) }
    }

    pub fn set_speed(&self, speed: f64) -> PWMResult<()> {
        unsafe { hal_call_s!(HAL_SetPWMSpeed, self.handle, speed).map_err(|e| e.into()) }
    }

    pub fn get_speed(&self) -> PWMResult<f64> {
        unsafe { hal_call_s!(HAL_GetPWMSpeed, self.handle).map_err(|e| e.into()) }
    }

    pub fn get_num_pwm_channels() -> i32 {
        unsafe { hal_call!(HAL_GetNumPWMChannels) }
    }

    pub fn get_num_pwm_headers() -> i32 {
        unsafe { hal_call!(HAL_GetNumPWMHeaders) }
    }
}

impl Drop for PWM {
    fn drop(&mut self) {
        unsafe {
            hal_call_s!(HAL_FreePWMPort, self.handle).expect("Failed to free pwm port");
        }
    }
}
