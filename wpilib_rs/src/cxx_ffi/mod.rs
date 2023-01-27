use std::fmt::Debug;

#[allow(unused)]
#[cfg(target_pointer_width = "64")]
pub type PtrInt = i64;
#[allow(unused)]
#[cfg(target_pointer_width = "64")]
pub type PtrUInt = u64;
#[allow(unused)]
#[cfg(target_pointer_width = "32")]
pub type PtrInt = i32;
#[allow(unused)]
#[cfg(target_pointer_width = "32")]
pub type PtrUInt = u32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[cfg(target_pointer_width = "32")]
struct CxxStringShort {
    ptr: *mut u8,
    length: u32,
    chars: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[cfg(target_pointer_width = "64")]
struct CxxStringShort {
    ptr: *mut u8,
    length: u64,
    chars: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[cfg(target_pointer_width = "32")]
struct CxxStringLong {
    ptr: *mut u8,
    length: u32,
    capacity: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[cfg(target_pointer_width = "64")]
struct CxxStringLong {
    ptr: *mut u8,
    capacity: u32,
    length: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct ByteField {
    // #[cfg(target_endian = "big")]
    __padding: [u8; 3],
    byte: u8,
}

#[repr(C)]
pub union CxxString {
    short: CxxStringShort,
    long: CxxStringLong,
    bits: ByteField,
}

impl CxxString {
    /// # Safety
    /// This "struct" contains a self refering field such that if copied to another location
    /// other than the immediate return when calling a function that generates the input stuct
    /// will break it...
    ///
    /// So if you wish to call this be sure NOT to move it and pass it immedietly to this function
    ///
    /// Discard the original array immediatly as calling this again with the same value will allow for a double free
    ///
    #[inline(never)]
    pub unsafe fn as_native(string: *const crate::bindings::std::string) -> String {
        let self_ref: *const Self = std::mem::transmute(string);
        if (*self_ref).short.ptr as *const u8 != (*self_ref).short.chars.as_ptr() {
            String::from_raw_parts(
                (*self_ref).long.ptr,
                (*self_ref).long.length as usize,
                (*self_ref).long.capacity as usize,
            )
        } else {
            let mut string = String::with_capacity((*self_ref).short.length as usize);
            let vec = string.as_mut_vec();
            for byte in (*self_ref).short.chars {
                if byte == 0 {
                    break;
                }
                vec.push(byte);
            }
            string
        }
    }
}

impl Debug for CxxString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            f.debug_struct("CxxStringUnion")
                .field("short", &self.short)
                .field("long", &self.long)
                .field("lsb", &self.bits)
                .finish()
        }
    }
}

pub trait ToStringView {
    fn to_string_view(self) -> crate::bindings::std::string_view;
}

impl<'a> ToStringView for &'a str {
    fn to_string_view(self) -> crate::bindings::std::string_view {
        [
            self.as_bytes().len() as PtrUInt,
            self.as_bytes().as_ptr() as PtrUInt,
        ]
    }
}
