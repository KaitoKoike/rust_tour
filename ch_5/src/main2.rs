
fn main(){
    let x = 1;
    {
        let r ;
        r = &x;
        assert_eq!(*r, 1);
    }
    //global変数はstaticで表される
    //staticは初期化されねばならない
    //可変なstatic変数はスレッド安全ではない．
    //
    /*
    * static mut STASH: &i32;
    * fn f(p:&i32)
    * 上のコードはコンパイルできないがunsafe内に入れば可変なstatic変数もコンパイルできる
    */ 
    
    /**
     * static mut STASH: &i32 = &128;
        fn f(p: &i32){
            unsafe{
                STASH = p;
            }
        }
    これだとStashの生存期間とpの生存期間が一致するとは限らない
     */
    static mut STASH: &i32 = &128;
    fn f(p:&'static i32){
        unsafe{
            STASH = p;
        }
    }
    //このコードを読めば，グローバルな変数が隠れているかどうかがわかるのでじっくりコードを読む必要はない

    fn smallest<'a>(v:&'a [i32]) -> &i32{
        let mut s = &v[0];
        for r in &v[1..]{
            if *r < *s{
                s = r;
            }
        }
        s
    }
    let parabola = [9,4,1,0,1,4,9];
    let s = smallest(&parabola);
    assert_eq!(*s, 0);
}