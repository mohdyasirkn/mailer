use std::{fs::File, io::BufReader};

use crate::utils::mail_content::Receiver;

pub fn mail_list_from_csv() -> Vec<Receiver> {
    //input csv from arg
    //read data then store to vec<Receiver>
    // return this data from this fn
    let mut vec: Vec<Receiver> = vec![];
    let mut rdr = csv::Reader::from_reader(BufReader::new(File::open("./Recepients.csv").unwrap()));
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let mut record: Receiver = result.unwrap();
        vec.push(record);
    }
    vec
}

pub fn mail_list_test() -> String {
    "varunkkv.mec@gmail.com".to_string()
}
