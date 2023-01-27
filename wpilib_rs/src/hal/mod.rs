use crate::bindings;

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
        bindings::HALUsageReporting::tResourceType_kResourceType_Language,
        7,
        0,
        bindings::GetWPILibVersion(),
    );

    if !bindings::frc::Notifier::SetHALThreadPriority(true, 40) {
        crate::ReportHalErrorBackTrace!(
            crate::frc::errors::Warnings::Warning,
            "Setting HAL Notifier RT priority to 40 failed\n"
        );
    }

    println!("\n********** Robot program starting **********")
}
