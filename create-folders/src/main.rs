use clap::{Parser, Subcommand};
use std::path::Path;
use std::fs;

#[derive(Parser)]
#[command(name="CreateDirs")]
#[command(version="0.1.0")]
#[command(about="Creatin directories according to the specified algoritms")]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create dirs whit constant root
    Root {
        /// dir name
        root: String,

        /// dir name prefix
        #[arg(short, long)]
        pref: Vec<String>,

        /// dir name end
        #[arg(short, long)]
        end: Vec<String>,

        /// dir name separator
        #[arg(short, long, default_value_t = String::from("_"))]
        sep: String

    }
}

fn main() {
    let cli = Cli::parse();

    let result: Result<(), &'static str> = match cli.command {
        Commands::Root { 
            root, 
            pref, 
            end ,
            sep,
        } => create_dirs_whit_const_root(String::from("./"), root, pref, end, sep)
    };

    result.unwrap()
}

/// Create dirs
/// 
/// 
fn create_dirs_whit_const_root(dir: String, root: String, pref: Vec<String>, end: Vec<String>, sep: String) -> Result<(), &'static str> {
    let bas_dir: &Path = Path::new(&dir);


    if !bas_dir.is_dir() {
        return Err("Bas dir don't exist");
    }




    let max_lengt = [pref.len(), end.len()].iter().max().unwrap().clone();
    
    // String::from(value)
    let mut  pref_iter = pref.iter();
    let mut end_iter = end.iter();

    for num in 0..max_lengt {


        let plasholder = String::from("");
        let pref_name = pref_iter.next().unwrap_or(&plasholder);
        let end_name = end_iter.next().unwrap_or(&plasholder);
        let dir_name = format!("{}{}{}{}{}", pref_name, &sep, &root, &sep, end_name);
        // println!("file name: {}{}{}{}{}", pref_name, &sep, &root, &sep, end_name);
        let path = bas_dir.join(dir_name);

        if !path.is_dir() {
            println!("{:?}", path);

            fs::create_dir(path).unwrap();
            
        }

        
    }



   
    // println!("dit: {:?}", dir);
    // // println!("root: {:?}", root);
    // println!("pref: {:?}", pref);
    // println!("end: {:?}", end);
    // println!("sep: {:?}", sep);
    Ok(())
}