
mod board; use board::*;
mod groups; use groups::*;

pub fn initialise()
{
    Groups::initialise();
}

pub fn solve(cells: &[u8;81]) -> Option<[u8;81]>
{
    let mut board = Board::from(*cells);

    // 1. Initialise the indexing system.

    let mut index = Some(0);

    let next_index = |mut idx: usize, backtrack: bool| -> Option<usize>
    {
        loop
        {
            if backtrack && idx == 0 { return None; }
            idx = if backtrack { idx - 1 } else { idx + 1 };

            if idx >= 81 || cells[idx] == 0
            {
                break;
            }
        }

        return Some(idx);
    };

    if cells[0] != 0
    {
        index = next_index(0, false);
    }

    // 2. While there exists an unsolved cell, either find a solution or backtrack.

    while let Some(idx) = index
    {
        if idx >= 81
        {
            break; // found a solution
        }

        let next = board.next_solution(idx);

        board.mark_solution(idx, next.unwrap_or(0));
        index = next_index(idx, next == None); // backtrack if no solution
    }

    // 3. If we backtracked past the origin, no solution exists.
    //  Otherwise, return the solved board.

    if index == None
    {
        return None;
    }

    return Some(*board.cells());
}
