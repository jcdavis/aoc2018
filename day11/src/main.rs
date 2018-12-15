fn main() {
    println!("{:?}", part1(3613));
    println!("{:?}", part2(3613));
}

fn part1(sn: i32) -> (i32, i32) {
    let dim = 3;
    let mut max = -50000;
    let mut max_c = (0,0);
    for x in 1..(301-dim) {
        for y in 1..(301-dim) {
            let mut sum = 0;
            for dx in 0..dim {
                for dy in 0..dim {
                    sum += power(x+dx, y+dy, sn);
                }
            }
            if sum > max {
                max = sum;
                max_c = (x, y);
            }
        }
    }
    max_c
}

fn part2(sn: i32) -> (i32, i32, i32) {
    let mut powers = [[0; 301]; 301];

    for x in 1..301 {
        for y in 1..301 {
            powers[x][y] = power(x as i32, y as i32, sn);
        }
    }

    let mut prefixes = [[0; 301]; 301];

    for x in 1..301 {
        for y in 1..301 {
            let sum_left = prefixes[x-1][y];
            let sum_up = prefixes[x][y-1];
            prefixes[x][y] = sum_left + sum_up - prefixes[x-1][y-1] + powers[x][y];
        }
    }

    let mut max = -50000;
    let mut max_c = (0,0, 0);
    for x in 1..301 {
        for y in 1..301 {
            let mut dim = 1;

            while x + dim <= 301 && y + dim <= 301 {
                let sum = prefixes[x + dim - 1][y + dim-1] -
                    prefixes[x-1][y + dim-1] - prefixes[x + dim -1][y-1] + prefixes[x-1][y-1];
                if sum > max {
                    max = sum;
                    max_c = (x as i32, y as i32, dim as i32);
                }

                dim += 1;
            }
        }
    }
    max_c
}

fn power(x: i32, y: i32, sn: i32) -> i32 {
    let rackid = x+10;
    let full = (rackid * y + sn) * rackid;
    (full / 100) %10 -5
}
