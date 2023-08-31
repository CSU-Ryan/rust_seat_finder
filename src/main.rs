struct Coordinate {
    row: usize,
    col: usize
}

fn main() {
    let seats_per_row = input_to_usize("Please enter the amount of seats per row: ");

    let seat_num = input_to_usize("Please enter your seat number: ")
        - 1; // if you're in the 1st seat, index should be 0;

    let coord = seat_finder(seats_per_row, seat_num);

    println!("row: {0}\tseat: {1}", (&coord.row + 1), (&coord.col + 1));
}

fn input_to_usize(message: &str) -> usize {
    let mut input = String::new();

    println!("{}", message);
    std::io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse::<usize>()
        .expect("Please type a number")
}

fn seat_finder(seats_per_row: usize, seat_num: usize) -> Coordinate {
    let row = seat_num / seats_per_row;
    let col = seat_num % seats_per_row;

    Coordinate { row, col }
}
