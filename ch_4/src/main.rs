fn main() {
    println!("Hello, world!");

    //4.1所有権
    //変数はスタックフレームにヒープ上のポインタと容積と現在の長さを持つ．これを所有という（ヒープ上のバッファを所有している）．が，この変数が使われなくなると．ヒープ上に確保されているメモリについても削除される
    fn print_padovan(){
        let mut padovan = vec![1,1,1];
        for i in 3..10{
            let next = padovan[i-3] + padovan[i-2];
            padovan.push(next);
        }
        println!("P(1..10) = {:?}",padovan);
    }
    print_padovan();

    struct Person {name: String, birth: i32};

    let mut composers = Vec::new();
    composers.push(Person {name: "Palestrina".to_string(), birth:1525});
    composers.push(Person {name: "Dowland".to_string(), birth:1563});
    composers.push(Person {name: "Lully".to_string(), birth:1632});
    for composer in &composers{
        println!("{}, born {} ", composer.name,composer.birth);
    }

    //4.2 移動
    let s = vec!["udon".to_string(),"ra-men".to_string(),"soba".to_string()];
    let t = s;
    let u = t;
    for value in &u{
        println!("{}",value);
    }

    let mut s = "Govinda".to_string();
    let t = s;
    s = "Siddhartha".to_string();
    println!("{}",t);
    println!("{}", s);

    //コピー型
    //簡単な数字や，キャラクター，bool値などはスタック上に直接値を乗せるものであり，それを複製する．
    let str1 = "somnabulance".to_string();
    let _str2 = str1;
    //上のは移動，下はコピー
    let num1:i32 = 32;
    let num2 = num1;
    println!("num1:{}",num1);
    println!("num2:{}",num2);

    #[derive(Copy,Clone)]
    struct Label{number:u32}; //Label{number:u32,name:String} はできない
    fn print(l:Label) { println!("STAMP: {}",l.number);}
    //下の変換はコピー型ではないので移動した後の参照はできないが，Label構造体のフィールドはuしかないので，copy型として定義できる(String)などの継承はできない
    let l = Label {number:3};
    print(l);
    println!("My label number is : {}",l.number);

    //4.4 Rc型とArc型
    //Rc型はスレッド安全ではないが高速に動作する．一方でArc型はスレッド安全なコードで管理できるが，速度の面でRC型に劣る．
    use std::rc::Rc;

    let s:Rc<String> = Rc::new("shirataki".to_string());
    let t:Rc<String> = s.clone();
    let u:Rc<String> = s.clone();

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);
    






}
