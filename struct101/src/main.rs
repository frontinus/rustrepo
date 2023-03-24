mod esempio{

    #[derive(Debug)]
    pub struct S2{
        alfa: i32,
        beta: bool
    }
    impl S2{
        pub fn adalgisa(alfa:i32, beta:bool) -> Self{
            Self{
                alfa: alfa,
                beta: beta
            }
        }
        pub fn getalfa(&self) -> i32{
            return self.alfa;
        }
        pub fn getbeta(&self) -> bool{
            return self.beta;
        }
    }
}
use esempio::S2;
 #[derive(Debug)]
struct S1{
    alfa: i32,
    beta: bool
}

fn main() {
    let s1 = S1{alfa: 5, beta: false};
    let s2 = S2::adalgisa(5, false);
    println!("{:?}",s1.alfa);
    println!("{:?}",s1.beta);
    println!("{:?}",s2.getalfa());
    println!("{:?}",s2.getalfa());
    println!("{:?}",s2.getbeta());
}
