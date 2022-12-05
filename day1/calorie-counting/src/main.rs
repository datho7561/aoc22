use std::fs;
use std::str::FromStr;

fn main() {
    let input_text: String = fs::read_to_string("input.txt").unwrap();
    part2(input_text);
}

fn part2(input_text: String) {
    let mut three_sorted = ThreeSorted {
        top: 0,
        mid: 0,
        bot: 0,
        _i: 0,
    };
    for elfs_inventory in input_text.splitn(1000, "\n\n") {
        let mut elfs_total_cals: i32 = 0;
        for item in elfs_inventory.splitn(1000, "\n") {
            if item.len() != 0 {
                let food_cals = i32::from_str(item);
                match food_cals {
                    Ok(v) => {
                        elfs_total_cals += v;
                    }
                    Err(e) => {
                        println!("Error while parsing integer: {}", e);
                    }
                };
            }
        }
        three_sorted.put(elfs_total_cals);
    }
    let mut sum_top_three: i32 = 0;
    for item in three_sorted {
        sum_top_three += item;
    }
    println!("{}", sum_top_three);
}

struct ThreeSorted {
    top: i32,
    mid: i32,
    bot: i32,
    _i: i32,
}

trait Container {
    type Item;
    fn put(&mut self, item: Self::Item);
}

impl Container for ThreeSorted {
    type Item = i32;

    fn put(&mut self, item: i32) {
        if item < self.bot {
            return;
        }
        self.bot = item;
        if item > self.mid {
            self.bot = self.mid;
            self.mid = item;
        }
        if item > self.top {
            self.mid = self.top;
            self.top = item;
        }
        return;
    }
}

impl Iterator for ThreeSorted {
    fn next(&mut self) -> Option<Self::Item> {
        match self._i {
            0 => {
                self._i += 1;
                Some(self.top)
            },
            1 => {
                self._i += 1;
                Some(self.mid)
            },
            2 => {
                self._i += 1;
                Some(self.bot)
            }
            _ => {
                None
            }
        }
    }

    type Item = i32;
}

fn part1(input_text: String) {
    let mut max: i32 = 0;
    for elfs_inventory in input_text.splitn(1000, "\n\n") {
        let mut elfs_total_cals: i32 = 0;
        for item in elfs_inventory.splitn(1000, "\n") {
            if item.len() != 0 {
                let food_cals = i32::from_str(item);
                match food_cals {
                    Ok(v) => {
                        elfs_total_cals += v;
                    }
                    Err(e) => {
                        println!("Error while parsing integer: {}", e);
                    }
                };
            }
        }
        if elfs_total_cals > max {
            max = elfs_total_cals;
        }
    }
    println!("{}", max);
}
