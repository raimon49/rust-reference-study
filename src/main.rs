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

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        // sortは破壊的な操作であるため可変参照（mutable reference）の&mut Table型で受け取る必要がある
        works.sort();
    }
}

fn init_ref() {
    let x = 10;
    let r = &x; // &xはxへの共有参照
    assert!(*r == 10); // 明示的にrを参照解決

    let mut y = 32;
    let m = &mut y; // &mut yはyへの可変参照
    *m += 32; // 明示的にmを参照解決してyの値を変更
    assert!(*m == 64);

    struct Anime {
        name: &'static str,
        bachdel_pass: bool
    };
    let aria = Anime{ name: "Aria: The Animation", bachdel_pass: true };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert_eq!(anime_ref.bachdel_pass, true);
    assert_eq!((*anime_ref).name, "Aria: The Animation") // (*anime_ref) を明示しなくても暗黙的に左オペランドは参照解決される
}

fn assignment_ref() {
    let x = 10;
    let y = 20;
    let mut r = &x;
    r = &y; // 参照は代入されると新しい値を指すようになる（C++では再代入できない）

    assert!(*r == 10 || *r == 20);

    struct Point { x: i32, y: i32 };
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr; // Rustでは参照の参照は許可されている
    assert_eq!(rrr.y, 729);  // 何段でも暗黙的に参照解決されて値を取り出して比較される
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

    sort_works(&mut table);
    assert_eq!(table["Gesualdo"][0], "Tenebrae Responsoria"); // sortでworksの順序が変更済み

    init_ref();

    assignment_ref();
}
