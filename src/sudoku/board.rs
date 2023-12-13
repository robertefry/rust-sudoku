
use super::*;

pub struct Board
{
    cells: [u8;81],
}

impl Board
{
    pub fn from(cells: [u8;81]) -> Self
    {
        return Board{ cells };
    }

    pub fn next_solution(&self, index: usize) -> Option<u8>
    {
        for value in self.cells[index]+1..=9
        {
            if self.is_invalid_solution(index, value)
            {
                continue;
            }

            return Some(value);
        }

        return None;
    }

    fn is_invalid_solution(&self, index: usize, value: u8) -> bool
    {
        Groups::at(index).any(|k| self.cells[k] == value)
    }

    pub fn mark_solution(&mut self, index: usize, value: u8)
    {
        self.cells[index] = value;
    }

    pub fn cells(&self) -> &[u8;81]
    {
        return &self.cells;
    }
}
