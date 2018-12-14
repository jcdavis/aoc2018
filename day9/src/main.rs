fn main() {
    println!("{}", score(404, 71852));
    println!("{}", score(404, 7185200));
}

//Double linked list using array indices because rust
struct Position {
    marble: i32,
    prev: usize,
    next: usize,
}

fn score(players: usize, last: i32) -> i64 {
    let mut circle: Vec<Position> = Vec::new();
    circle.push(Position {marble: 0, prev: 0, next: 0});
    let mut current_i = 0;
    let mut scores: [i64; 1000] = [0; 1000];
    let mut current_p = 0;

    for marble in 1..(last+1) {
        if marble % 23 != 0 {
            let b_i = circle[current_i].next;
            let a_i = circle[b_i].next;
            let new_i = circle.len();
            circle[a_i].prev = new_i;
            circle[b_i].next = new_i;
            circle.push(Position {marble: marble, prev: b_i, next: a_i});
            current_i = new_i;
        } else {
            scores[current_p] += marble as i64;
            let mut removed_i = current_i;
            for _ in 0..7 {
                removed_i = circle[removed_i].prev;
            }
            scores[current_p] += circle[removed_i].marble as i64;
            let p_i = circle[removed_i].prev;
            let n_i = circle[removed_i].next;
            circle[p_i].next = n_i;
            circle[n_i].prev = p_i;
            current_i = n_i;
        }
        current_p = (current_p + 1) % players;
    }
    return *scores.iter().max().unwrap();
}
