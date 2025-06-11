use std::fs;

mod arts;
pub fn run(file_path: String){
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("{content}");
    search(&content);
}

pub fn search<'a>(contents: &'a str){
    let mut final_text: String = String::new();
    for line in contents.lines(){
        if line.contains("/anchor"){
            final_text.push_str(&arts::arts::ANCHOR.to_string().as_str());
        }else if line.contains("/porg"){
          final_text.push_str(&arts::arts::PORG.to_string().as_str());
        }else if line.contains("/ban"){
          final_text.push_str(&arts::arts::BAN.to_string().as_str());
        }else if line.contains("/ratex"){
          final_text.push_str(&arts::arts::RATEX.to_string().as_str());
        }else if line.contains("/computer"){
          final_text.push_str(&arts::arts::COMPUTER.to_string().as_str());
        }
        
        else{
            final_text.push_str(format!("{line}\n").as_str());
        }
        fs::File::create("./output.txt").unwrap();
        fs::write("./output.txt", &final_text);
    }
}


pub fn get_file_path(args: &[String]) -> Result<String, &'static str> {
    if args.len() < 2{
        return Err("not enough arguments")
    }
    let file_path = args[1].clone();

    Ok(file_path)
}

