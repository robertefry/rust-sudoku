
pub trait CollectToArray<T, const N: usize>
{
    fn collect_to_array(self) -> Option<[T;N]>;
}

impl<T, const N: usize> CollectToArray<T::Item,N> for T
where
    T: Iterator,
{
    fn collect_to_array(self) -> Option<[T::Item;N]>
    {
        use std::mem::MaybeUninit;

        let mut array: [MaybeUninit<T::Item>;N] = unsafe{ MaybeUninit::uninit().assume_init() };
        let mut count = 0;

        for elem in self.into_iter()
        {
            if count >= N
            {
                return None;
            }

            unsafe
            {
                std::ptr::write(array[count].as_mut_ptr(), elem);
            }
            count += 1;
        }

        if count != N
        {
            return None;
        }

        return Some(unsafe{ std::mem::transmute_copy(&array) });
    }
}
