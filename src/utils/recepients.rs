use std::{fs::File, io::BufReader};

use crate::utils::mail_content::Receiver;

pub fn mail_list_from_csv(filename: String) -> Vec<String> {
    //input csv from arg
    //read data then store to vec<Receiver>
    // return this data from this fn
    let mut rdr = csv::Reader::from_reader(BufReader::new(File::open("./data.csv").unwrap()));
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Receiver = result.unwrap();
        //append to vector
        // println!("email is:{:?}", record.email);
        // println!("name is:{:?}", record.name);
        // match record.team {
        //     Team::Tech => {
        //         println!("yes team is tech");
        //     }
        //     _ => {
        //         println!("team is not tech");
        //     }
        // }
    }
    vec![]
}

pub fn mail_list_test() -> String {
    "varunkkv.mec@gmail.com".to_string()
}
