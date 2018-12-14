fn main() {
    println!("{}", score(404, 71852));
    println!("{}", score(404, 7185200));
}

fn score(players: usize, last: i32) -> i32 {
    let mut circle = vec![0, 1];
    let mut current_i = 1;
    let mut scores = [0; 1000];
    let mut current_p = 0;

    for marble in 2..(last+1) {
        if marble % 23 != 0 {
            let idx = (current_i + 2) % circle.len();
            circle.insert(idx, marble);
            current_i = idx;
        } else {
            scores[current_p] += marble;
            let removed_i = (current_i + circle.len() - 7) % circle.len();
            let removed = circle.remove(removed_i);
            scores[current_p] += removed;
            current_i = removed_i % circle.len();
            //println!("{} got {} + {}", current_p, marble, removed);
        }
        //println!("{:?}", circle);
        current_p = (current_p + 1) % players;
    }
    return *scores.iter().max().unwrap();
}
