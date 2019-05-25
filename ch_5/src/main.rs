use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn show(table: &Table){
    for (artist, works) in table{
        println!("works by {} : ", artist);
        for work in works{
            println!(" {}", work);
        }
    }
}

fn sort_works(table: &mut Table){
    for (_artist,works) in table{
        works.sort();
    }
}

fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(),"Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),vec!["The Musicians".to_string(),"The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(),"a salt celler".to_string()]);
    show(&table);
    sort_works(&mut table);
    println!("After sorted " );
    show(&table);
    //参照渡しと値渡しの違い
    //参照渡しと値渡しはC++にもあるが，rustの場合は参照にも可変参照とただの参照が明示的に分かれている．
    let x = 10;
    let r = &x;
    assert!(*r == 10);

    //可変参照 は&mutを用いて渡してあげる
    let mut y = 10;
    let a = &mut y;
    assert!(*a == 10);
    *a = *a + 34;
    assert!(*a == 44);

    let x = 10;
    let y = 20;
    let b = true;
    let mut r = &x;
    if b { r = &y ;}

    assert!(*r==10 || *r==20);
    
    //参照への参照
    struct Point {x:i32,y:i32};
    let point = Point{x:1000,y:729};
    let r: &Point = &point;
    let rr : &&Point = &r;
    let rrr: &&&Point = &rr;
    assert!(rrr.y == 729);
    //参照の場合，.yで参照解決するまで遡ることができる

    //これは比較演算子でも参照解決するまで奥に潜っていく
    let x= 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);
    //std::ptr::eqを用いてばポインタが同じかどうかの判定ができる
    //assert!(std::ptr::eq(rx,ry));

    //参照の際は変数だけでなく，関数にも使える．その場合は関数の式の値を保持する無名の変数を作り参照がそれを指すようにする．






}
