
use com::interfaces::{IUnknown,IClassFactory};

com::interfaces! {
    #[uuid("b05271fc-4a1d-45ae-8f7a-c5f2dc0cde88")]
    pub unsafe interface IBike : IUnknown {
        pub fn ride(&self);
    }

    #[uuid("2015c764-184c-4f50-86cf-a00e831658e1")]
    pub unsafe interface ICar : IUnknown {
        pub fn drive(&self);
    }
}

com::class! {
    pub class HybridVehicle : IBike, ICar {}

    impl IBike for HybridVehicle {
        fn ride(&self) {}
    }

    impl ICar for HybridVehicle {
        fn drive(&self) {}
    }
}

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
pub extern fn hello_com() -> *const u8 {
    "Hello, world!\0".as_ptr()
}
