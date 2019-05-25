

fn main() {
    assert_eq!(5_f32.sqrt()*5f32.sqrt(),5.);
    println!("Hello, world!");
    println!("{}",(2_f32).sqrt());
    assert_eq!(false as i32,0);
    let text = "I see the eigenvalue in thine eye";
    let (head,tail) = text.split_at(21);
    assert_eq!(head , "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    // 配列
    let lazy_caterer: [u32; 6] = [1,2,3,4,5,6];
    let taxonomy = ["Animalia","Arthropada","Insecta"];
    //for文
    for i in lazy_caterer.iter(){
        print!("{}",i);
    }
    print!("{}","\n");
    assert_eq!(taxonomy.len() ,3);
    let mut sieve = [true; 10000];
    for i in 2..100{
        if sieve[i] {
            let mut j = i*i;
            while j < 10000{
                sieve[j] = false;
                j += i
            }
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);

    //sortをするのは配列のメソッドではなく，スライスのメソッド
    let mut chaos = [3,4,5,1,4,6];
    chaos.sort();
    assert_eq!(chaos,[1,3,4,4,5,6]);

    //vecを使えば可変長のリストとして使うことができる 一番簡単な作り方は以下の方法
    let vector = vec![2,3,4];
    assert_eq!(vector,[2,3,4]);
    //以下の方法で作っても同じ
    let mut v = Vec::new();
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    //iteratorから作る方法
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v,[0,1,2,3,4]);
    //vectorの要素を増やすときにバッファに保持されている領域が足りなくなると，新たなより大きい領域をバッファ内に確保してそっちにうつしてしまうので手間がある
    //そんなときなwith_capacityを用いる
    let mut cap_v = Vec::with_capacity(2);
    assert_eq!(cap_v.len(),0);
    assert_eq!(cap_v.capacity(),2);

    cap_v.push(1);
    cap_v.push(2);
    cap_v.push(3);
    assert_eq!(cap_v.len(),3);
    assert_eq!(cap_v.capacity(),4);

    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!("{} : {}", l,
            if l.len() % 2 == 0{
                "functional"
            }else{
                "imperative"
            }
        );
    }

    let v:Vec<f64> = vec![1.0,0.0,0.87,1.0,0.707];
    let a:[f64; 5] = [1.0,0.0,0.87,1.0,0.707];
    //svとsaはvecと配列の最初のヒープへの参照ポインタと配列やvecの長さを持っている．それを使えば所有権のない参照は可能なのである．
    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    fn print(n: &[f64]){
        for elt in n {
            println!("{}",elt);
        }    
    }

    print(sv);
    print(sa);

    //文字列リテラル
    let speech = "\"Ouch!!\" said the well. \n ";
    println!("in the room the women come and go, \
        Spring of Mount Abora");
    println!(r###"
        This raw string started with 'r###"'.
        Therefore it does not end until we reach a quote mark ('"')
        followed immediately by three signs ('###'):
        "###);
    
    
}
