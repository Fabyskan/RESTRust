use std::process;
pub struct Input {
    mainarg: String,
    args: Vec<String>,
}
impl Input {
    pub fn new(args: &[String]) -> Input {
        if args.len() < 2 {
            eprintln!("No Arguments given!");
            process::exit(1);
        }
        let input = Input {
            mainarg: args[1].clone(),
            args: args[2..].to_vec(),
        };
        println!("{}", input.mainarg);
        println!("{:?}", input.args);
        input
    }
}
