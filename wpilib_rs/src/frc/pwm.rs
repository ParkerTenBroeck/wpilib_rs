pub struct PWM {
    handle: i32,
}

#[derive(Debug)]
pub struct PWMError {
    #[allow(unused)]
    err: i32,
}

impl PWM {
    pub fn new(port: u32) -> Result<Self, PWMError> {
        unsafe {
            let handle = crate::bindings::HAL_GetPort(port as i32);

            let mut status = 0;
            let handle = crate::bindings::HAL_InitializePWMPort(
                handle,
                std::ptr::null_mut(),
                &mut status as *mut i32,
            );
            if status != 0 {
                if status == -1156{
                    crate::bindings::HAL_GetLastError(&mut status as *mut i32);
                }
               Err(PWMError { err: status })
            } else {
                Ok(PWM { handle })
            }
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
            let mut status = 0;
            crate::bindings::HAL_SetPWMConfig(
                self.handle,
                max,
                deadband_max,
                center,
                deadband_min,
                min,
                &mut status as &mut i32,
            );
            if status != 0 {
                panic!("Failed to set PWM Config: {status}");
            }
        }
    }

    pub fn turn_off(&self){
        unsafe {
            let mut status = 0;
            crate::bindings::HAL_SetPWMDisabled(
                self.handle,
                &mut status as &mut i32,
            );
            if status != 0 {
                panic!("Failed to set PWM Config");
            }
        }
    }

    pub fn set_speed(&self, speed: f64){
        unsafe{
            let mut status = 0;
            crate::bindings::HAL_SetPWMSpeed(
                self.handle,
                speed,
                &mut status as &mut i32,
            );
            if status != 0 {
                panic!("Failed to set PWM Speed: {status}");
            }
        }
    }

    pub fn get_speed(&self) -> f64{
        // crate::bindings::HAL_PWM
        unsafe{
            let mut status = 0;
            let speed = crate::bindings::HAL_GetPWMSpeed(
                self.handle,
                &mut status as &mut i32,
            );
            if status != 0 {
                panic!("Failed to set PWM Speed: {status}");
            }else{
                speed
            }
        }
    }
}


impl Drop for PWM{
    fn drop(&mut self) {
        unsafe{
            let mut status = 0;
            crate::bindings::HAL_FreePWMPort(self.handle, &mut status as *mut i32);
            if status != 0{
                panic!("{status}");
            }
        }
    }
}