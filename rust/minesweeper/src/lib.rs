pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }
    let rows = minefield.len();
    let cols = minefield[0].len();
    let mut final_vec = vec![vec![b'0'; cols]; rows];

    for (i, row) in minefield.iter().enumerate() {
        for (j, &col) in row.as_bytes().iter().enumerate() {
            if col == b'*' {
                final_vec[i][j] = b'*';
            } else {
                let mine_counter = (-1..=1)
                    .flat_map(|di| (-1..=1).map(move |dj| (di, dj))) // generate all possible combinations of ((-1..=1), (-1..=1))
                    // filter to return only Some values and unwrap them
                    .filter_map(|(di, dj)| {
                        print!("{:?}", (di, dj));
                        let new_row = (i as isize + di) as usize; // if i is 0 and di is -1, then new_row is usize::MAX - 1
                        let new_col = (j as isize + dj) as usize;
                        if new_row < rows // usize::MAX - 1 is for sure bigger then rows
                            && new_col < cols
                            && minefield[new_row].as_bytes()[new_col] == b'*'
                        {
                            Some(1)
                        } else {
                            None
                        }
                    })
                    .count(); // count all unwraped values
                final_vec[i][j] = match mine_counter as u8 {
                    0 => b' ',
                    n => b'0' + n,
                }
            }
        }
    }

    final_vec
        .iter()
        .map(|vec| std::str::from_utf8(vec).unwrap().to_string())
        .collect::<Vec<String>>()
}
