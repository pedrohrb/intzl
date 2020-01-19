use std::io::Write;
use std::fs;
use std::env;
use std::path::Path;
use clap::{Arg,App};

fn create_folder(foldername: String) -> std::io::Result<()>{
	fs::create_dir(foldername)?;
	Ok(())
}

fn all_file(filename: String , ext: String) -> std::io::Result<()>{

	let file = filename.to_string() + "." + &ext.to_string();

	let mut cfile = fs::File::create(file.to_string())?;


	cfile.write_all(b"from flask import Flask \n\napp = Flask(__name__) \n\n@app.route(\"/\") \ndef index():\n    return \"Fun!\" \n)\n\n\n app.run(debug=True)");
	cfile.sync_data()?;

	Ok(())
}


fn enter_folder(name: String){
	let name_folder = name.to_string();
	let root = Path::new(&name_folder);
	assert!(env::set_current_dir(&root).is_ok());
}

fn main() {
    let app = App::new("intzler")
    				  .version("0.1")
    				  .author("Pedro Henrique")
    				  .about("Its a flask-initializer")
    				  .arg(
    				  	Arg::with_name("name")
    				  	.long("--name")
    				  	.takes_value(true)
    				  )
    				  .get_matches();


    if let Some(matches) = app.value_of("name") {
    	let result: String = matches.to_string();
    	create_folder(result.to_string());
    	println!("Creating folders");
    	let to_enter = "./".to_string() + &result.to_string();  
    	enter_folder(to_enter.to_string());
    	all_file("app".to_string(),"py".to_string());
    	println!("Creating program");
    	println!("Finish!");
    	println!("Run \n 1 - Create a venv \n 2 - pip install flask ");

    }

    
}
