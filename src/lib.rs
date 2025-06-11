use std::fs;

pub fn run(file_path: String){
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("{content}");
    search(&content);
}

pub fn search<'a>(contents: &'a str){
    let mut final_text: String = String::new();
    for line in contents.lines(){
        if line.contains("/anchor"){
            final_text.push_str(r"                                
                        ####                      
                    ############                  
                    ############                  
                    ####    ####                  
                    ############                  
                    ############                  
                      ######                      
                        ####                      
                        ####                      
                    ############                  
        ##          ############          ##      
      ######            ####            ######    
    ##########          ####          ##########  
    ##########          ####        ############  
      ######            ####          ########    
        ######          ####          ######      
          ####          ####        ######        
          ########      ####      ########        
            ########    ####    ########          
              ########################            
                  ################                
");
        }else{
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

