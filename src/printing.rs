// TODO: It would be great to support this without an allocator.
#[macro_export]
macro_rules! dbg {
    () => {
        {
            let loc: &str = concat!("[", file!(), ":", line!(), ":", column!(), "] \0");

            // Note: UARTPrintf() expects a C String, which means "\0" terminated
            // This function is unsafe, but this warning fires on nested unsafe blocks.
            #[allow(unused_unsafe)]
            unsafe {
                UARTPrintf(c"%s\n".as_ptr(), loc.as_bytes().as_ptr());
            }
        }
    };

    ($val:expr $(,)?) => {
        {
            let loc: &str = concat!("[", file!(), ":", line!(), ":", column!(), "]");

            let val = $val;
            let val_str: &str = stringify!($val);
            let formatted: ::alloc::string::String = ::alloc::format!("{loc} {val_str} = {val:#?}\0",);
            let formatted: ::alloc::vec::Vec<u8> = formatted.into();

            // Note: UARTPrintf() expects a C String, which means "\0" terminated
            // This function is unsafe, but this warning fires on nested unsafe blocks.
            #[allow(unused_unsafe)]
            unsafe {
                UARTPrintf(c"%s\n".as_ptr(), formatted.as_ptr());
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
            formatted.push_str("\n\0");

            // Note: UARTPrintf() expects a C String, which means "\0" terminated
            // This function is unsafe, but this warning fires on nested unsafe blocks.
            #[allow(unused_unsafe)]
            unsafe {
                UARTPrintf(c"%s\n".as_ptr(), formatted.as_ptr());
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
