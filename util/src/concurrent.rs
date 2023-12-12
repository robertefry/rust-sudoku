
#[macro_export]
macro_rules! concurrent
{
    ( $expr:expr ) =>
    {
        std::thread::scope(move |_| $expr)
    }
}
