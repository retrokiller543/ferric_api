#[macro_export]
macro_rules! mod_def {
    {$($vis:vis mod $ident:ident $(;)?)+} => {
        $($vis mod $ident;
        #[allow(unused_imports)]
        $vis use $ident::*;)+
    };
}
