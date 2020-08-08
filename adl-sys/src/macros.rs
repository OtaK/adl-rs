macro_rules! adlfn {
    (
        $(#[$meta:meta])*
        pub unsafe fn $fn:ident(&self, $($arg:ident: $arg_type:ty),*) -> $ret:ty;
    ) => {
        $(#[$meta])*
        pub unsafe fn $fn(&self, $($arg: $arg_type), *) -> $ret {
            static ADDR_CACHE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

        }
    };
}
