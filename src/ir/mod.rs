pub mod compile;

macro_rules! IrType {
    ($name:tt, $trait:ident, $( $param:ident ),*) => {
        struct $name<$( $param ),*> {
            _phantom: std::marker::PhantomData<($( $param ),*)>,
        }

        impl<$( $param ),*> $name<$( $param ),*> {
            #[allow(dead_code)]
            pub fn new() -> Self {
                Self {
                    _phantom: std::marker::PhantomData,
                }
            }
        }

        #[allow(dead_code)]
        trait $trait {}
        impl<$( $param ),*> $trait for $name<$( $param ),*> {}  
    };
}

IrType!(Add, AddTrait, T, U);