use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::process::Command;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-22-8
*/

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct FASTA {
    pub id: String,
    pub sequence: String,
}

#[tokio::main]
pub async fn readfasta(pathfile: &str) -> Result<Vec<FASTA>, Box<dyn Error>> {
    let fileopen = File::open(pathfile).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let mut fastavec: Vec<FASTA> = Vec::new();
    let mut id: Vec<String> = Vec::new();
    let mut sequence: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            id.push(line.replace(">", ""));
        } else if !line.starts_with("#") {
            sequence.push(line);
        }
    }
    for i in 0..id.len() {
        fastavec.push(FASTA {
            id: id[i].clone().to_string(),
            sequence: sequence[i].clone().to_string(),
        })
    }

    Ok(fastavec)
}

#[tokio::main]
pub async fn generatetag(pathfile: &str) -> Result<String, Box<dyn Error>> {
    let fastaunpack: Vec<FASTA> = readfasta(pathfile).expect("file not present");
    let mut fastawrite = File::create("fastafile").expect("file not present");
    for i in fastaunpack.iter() {
        writeln!(fastawrite, "{}", i.sequence).expect("File not found");
    }
    let _ = Command::new("sh")
        .arg("runmd5.sh")
        .output()
        .expect("command failed");

    let fileopenadd = File::open("estimatetag.txt").expect("file not present");
    let fileopenreadadd = BufReader::new(fileopenadd);
    let mut sequenceid: Vec<String> = Vec::new();
    for i in fastaunpack.iter() {
        sequenceid.push(i.id.clone());
    }
    // making a new vector for the doi generator
    let mut doivec: Vec<String> = Vec::new();
    let mut doifinaltag: Vec<String> = Vec::new();
    for i in fileopenreadadd.lines() {
        let tagline = i.expect("line not found");
        let tagcollectstring = tagline
            .chars()
            .collect::<Vec<_>>()
            .into_iter()
            .take(6)
            .collect::<Vec<char>>()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .concat()
            .to_string();

        let finaltagabout = tagline
            .chars()
            .collect::<Vec<_>>()
            .into_iter()
            .skip(6)
            .take(6)
            .collect::<Vec<char>>()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .concat()
            .to_string();
        doivec.push(tagcollectstring);
        doifinaltag.push(finaltagabout);
    }

    let mut finaltag: Vec<_> = Vec::new();
    for i in 0..sequenceid.len() {
        let doitag = format!("{}/{}/{}", "doi.org", doivec[i], doifinaltag[i]);
        finaltag.push(doitag);
    }

    let mut finaltagwrite = File::create("doigenerated.txt").expect("file not present");
    for i in 0..finaltag.len() {
        writeln!(finaltagwrite, "{}\t{}", sequenceid[i], finaltag[i]).expect("File not found");
    }

    Ok("the doi for the sequences have been written".to_string())
}
