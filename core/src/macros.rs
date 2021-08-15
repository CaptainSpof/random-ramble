#[macro_export]
macro_rules! fail {
    ($($arg:tt)*) => {
        $crate::Jabber::Custom(format!("{}", format_args!($($arg)*)))
    };
}

#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        return Err($crate::Jabber::Custom(format!("{}", format_args!($($arg)*))))
    };
}
