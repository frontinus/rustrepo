

struct Test(i32);

impl Drop for Test{
    fn drop(&mut self){
        println!("Dropping Test({}) @ {:p}", self.0, self);
    }
}

fn alpha(t: Test){
    println!("Chiamata la funzione alfa =() con Test({}) @ {:p}",t.0, &t);
}

fn main() {
let t = Test(12);
println!("Creating Test({}) @ {:p}", t.0, &t);
alpha(t);
println!("it's over");
}
