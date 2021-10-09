use toy_vec::ToyVec;

fn main() {
    let mut v = ToyVec::new();
    v.push("最初の要素");
    v.push("次の要素");

    let mut iter = v.iter();

    // これはコンパイルエラーとなる
    // v.push("おお");

    assert_eq!(iter.next(), Some(&"最初の要素"));

    // これはコンパイルが通る
    v.push("おお");
}
