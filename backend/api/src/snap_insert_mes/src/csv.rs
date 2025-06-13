use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
};

pub fn write_to_csv(filename: &str, insert_sizes: &[usize], mes: &[f64]) {
    if insert_sizes.len() != mes.len() {
        panic!("Different length!");
    }

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .expect("Failed to open or create file");

    let mut writer = BufWriter::new(file);

    writeln!(writer, "insert_size,measure").expect("Failed to write header");

    for (size, measure) in insert_sizes.into_iter().zip(mes.into_iter()) {
        writeln!(writer, "{},{}", size, measure).expect("Failed to write row");
    }
}

pub fn read_from_csv(filename: &str) -> (Vec<usize>, Vec<f64>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut insert_sizes = Vec::new();
    let mut measures = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if i == 0 {
            continue;
        }
        let parts: Vec<&str> = line.trim().split(',').collect();
        if parts.len() != 2 {
            panic!("Invalid format at line {}", i + 1);
        }

        let insert_size = parts[0].parse::<usize>().unwrap();
        let measure = parts[1].parse::<f64>().unwrap();

        insert_sizes.push(insert_size);
        measures.push(measure);
    }

    (insert_sizes, measures)
}
