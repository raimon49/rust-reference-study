use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        // 関数showの仮引数が&Table型の場合、forループ変数artistも&String型となる
        println!("works by {}:", artist);
        for work in works {
            // 関数showの仮引数が&Table型の場合、forループ変数workも&Vec<String>型となる
            println!(" {}", work);
        }
    }
}

fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                 vec!["many madrigals".to_string(),
                      "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
                 vec!["The Musitians".to_string(),
                      "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
                 vec!["Peres with the head of Medusa".to_string(),
                      "a salt cellar".to_string()]);

    show(&table);
    // 参照渡しをしなかった場合、関数側に所有権が移って「value borrowed here after move」で怒られる
    assert_eq!(table["Gesualdo"][0], "many madrigals");
}
