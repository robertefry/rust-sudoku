
use std::sync::*;

pub fn initialise()
{
    Groups::initialise();
}

pub struct Groups
{
    group_xyz: [[usize;20];81],
}

static GROUPS: OnceLock<Groups> = OnceLock::new();

impl Groups
{
    fn initialise()
    {
        Self::get_or_init();
    }

    pub fn get_group_xyz<'a>(index: usize) -> &'a [usize;20]
    {
        return &Self::get_or_init().group_xyz[index];
    }

    fn get_or_init<'g>() -> &'g Self
    {
        GROUPS.get_or_init(||
        {
            use util::iter::Unique;
            use util::iter::CollectToArray;

            macro_rules! gen_group
            {
                ( $($f:expr),* ) =>
                {{
                    (0..81)
                        .map(|k| std::iter::empty()
                            $( .chain((0..81).filter(|&x| k != x && $f(k) == $f(x))) )*
                            .unique()
                            .collect_to_array()
                            .unwrap())
                        .collect_to_array()
                        .unwrap()
                }};
            }

            let get_x = |k: usize| k % 9;
            let get_y = |k: usize| k / 9;
            let get_z = |k: usize| (get_x(k) / 3) * 3 + (get_y(k) / 3) * 27;

            return Groups
            {
                group_xyz: gen_group!(get_x,get_y,get_z),
            };
        })
    }
}
