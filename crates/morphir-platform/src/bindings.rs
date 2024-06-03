// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod morphir {
        #[allow(dead_code)]
        pub mod platform {
            #[allow(dead_code, clippy::all)]
            pub mod workspace {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                #[doc(hidden)]

                macro_rules! __export_morphir_platform_workspace_0_1_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _: () = {};
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_morphir_platform_workspace_0_1_0_cabi;
            }

            #[allow(dead_code, clippy::all)]
            pub mod project {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Clone)]
                pub struct ProjectInfo {
                    pub name: _rt::String,
                    pub source_directory: _rt::String,
                    pub includes: _rt::Vec<_rt::String>,
                    pub dependencies: _rt::Vec<_rt::String>,
                    pub local_dependencies: _rt::Vec<_rt::String>,
                }
                impl ::core::fmt::Debug for ProjectInfo {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("ProjectInfo")
                            .field("name", &self.name)
                            .field("source-directory", &self.source_directory)
                            .field("includes", &self.includes)
                            .field("dependencies", &self.dependencies)
                            .field("local-dependencies", &self.local_dependencies)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_project_info_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::get_project_info(_rt::string_lift(bytes0));
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let ProjectInfo {
                                name: name3,
                                source_directory: source_directory3,
                                includes: includes3,
                                dependencies: dependencies3,
                                local_dependencies: local_dependencies3,
                            } = e;
                            let vec4 = (name3.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr2.add(8).cast::<usize>() = len4;
                            *ptr2.add(4).cast::<*mut u8>() = ptr4.cast_mut();
                            let vec5 = (source_directory3.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr2.add(16).cast::<usize>() = len5;
                            *ptr2.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                            let vec7 = includes3;
                            let len7 = vec7.len();
                            let layout7 =
                                _rt::alloc::Layout::from_size_align_unchecked(vec7.len() * 8, 4);
                            let result7 = if layout7.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout7).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout7);
                                }
                                ptr
                            } else {
                                {
                                    ::core::ptr::null_mut()
                                }
                            };
                            for (i, e) in vec7.into_iter().enumerate() {
                                let base = result7.add(i * 8);
                                {
                                    let vec6 = (e.into_bytes()).into_boxed_slice();
                                    let ptr6 = vec6.as_ptr().cast::<u8>();
                                    let len6 = vec6.len();
                                    ::core::mem::forget(vec6);
                                    *base.add(4).cast::<usize>() = len6;
                                    *base.add(0).cast::<*mut u8>() = ptr6.cast_mut();
                                }
                            }
                            *ptr2.add(24).cast::<usize>() = len7;
                            *ptr2.add(20).cast::<*mut u8>() = result7;
                            let vec9 = dependencies3;
                            let len9 = vec9.len();
                            let layout9 =
                                _rt::alloc::Layout::from_size_align_unchecked(vec9.len() * 8, 4);
                            let result9 = if layout9.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout9).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout9);
                                }
                                ptr
                            } else {
                                {
                                    ::core::ptr::null_mut()
                                }
                            };
                            for (i, e) in vec9.into_iter().enumerate() {
                                let base = result9.add(i * 8);
                                {
                                    let vec8 = (e.into_bytes()).into_boxed_slice();
                                    let ptr8 = vec8.as_ptr().cast::<u8>();
                                    let len8 = vec8.len();
                                    ::core::mem::forget(vec8);
                                    *base.add(4).cast::<usize>() = len8;
                                    *base.add(0).cast::<*mut u8>() = ptr8.cast_mut();
                                }
                            }
                            *ptr2.add(32).cast::<usize>() = len9;
                            *ptr2.add(28).cast::<*mut u8>() = result9;
                            let vec11 = local_dependencies3;
                            let len11 = vec11.len();
                            let layout11 =
                                _rt::alloc::Layout::from_size_align_unchecked(vec11.len() * 8, 4);
                            let result11 = if layout11.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout11).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout11);
                                }
                                ptr
                            } else {
                                {
                                    ::core::ptr::null_mut()
                                }
                            };
                            for (i, e) in vec11.into_iter().enumerate() {
                                let base = result11.add(i * 8);
                                {
                                    let vec10 = (e.into_bytes()).into_boxed_slice();
                                    let ptr10 = vec10.as_ptr().cast::<u8>();
                                    let len10 = vec10.len();
                                    ::core::mem::forget(vec10);
                                    *base.add(4).cast::<usize>() = len10;
                                    *base.add(0).cast::<*mut u8>() = ptr10.cast_mut();
                                }
                            }
                            *ptr2.add(40).cast::<usize>() = len11;
                            *ptr2.add(36).cast::<*mut u8>() = result11;
                        }
                        Err(_) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_get_project_info<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                            let l3 = *arg0.add(12).cast::<*mut u8>();
                            let l4 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l3, l4, 1);
                            let l7 = *arg0.add(20).cast::<*mut u8>();
                            let l8 = *arg0.add(24).cast::<usize>();
                            let base9 = l7;
                            let len9 = l8;
                            for i in 0..len9 {
                                let base = base9.add(i * 8);
                                {
                                    let l5 = *base.add(0).cast::<*mut u8>();
                                    let l6 = *base.add(4).cast::<usize>();
                                    _rt::cabi_dealloc(l5, l6, 1);
                                }
                            }
                            _rt::cabi_dealloc(base9, len9 * 8, 4);
                            let l12 = *arg0.add(28).cast::<*mut u8>();
                            let l13 = *arg0.add(32).cast::<usize>();
                            let base14 = l12;
                            let len14 = l13;
                            for i in 0..len14 {
                                let base = base14.add(i * 8);
                                {
                                    let l10 = *base.add(0).cast::<*mut u8>();
                                    let l11 = *base.add(4).cast::<usize>();
                                    _rt::cabi_dealloc(l10, l11, 1);
                                }
                            }
                            _rt::cabi_dealloc(base14, len14 * 8, 4);
                            let l17 = *arg0.add(36).cast::<*mut u8>();
                            let l18 = *arg0.add(40).cast::<usize>();
                            let base19 = l17;
                            let len19 = l18;
                            for i in 0..len19 {
                                let base = base19.add(i * 8);
                                {
                                    let l15 = *base.add(0).cast::<*mut u8>();
                                    let l16 = *base.add(4).cast::<usize>();
                                    _rt::cabi_dealloc(l15, l16, 1);
                                }
                            }
                            _rt::cabi_dealloc(base19, len19 * 8, 4);
                        }
                        _ => (),
                    }
                }
                pub trait Guest {
                    fn get_project_info(project_dir: _rt::String) -> Result<ProjectInfo, ()>;
                }
                #[doc(hidden)]

                macro_rules! __export_morphir_platform_project_0_1_0_cabi{
        ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

          #[export_name = "morphir:platform/project@0.1.0#get-project-info"]
          unsafe extern "C" fn export_get_project_info(arg0: *mut u8,arg1: usize,) -> *mut u8 {
            $($path_to_types)*::_export_get_project_info_cabi::<$ty>(arg0, arg1)
          }
          #[export_name = "cabi_post_morphir:platform/project@0.1.0#get-project-info"]
          unsafe extern "C" fn _post_return_get_project_info(arg0: *mut u8,) {
            $($path_to_types)*::__post_return_get_project_info::<$ty>(arg0)
          }
        };);
      }
                #[doc(hidden)]
                pub(crate) use __export_morphir_platform_project_0_1_0_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 44]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 44]);
            }

            #[allow(dead_code, clippy::all)]
            pub mod command_runner {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_run_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let base3 = arg0;
                    let len3 = arg1;
                    let mut result3 = _rt::Vec::with_capacity(len3);
                    for i in 0..len3 {
                        let base = base3.add(i * 8);
                        let e3 = {
                            let l0 = *base.add(0).cast::<*mut u8>();
                            let l1 = *base.add(4).cast::<usize>();
                            let len2 = l1;
                            let bytes2 = _rt::Vec::from_raw_parts(l0.cast(), len2, len2);

                            _rt::string_lift(bytes2)
                        };
                        result3.push(e3);
                    }
                    _rt::cabi_dealloc(base3, len3 * 8, 4);
                    let result4 = T::run(result3);
                    let ptr5 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result4 {
                        Ok(e) => {
                            *ptr5.add(0).cast::<u8>() = (0i32) as u8;
                            let vec6 = (e.into_bytes()).into_boxed_slice();
                            let ptr6 = vec6.as_ptr().cast::<u8>();
                            let len6 = vec6.len();
                            ::core::mem::forget(vec6);
                            *ptr5.add(8).cast::<usize>() = len6;
                            *ptr5.add(4).cast::<*mut u8>() = ptr6.cast_mut();
                        }
                        Err(_) => {
                            *ptr5.add(0).cast::<u8>() = (1i32) as u8;
                        }
                    };
                    ptr5
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_run<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                        _ => (),
                    }
                }
                pub trait Guest {
                    fn run(args: _rt::Vec<_rt::String>) -> Result<_rt::String, ()>;
                }
                #[doc(hidden)]

                macro_rules! __export_morphir_platform_command_runner_0_1_0_cabi{
      ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

        #[export_name = "morphir:platform/command-runner@0.1.0#run"]
        unsafe extern "C" fn export_run(arg0: *mut u8,arg1: usize,) -> *mut u8 {
          $($path_to_types)*::_export_run_cabi::<$ty>(arg0, arg1)
        }
        #[export_name = "cabi_post_morphir:platform/command-runner@0.1.0#run"]
        unsafe extern "C" fn _post_return_run(arg0: *mut u8,) {
          $($path_to_types)*::__post_return_run::<$ty>(arg0)
        }
      };);
    }
                #[doc(hidden)]
                pub(crate) use __export_morphir_platform_command_runner_0_1_0_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 12]);
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub use alloc_crate::alloc;
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }
    extern crate alloc as alloc_crate;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_main_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::morphir::platform::workspace::__export_morphir_platform_workspace_0_1_0_cabi!($ty with_types_in $($path_to_types_root)*::exports::morphir::platform::workspace);
  $($path_to_types_root)*::exports::morphir::platform::project::__export_morphir_platform_project_0_1_0_cabi!($ty with_types_in $($path_to_types_root)*::exports::morphir::platform::project);
  $($path_to_types_root)*::exports::morphir::platform::command_runner::__export_morphir_platform_command_runner_0_1_0_cabi!($ty with_types_in $($path_to_types_root)*::exports::morphir::platform::command_runner);
  )
}
#[doc(inline)]
pub(crate) use __export_main_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:main:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 447] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xc4\x02\x01A\x02\x01\
A\x06\x01B\0\x04\x01\x20morphir:platform/workspace@0.1.0\x05\0\x01B\x06\x01ps\x01\
r\x05\x04names\x10source-directorys\x08includes\0\x0cdependencies\0\x12local-dep\
endencies\0\x04\0\x0cproject-info\x03\0\x01\x01j\x01\x02\0\x01@\x01\x0bproject-d\
irs\0\x03\x04\0\x10get-project-info\x01\x04\x04\x01\x1emorphir:platform/project@\
0.1.0\x05\x01\x01B\x04\x01ps\x01j\x01s\0\x01@\x01\x04args\0\0\x01\x04\0\x03run\x01\
\x02\x04\x01%morphir:platform/command-runner@0.1.0\x05\x02\x04\x01\x1bmorphir:pl\
atform/main@0.1.0\x04\0\x0b\x0a\x01\0\x04main\x03\0\0\0G\x09producers\x01\x0cpro\
cessed-by\x02\x0dwit-component\x070.208.1\x10wit-bindgen-rust\x060.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
