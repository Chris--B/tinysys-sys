// TODO: It would be great to support this without an allocator.
#[macro_export]
macro_rules! dbg {
    () => {
        {
            let loc: &str = concat!("[", file!(), ":", line!(), ":", column!(), "]\n");

            // This function is unsafe, but this warning fires on nested unsafe blocks.
            #[allow(unused_unsafe)]
            unsafe {
                UARTSendBlock(loc.as_bytes().as_ptr() as *mut u8, loc.len() as u32);
            }
        }
    };

    ($val:expr $(,)?) => {
        {
            let loc: &str = concat!("[", file!(), ":", line!(), ":", column!(), "]");

            let val = $val;
            let val_str: &str = stringify!($val);
            let formatted: ::alloc::string::String = ::alloc::format!("{loc} {val_str} = {val:#?}\n",);

            // This function is unsafe, but this warning fires on nested unsafe blocks.
            #[allow(unused_unsafe)]
            unsafe {
                UARTSendBlock(formatted.as_bytes().as_ptr() as *mut u8, formatted.len() as u32);
            }

            val
        }
    };

    ($($val:expr),+ $(,)?) => {
        {
            let loc: &str = concat!("[", file!(), ":", line!(), ":", column!(), "]");
            let mut formatted: ::alloc::string::String = ::alloc::format!("{loc} ",);
            $(
                {
                    let val = $val;
                    let val_str: &str = stringify!($val);

                    let next: ::alloc::string::String = ::alloc::format!("{val_str} = {val:#?}, ",);
                    formatted.push_str(&next);
                }
            )+
            formatted.push_str("\n");

            // This function is unsafe, but this warning fires on nested unsafe blocks.
            #[allow(unused_unsafe)]
            unsafe {
                UARTSendBlock(formatted.as_bytes().as_ptr() as *mut u8, formatted.len() as u32);
            }

            // "Return" the expression list as a tuple
            (
                $(
                    $val,
                )+
            )
        }
    };
}
