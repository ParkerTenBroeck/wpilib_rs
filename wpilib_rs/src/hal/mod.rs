use crate::bindings;
pub mod errors;

#[macro_export]
#[cfg(feature = "export_bindings")]
macro_rules! hal_call {
    ($call_name:ident, $($arg:tt)*) => {
        {
            $crate::bindings::$call_name($($arg)*)
        }
    };

    ($call_name:ident) => {
        {
            $crate::bindings::$call_name()
        }
    };
}

#[macro_export]
#[cfg(feature = "export_bindings")]
macro_rules! hal_call_s {
    ($call_name:ident, $($arg:tt)*) => {
        {
            let mut status = 0;
            let res = $crate::bindings::$call_name($($arg)*, &mut status as *mut i32);

            if status != $crate::hal::errors::HAL_SUCCESS.get_raw() {
                if status == $crate::hal::errors::HAL_USE_LAST_ERROR.get_raw(){
                    $crate::bindings::HAL_GetLastError(&mut status as *mut i32);
                }
               Err($crate::hal::errors::HalError::from_i32(status))
            }else{
                Ok(res)
            }
        }
    };

    ($call_name:ident) => {
        {
            let mut status = 0;
            let res = $crate::bindings::$call_name(&mut status as *mut i32);

            if status != $crate::hal::errors::HAL_SUCCESS.get_raw() {
                if status == $crate::hal::errors::HAL_USE_LAST_ERROR.get_raw(){
                    $crate::bindings::HAL_GetLastError(&mut status as *mut i32);
                }
                Err($crate::hal::errors::HalError::from_i32(status))
            }else{
                Ok(res)
            }
        }
    };
}

///
/// # Panics
///
/// This will panic if initialization fails
///
/// # Safety
/// Dont call this multiple times
///
pub unsafe fn run_hal_initialization() {
    let init_code = bindings::HAL_Initialize(500, 0);
    if init_code != 1 {
        panic!("FATAL ERROR: HAL could not be initialized: {init_code}");
    }
    // bindings::HALUsageReporting::tInstances_kLanguage_CPlusPlus
    // but theres no rust language so I picked 7 whoops
    bindings::HAL_Report(
        bindings::HALUsageReporting_tResourceType_kResourceType_Language,
        7,
        0,
        bindings::GetWPILibVersion(),
    );

    if !bindings::frc_Notifier_SetHALThreadPriority(true, 40) {
        crate::ReportHalErrorBackTrace!(
            crate::frc::errors::Warnings::Warning,
            "Setting HAL Notifier RT priority to 40 failed\n"
        );
    }

    println!("\n********** Robot program starting **********")
}
