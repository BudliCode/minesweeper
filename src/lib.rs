use std::cmp::min;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut new_minefield: Vec<String> = Vec::from(minefield)
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    for y in 0..minefield.len() {
        for (x, _) in minefield[y].match_indices(' ') {
            let bombs = calc_bomb_count(minefield, y, x);
            if bombs > 0 {
                new_minefield[y].replace_range(x..x + 1, &bombs.to_string())
            }
        }
    }

    new_minefield
}

fn calc_bomb_count(minefield: &[&str], y: usize, x: usize) -> usize {
    let y_bound = minefield.len() - 1;
    let x_bound = minefield[0].len() - 1;

    let y_lo = match y {
        0 => 0,
        _ => y - 1,
    };
    let y_hi = min(y_bound, y + 1);

    let x_lo = match x {
        0 => 0,
        _ => x - 1,
    };
    let x_hi = min(x_bound, x + 1);

    let mut bomb_count = 0;
    for row in minefield[y_lo..=y_hi].iter() {
        for _ in row[x_lo..=x_hi].matches('*') {
            bomb_count += 1;
        }
    }

    bomb_count
}

#[test]
fn bottom_missing_calc_bomb() {
    #[rustfmt::skip]
    let minefield = &[
        "***",
        "* *",
        "* *",
    ];

    let actual = calc_bomb_count(minefield, 1, 1);
    let expected = 7;
    assert_eq!(actual, expected);
}

#[test]
fn top_missin_calc_bomb() {
    #[rustfmt::skip]
    let minefield = &[
        "* *",
        "* *",
        "***",
    ];

    let actual = calc_bomb_count(minefield, 1, 1);
    let expected = 7;
    assert_eq!(actual, expected);
}

#[test]
fn left_missing_calc_bomb() {
    #[rustfmt::skip]
    let minefield = &[
        "***",
        "  *",
        "***",
    ];

    let actual = calc_bomb_count(minefield, 1, 1);
    let expected = 7;
    assert_eq!(actual, expected);
}

#[test]
fn right_missing_calc_bomb() {
    #[rustfmt::skip]
    let minefield = &[
        "***",
        "*  ",
        "***",
    ];

    let actual = calc_bomb_count(minefield, 1, 1);
    let expected = 7;
    assert_eq!(actual, expected);
}
