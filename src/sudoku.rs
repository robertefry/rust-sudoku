
pub fn solve(cells: &[u8;81]) -> Option<[u8;81]>
{
    let mut index = Some(0);
    let mut board = Board::from(cells);

    let get_next_index = |mut idx: usize, backtrack: bool| -> Option<usize>
    {
        loop
        {
            if backtrack && idx == 0 { return None; }
            idx = if backtrack { idx - 1 } else { idx + 1 };

            if idx >= cells.len() || cells[idx] == 0
            {
                break;
            }
        }

        return Some(idx);
    };

    // 1. Find the optional index of the first unknown cell.

    if cells[0] != 0
    {
        index = get_next_index(0, false);
    }

    // 2. while there exists an unsolved cell, either find a solution or
    //  backtrack.

    while let Some(idx) = index
    {
        if idx >= cells.len()
        {
            break; // found a solution
        }

        let next = board.get_next_solution(idx);

        board[idx] = next.unwrap_or(0);
        index = get_next_index(idx, next == None); // backtrack if no solution
    }

    // 3. If we backtracked past the origin, no solution exists.

    if index == None
    {
        return None;
    }

    // 4. Return the solution.

    return Some(board.cells);
}

use std::ops::*;

struct Board
{
    cells: [u8;81]
}

impl Index<usize> for Board
{
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output
    {
        return &self.cells[index];
    }
}
impl IndexMut<usize> for Board
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {
        return &mut self.cells[index];
    }
}

impl Board
{
    fn from(cells: &[u8;81]) -> Self
    {
        return Board{ cells: *cells };
    }

    fn get_next_solution(&self, index: usize) -> Option<u8>
    {
        for value in self.cells[index]+1..=9
        {
            if ! self.is_valid_solution(index, value)
            {
                continue;
            }

            return Some(value);
        }

        return None;
    }

    fn is_valid_solution(&self, index: usize, value: u8) -> bool
    {
        for k in GroupIterator::over_x(index)
        {
            if self.cells[k] == value { return false; }
        }

        for k in GroupIterator::over_y(index)
        {
            if self.cells[k] == value { return false; }
        }

        for k in GroupIterator::over_z(index)
        {
            if self.cells[k] == value { return false; }
        }

        return true;
    }
}

struct GroupIterator
{
    index: usize,
    count: usize,
    fn_next: fn(usize)->usize,
}

impl Iterator for GroupIterator
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.count >= 9
        {
            return None;
        }

        let current = self.index;
        self.index = (self.fn_next)(self.index);
        self.count += 1;

        return Some(current);
    }
}

impl GroupIterator
{
    fn over_x(k: usize) -> Self
    {
        return GroupIterator
        {
            index: (k/9) * 9,
            count: 0,
            fn_next: |k| k + 1,
        };
    }

    fn over_y(k: usize) -> Self
    {
        return GroupIterator
        {
            index: k % 9,
            count: 0,
            fn_next: |k| k + 9,
        };
    }

    fn over_z(k: usize) -> Self
    {
        return GroupIterator
        {
            index: ((k%9) / 3) * 3 + ((k/9) / 3) * 27,
            count: 0,
            fn_next: |k| k + if k % 3 == 2 { 7 } else { 1 },
        };
    }
}
