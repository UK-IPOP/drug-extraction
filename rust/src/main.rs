mod utils;

fn main() {
    let file_data = utils::load_data();
    let user_input = utils::get_user_input();
    match user_input.as_str() {
        "J" => utils::jarowinkler_runner(file_data),
        "L" => utils::levenshtein_runner(file_data),
        _ => println!("Not acceptable value provided, please use `J` or `L`."),
    }
}
