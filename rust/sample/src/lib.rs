
use com::interfaces::{IUnknown};
use core::ffi::c_void;
use std::ptr;

com::interfaces! {
    #[uuid("EFF8970E-C50F-45E0-9284-291CE5A6F771")]
    pub unsafe interface IAnimal: IUnknown {
        fn Eat(&self) -> u32;
    }
}

com::class! {
    pub class Monkey : IAnimal {}

    impl IAnimal for Monkey {
        fn Eat(&self) -> u32 {
            return 57;
        }
    }
}

#[no_mangle]
pub extern fn create_monkey() -> *mut c_void {
    unsafe {
        let instance = Monkey::allocate();
        let interface_handle = instance.query_interface::<IAnimal>().unwrap();
        interface_handle.AddRef();
        let mut pv_object: *mut c_void = ptr::null_mut();
        interface_handle.QueryInterface(&IID_IANIMAL, &mut pv_object);
        return pv_object;
    }
}
