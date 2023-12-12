
#[cfg(test)]
mod test
{

    #[test]
    fn test_dataset()
    {
        let dataset = sudoku::dataset::random(1_000);

        for (sudoku,expect) in dataset.into_iter()
        {
            let solved = sudoku::solve(&sudoku);
            assert_eq!(solved, Some(expect));
        }
    }

}
