fn print_list(list: &Vec<u32>) {
    for number in list {
        print!("{}\t", number);
    }
	
    println!("");
}

fn main() {
    let item = vec![1, 2, 3];

    print_list(&item);

    println!("Hello, world!");
}
