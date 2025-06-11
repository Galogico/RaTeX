use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = RaTeX::get_file_path(&args);
    RaTeX::run(config.expect("couldnt get file path"));
    
}