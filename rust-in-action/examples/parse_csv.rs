fn main() {
    println!("hello, world!");

    let penguin_data = "\
common name, length(cm)
Littel penguin, 33
Yellow-eyed penguin, 65
Fiordland penguin, 60
Invalid, data
";

    let records = penguin_data.lines();
   
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields = record.split(',').map(|field| field.trim()).collect::<Vec<_>>();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];

        if let Ok(length) = fields[1].parse::<i32>() {
            println!("{}, {}cm", name, length);
        }
    }
}