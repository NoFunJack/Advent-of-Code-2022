#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    return *collect_cals(input).iter().max().unwrap();
}

#[aoc(day1, part2)]
fn part2(input: &str) -> i32 {
    let mut all_cal: Vec<i32> = collect_cals(input);

    all_cal.sort_by(|a, b| b.cmp(a));

    return all_cal[0] + all_cal[1] + all_cal[2];
}

fn collect_cals(input: &str) -> Vec<i32> {
    return input
        .split("\n\n")
        .map(|elf| elf.lines().map(|l| l.parse::<i32>().unwrap()).sum())
        .collect();
}
