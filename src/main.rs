use std::io::Write;
use std::fs;
use std::env;
use std::path::Path;
use clap::{Arg,App};

fn create_folder(foldername: String) -> std::io::Result<()>{
	fs::create_dir(foldername)?;
	Ok(())
}


fn file(filename: String , ext: String , text: &[u8]) -> std::io::Result<()>{

	let file = filename.to_string() + "." + &ext.to_string();

	let mut cfile = fs::File::create(file.to_string())?;


	cfile.write_all(text);
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
    				  .arg(
    				  	Arg::with_name("type")
    				  	.long("--type")
    				  	.takes_value(true)
    				  )
    				  .get_matches();


    if let Some(matches) = app.value_of("name") {
    	let result: String = matches.to_string();
    	create_folder(result.to_string());
    	println!("Creating folders");
    	let to_enter = "./".to_string() + &result.to_string();  
    	enter_folder(to_enter.to_string());
    	file("__init__".to_string(),"py".to_string(),b"#init");
    	
    	if let Some(matches) = app.value_of("type") {
    		let result_type = matches.to_string();

    		if result_type == "mvc" {
    			file("app".to_string(),"py".to_string(),b"from flask import Flask \nfrom ext import views  \n\napp = Flask(__name__) \nviews.init_app(app)  \n\n\n app.run(debug=True)");
    			create_folder("ext".to_string());
    			enter_folder("./ext".to_string());

    			file("__init__".to_string(),"py".to_string(),b"#init app");

    			file("views".to_string(),"py".to_string(),b"def init_app(app):\n    @app.route(\"/\") \n    def index():\n        return \"Fun!\" \n");
    			file("db".to_string(),"py".to_string(),b"from flask_sqlalchemy import SQLAlchemy \n\ndef init_app(app):\n    db = SQLAlchemy(app)");

    		}else if result_type == "rest"{
    			file("app".to_string(),"py".to_string(),b"from flask import Flask , jsonify \n\napp = Flask(__name__) \n\n@app.route(\"/\") \ndef index():\n    return jsonify(res=200) \n)\n\n\n app.run(debug=True)");
    		}else{
    			println!("Enter a valid type of application");
    		}

    	}else{
    		println!("Enter a valid type of application");
    	}


    	println!("Creating program");
    	println!("Finish!");
    	println!("Run \n 1 - Create a venv \n 2 - pip install flask flask_sqlalchemy \n 3 - python app.py ");
	}else{
		println!("Enter a valid name!");
	}

    
}
