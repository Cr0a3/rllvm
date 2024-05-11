/// Executur for the inner function
pub struct ExecEngine<T> {
    pub inner: T,
}

macro_rules! impl_unsafe_fn {
    (@recurse $first:ident $( , $rest:ident )*) => {
        impl_unsafe_fn!($( $rest ),*);
    };

    (@recurse) => {};

    ($( $param:ident ),*) => {
        impl<Output, $( $param ),*> ExecEngine<unsafe extern "C" fn($( $param ),*) -> Output> {
            /// Calls the function
            #[allow(non_snake_case)]
            #[inline(always)]
            pub unsafe fn call(&self, $( $param: $param ),*) -> Output {
                (self.inner)($( $param ),*)
            }
        }

        impl_unsafe_fn!(@recurse $( $param ),*);
    };
}

impl_unsafe_fn!(
    A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20, A21,
    A22, A23, A24, A25, A26, A27, A28, A29, A30, A31, A32, A33, A34, A35, A36, A37, A38, A39, A40,
    A41, A42, A43, A44, A45, A46, A47, A48, A49, A50
);
