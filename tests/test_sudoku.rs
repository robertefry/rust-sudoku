
#[cfg(test)]
mod test
{
    use rand::seq::index::sample;
    use rand::thread_rng;

    #[test]
    fn test_dataset()
    {
        let dataset = sudoku::load_dataset();

        let mut rng = thread_rng();
        let sample = sample(&mut rng, dataset.len(), 1_000);

        for (sudoku,expect) in sample.iter().map(|index| dataset[index])
        {
            let solved = sudoku::solve(&sudoku);
            assert_eq!(solved, Some(expect));
        }
    }

}
