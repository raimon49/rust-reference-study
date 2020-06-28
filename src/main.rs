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
    assert_eq!(*r, 10);
    r = &y; // 参照は代入されると新しい値を指すようになる（C++では再代入できない）

    assert!(*r == 10 || *r == 20);

    struct Point { x: i32, y: i32 };
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr; // Rustでは参照の参照は許可されている
    assert_eq!(rrr.x, 1000); // 何段でも暗黙的に参照解決されて値を取り出して比較される
    assert_eq!(rrr.y, 729);
}

fn compare_ref() {
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    // .演算子と同様に、比較演算子も、連鎖する参照を何段でも見通して解決できる
    assert!(rrx <= rry);
    assert!(rrx == rry);

    // 参照先の値を取り出しての比較でなく同じメモリを指しているか比較したい時はstd::ptr::eqを使う
    assert!(rx == ry);
    assert!(!std::ptr::eq(rx, ry)); // 値は同じだが別のメモリ上にあるため等しくならない
}

fn factorial_ref() {
    fn factorial(n: usize) -> usize {
        (1..n+1).fold(1, |a, b|a * b)
    }

    let r = &factorial(6);

    // 数値演算子は、参照を一段見通すことができる
    assert_eq!(r + &1009, 1729);
}

fn lifetime_ref() {
    {
        let x = 1;
        let v;
        {
            let r = &x; // 参照rが借用先xの生存期間に包含している時、コンパイラはエラーを起こさない
            assert_eq!(*r, 1);

            v = vec![1, 2, 3];
        }
        let vr = &v[1];
        assert_eq!(*vr, 2);
    }
}

static mut STASH: &i32 = &10; // static変数は初期化されなければコンパイルできない
static WORTH_POINTING_AT: i32 = 1000;

// 関数fは'static（tick-static生存期間）の参照を受け取るとシグネチャで宣言
fn f(p: &'static i32) {
    unsafe { // 可変なstatic変数はスレッドセーフでないためunsafeブロックの中でしかアクセスできない
        STASH = p;
    }
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r; }
    }

    s
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

    compare_ref();

    factorial_ref();

    lifetime_ref();

    f(&WORTH_POINTING_AT);
}
