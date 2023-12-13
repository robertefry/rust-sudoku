
use itertools::*;

pub fn first(count: usize) -> Vec<([u8;81],[u8;81])>
{
    use util::iter::CollectToArray;

    csv::Reader::from_path("dataset.csv")
        .unwrap()
        .records()
        .take(count)
        .filter_map(|r| r.ok())
        .filter_map(|r| r.iter()
            .filter_map(|string| string.chars()
                .map(|c| c.to_digit(10).map(|d| d as u8).unwrap())
                .collect_to_array())
            .next_tuple())
        .collect()
}

pub fn load() -> Vec<([u8;81],[u8;81])>
{
    use util::iter::CollectToArray;

    csv::Reader::from_path("dataset.csv")
        .unwrap()
        .records()
        .filter_map(|r| r.ok())
        .filter_map(|r| r.iter()
            .filter_map(|string| string.chars()
                .map(|c| c.to_digit(10).map(|d| d as u8).unwrap())
                .collect_to_array())
            .next_tuple())
        .collect()
}

pub fn random(count: usize) -> Vec<([u8;81],[u8;81])>
{
    use rand::seq::index::sample;
    use rand::thread_rng;

    let dataset = crate::dataset::load();

    let mut rng = thread_rng();
    let sample = sample(&mut rng, dataset.len(), count);

    return sample.into_iter()
        .map(|index| dataset[index])
        .collect();
}
