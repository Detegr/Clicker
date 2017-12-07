macro_rules! wasm_export {
    ($fn_name:ident($($arg:ident:$type:ty),*) $code:block) => {
        #[no_mangle] pub extern "C" fn $fn_name($($arg:$type),*) {
            $code
        }
    }
}

macro_rules! wasm_export_unsafe {
    ($fn_name:ident($($arg:ident:$type:ty),*) $code:block) => {
        #[no_mangle] pub unsafe extern "C" fn $fn_name($($arg:$type),*) {
            $code
        }
    }
}
