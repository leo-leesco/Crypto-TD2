fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 3);
    let rs: String;
    let path_to_file: String;
    unsafe {
        rs = args.get_unchecked(1).to_string();
        path_to_file = args.get_unchecked(2).to_string();
    }
    println!("rs={rs} ; path_to_file={path_to_file}");
}
