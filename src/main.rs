extern crate libc;

pub mod egl {
    #![allow(non_camel_case_types)]

    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

    // gl_bindings requires us to define these or the bindings will fail to compile
    use std::os::raw;
    pub type khronos_utime_nanoseconds_t = raw::c_int;
    pub type khronos_uint64_t = u64;
    pub type khronos_ssize_t = isize;
    pub type EGLNativeDisplayType = *const raw::c_void;
    pub type EGLNativePixmapType = *const raw::c_void;
    pub type EGLNativeWindowType = *const raw::c_void;
    pub type EGLint = raw::c_int;
    pub type NativeDisplayType = *const raw::c_void;
    pub type NativePixmapType = *const raw::c_void;
    pub type NativeWindowType = *const raw::c_void;
}

// we need to link
#[link(name = "EGL")]
extern "C" {
    pub fn eglGetProcAddress(s: *const libc::c_char) -> *const libc::c_void;
}

static ATTRIBS: &'static [egl::types::EGLenum] = &[
    egl::RED_SIZE,
    8,
    egl::GREEN_SIZE,
    8,
    egl::BLUE_SIZE,
    8,
    egl::NONE,
];

static PBATTRIBS: &'static [egl::types::EGLenum] = &[egl::HEIGHT, 32, egl::WIDTH, 32, egl::NONE];

use std::ffi::CString;
use std::time::Instant;

fn main() {
    unsafe {
        // Loading non-extension symbols is only valid on EGL 1.5
        egl::load_with(|s| {
            let cs = CString::new(s).unwrap();
            eglGetProcAddress(cs.as_ptr())
        });

        let display = egl::GetDisplay(egl::DEFAULT_DISPLAY);
        egl::Initialize(display, std::ptr::null_mut(), std::ptr::null_mut());

        let mut num_configs = 256;
        let mut config_list =
            vec![std::mem::zeroed::<egl::types::EGLConfig>(); num_configs as usize];

        let c1 = Instant::now();
        egl::ChooseConfig(
            display,
            ATTRIBS.as_ptr() as *const i32,
            config_list.as_mut_ptr(),
            num_configs,
            &mut num_configs,
        );
        let c2 = Instant::now();
        println!("c {:?}", c2 - c1);

        let context = egl::CreateContext(
            display,
            config_list[0],
            egl::NO_CONTEXT,
            std::ptr::null_mut(),
        );

        let surface =
            egl::CreatePbufferSurface(display, config_list[0], PBATTRIBS.as_ptr() as *const i32);

        egl::MakeCurrent(display, surface, surface, context);
        println!("{:?}", egl::GetError()); // 12288 = success

        let mut max_formats = 256;
        let mut format_list = vec![std::mem::zeroed::<egl::types::EGLint>(); max_formats as usize];
        let s1 = Instant::now();
        egl::QueryDmaBufFormatsEXT(
            display,
            max_formats,
            format_list.as_mut_ptr(),
            &mut max_formats,
        );
        let s2 = Instant::now();
        println!("s {:?}", s2 - s1);

        for i in 0..max_formats {
            let mut max_modifiers = 256;
            let mut modifier_list =
                vec![std::mem::zeroed::<egl::types::EGLuint64KHR>(); max_modifiers as usize];
            let mut external_list =
                vec![std::mem::zeroed::<egl::types::EGLBoolean>(); max_modifiers as usize];

            let m1 = Instant::now();
            egl::QueryDmaBufModifiersEXT(
                display,
                format_list[i as usize],
                max_modifiers,
                modifier_list.as_mut_ptr(),
                external_list.as_mut_ptr(),
                &mut max_modifiers,
            );
            let m2 = Instant::now();
            println!("m {:?}", m2 - m1);
        }
    }
}
