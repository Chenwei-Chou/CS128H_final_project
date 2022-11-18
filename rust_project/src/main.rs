use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::fs;
extern crate base64;

use base64::{encode, decode};

use md5::{Digest, Md5};

fn main() -> Result<(), Box<dyn Error>> {

    //let x = "aGVsbG8gd29ybGQ=";
    //let xx = base64::decode(x).unwrap();
    //println!("{:?}", String::from_utf8(xx).unwrap());

    let mut args = env::args();

    args.next().unwrap();

    let arg_sum = args.len();
    if arg_sum != 3 {
        println!("You should use exactly three arguments to use this program.");
        println!("Format: cargo run [arg1] [arg2] [arg3]");
        println!("arg1: key file");
        println!("arg2: type: 1 for encrypt and 2 for decrypt");
        println!("arg3: source file");
        return Ok(());
    }
    

    if let Some(file_path) = args.next() {
        let mut file = File::open(file_path)?;
        let mut buffer = [0u8; 4096];
        let mut hasher = Md5::new();
        let mut sum = 0u64;

        loop {
            let c = file.read(&mut buffer)?;

            if c == 0 {
                break;
            }

            hasher.update(&buffer[0..c]);

            sum += c as u64;
            eprintln!("{sum} bytes key file have been read");
        }

        let md5_sum = format!("{:x}", hasher.finalize());

        println!("Your key file fingerprint: {}", md5_sum);

        if md5_sum == "27851c46096777f66599c18d2acb7c1a" {
            println!("[Unlock!!]");
        }
        // 

        if let Some(new_arg) = args.next() {
            if new_arg == "1" {
                println!("type: 1 (To encrypt)");
                println!("============ orginal content begin ============");
                let file_source = args.next().unwrap();
                //println!("{}", file_source.to_string());
                let input_data: String = fs::read_to_string(file_source).unwrap();
                println!("{}", input_data);
                println!("============ orginal content end ============");
                println!("Your encrypted content");
                println!("============ encrypted content begin ============");
                println!("{}", encode(input_data));
                println!("============ encrypted content end ============");
            } else if new_arg == "2" {
                println!("type: 2 (To decrypt)");
                println!("============ orginal content begin ============");
                let file_source = args.next().unwrap();
                //intln!("{}", file_source.to_string());
                let mut input_data: String = fs::read_to_string(file_source).unwrap();
                println!("{}", &input_data);
                println!("============ orginal content end ============");
                println!("Your decrypted content");
                println!("============ decrypted content begin ============");
                //let i_data = "aGVsbG8gd29ybGQ=";
                //println!("{}", &input_data);
                //let tmp = input_data.as_str();            debug with WeustiS
                //println!("{:?}, {:?}", tmp, i_data);
                input_data.truncate(input_data.len() - 1);
                let i_output = base64::decode(input_data).unwrap();
                //let result_data = base64::decode(&input_data).unwrap();
                //let data_out = format!("{:x}", output);
                println!("{}", String::from_utf8(i_output).unwrap());
                //println!("{:?}", String::from_utf8(i_output).unwrap());
                //println!("{:?}", String::from_utf8(de_data).unwrap());
                println!("============ decrypted content end ============");
            } else {
                return Err("The input key and file can't match.".into());
            }
        }
        //let test = args.next().unwrap();
        
        Ok(())
    } else {
        Err("Please provide a file path.".into())
    }
}



/* 
use std::fs;
use md5::{Md5, Digest};
use std::io;
use std::env;
//use hex_literal::hex;
extern crate base64;
use std::str;
use base64::{encode, decode};

fn main()  {

    let a = b"hello world";
    let b = "aGVsbG8gd29ybGQ=";

    println!("{}", encode(a));

    let bytes = base64::decode(b).unwrap();
    println!("{:?}", &bytes);
    let 

    //let result = String::from_utf8(bytes).unwrap();
    //println!("{}",result);

    // for i in bytes {
    //     println!("{}", i);
    // }

    // let aa = &bytes << 2;

    // println!("{}", aa);

    // let args: Vec<String> = env::args().collect();
    // let input_key = &args[1];
	// let input_raw: String = fs::read_to_string(input_key).unwrap();

    // let mut hasher = Md5::new();
    // hasher.update(&input_raw);
    // let result = hasher.finalize();
    // let check = format!("{:x}", &result);
    // println!("Your fingerprint: {}", check);
    // if check == "f764d0c341fcf624bca058709896918e" {
    //     println!("unlock!!");
    // }
    // else {
    //     println!("fuck!");
    // }

    // let input_file = &args[2];
	// let input_raw_file: String = fs::read_to_string(input_file).unwrap();
    // println!("========== file content start ==========");
    // println!("{}", input_raw_file);
    // println!("========== file content end ============");

    // let input_type = &args[3];
    // if input_type == "1" {
    //     println!("you want to read file");
    // }else if input_type == "2" {
    //     println!("you want to encrypt file");
    // }
    // else if input_type == "3" {
    // println!("you want to decrypt file");
    // }
    //
}
*/

