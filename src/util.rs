// TODO: Remove if/when Option::expect becomes const
macro_rules! const_expect {
    ($t:expr, $m:expr) => {
        match $t {
            Some(v) => v,
            None => panic!($m),
        }
    };
}

pub(crate) use const_expect;
