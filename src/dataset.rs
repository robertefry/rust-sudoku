
use itertools::*;

pub fn load_dataset() -> Vec<([u8;81],[u8;81])>
{
    csv::Reader::from_path("dataset.csv")
        .unwrap()
        .records()
        .filter_map(|r| r.ok())
        .filter_map(|r| r.iter()
            .filter_map(|string| string.chars()
                .map(|c| c.to_digit(10).map(|d| d as u8))
                .flatten()
                .collect::<Vec<u8>>()
                .try_into()
                .ok())
            .next_tuple())
        .collect()
}
