fn main(){

    let a = A{ a:10};
    let b = B{ a };

}

struct  A {
    a:u64
}

struct  B {
    a: A
}