use rocksdb::{Options, DB};

fn main() {
    // let user_name = "Rahul";
    let user_id = "userid123431";

    // create options
    let mut opts = Options::default();
    opts.create_if_missing(true);
    
    // open or create the db
    let db = DB::open(&opts, "my_rocks_db").unwrap();
    // put a key-value pair
    db.put(b"user_id", user_id).unwrap();


    match db.get(b"user_id") {
        Ok(Some(value)) => println!("User ID: {}", String::from_utf8(value).unwrap()),
        Ok(None) => println!("User ID not found"),
        Err(e) => println!("Error: {}", e),
    }

    db.delete(b"user_id").expect("Failed to delete user_id");

    match db.get(b"user_id"){
        Ok(Some(_value)) => println!("This should not get printed"),
        Ok(None) => println!("User ID deleted"),
        Err(e) => println!("Error: {}", e),
    }
}
