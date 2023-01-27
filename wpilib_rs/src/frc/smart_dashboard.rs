use crate::cxx_ffi::ToStringView;

pub fn put_number(key: impl AsRef<str>, value: f64) -> bool {
    let key = key.as_ref();
    unsafe { crate::bindings::frc::SmartDashboard::PutNumber(key.to_string_view(), value) }
}

pub fn put_string(key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
    let key = key.as_ref();
    let value = value.as_ref();
    unsafe {
        crate::bindings::frc::SmartDashboard::PutString(
            key.to_string_view(),
            value.to_string_view(),
        )
    }
}

pub fn put_boolean(key: impl AsRef<str>, value: bool) -> bool {
    let key = key.as_ref();
    unsafe { crate::bindings::frc::SmartDashboard::PutBoolean(key.to_string_view(), value) }
}

pub fn get_number(key: impl AsRef<str>, default: f64) -> f64 {
    let key = key.as_ref();
    unsafe { crate::bindings::frc::SmartDashboard::GetNumber(key.to_string_view(), default) }
}

pub fn get_string(key: impl AsRef<str>, default: impl AsRef<str>) -> String {
    let key = key.as_ref();
    let default = default.as_ref();
    unsafe {
        let str = crate::bindings::frc::SmartDashboard::GetString(
            key.to_string_view(),
            default.to_string_view(),
        );
        crate::cxx_ffi::CxxString::as_native(&str)
    }
}

pub fn get_boolean(key: impl AsRef<str>, default: bool) -> bool {
    let key = key.as_ref();
    unsafe { crate::bindings::frc::SmartDashboard::GetBoolean(key.to_string_view(), default) }
}

pub fn set_default_number(key: impl AsRef<str>, default: f64) -> bool {
    let key = key.as_ref();
    unsafe { crate::bindings::frc::SmartDashboard::SetDefaultNumber(key.to_string_view(), default) }
}

pub fn set_default_string(key: impl AsRef<str>, default: impl AsRef<str>) -> bool {
    let key = key.as_ref();
    let default = default.as_ref();
    unsafe {
        crate::bindings::frc::SmartDashboard::SetDefaultString(
            key.to_string_view(),
            default.to_string_view(),
        )
    }
}

pub fn set_default_boolean(key: impl AsRef<str>, default: bool) -> bool {
    let key = key.as_ref();
    unsafe {
        crate::bindings::frc::SmartDashboard::SetDefaultBoolean(key.to_string_view(), default)
    }
}
