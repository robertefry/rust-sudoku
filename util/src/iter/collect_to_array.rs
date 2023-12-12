
pub trait CollectToArray: Iterator
{
    fn collect_to_array<const N: usize>(self) -> Option<[Self::Item;N]>
    where
        Self: Sized
    {
        use std::mem::MaybeUninit;

        let mut array: [MaybeUninit<Self::Item>;N];
        let mut count = 0;

        array = unsafe{ MaybeUninit::uninit().assume_init() };

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

impl<I: Iterator> CollectToArray for I {}
