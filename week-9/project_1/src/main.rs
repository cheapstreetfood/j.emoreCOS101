use std::io::Write;
use std::fs::OpenOptions;
extern crate csv;

fn main() {
    let mut file = std::fs::File::create("data.csv").expect("Create failed");
    let mut filechange = OpenOptions::new().append(true).open("data.csv").expect("cannot open file");
 
   let file_path = std::path::Path::new("data.csv");
   let mut wtr = csv::Writer::from_path(file_path).expect("path failed");


    wtr.write_record(["  ", "Nigerian Brewery Ltd"]);
    wtr.write_record(&["Lager","Stout","Non-Alchoholic\n"]);
    wtr.write_record(&["33 Export","Legend","Maltina\n"]);
    wtr.write_record(&["Desperados","Turbo King","Amstel Malta\n"]);
    wtr.write_record(&["Goldberg","Williams","Malta Gold\n"]);
    wtr.write_record(&["Gulder","    ","Fayrouz\n"]);
    wtr.write_record(&["Heineken\n"]);
    wtr.write_record(&["Star"]);
    

}
