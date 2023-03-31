use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long,default_value_t=0 )]
   rows: i32,

   #[arg(short, long,default_value_t =0)]
    cols: i32,

    #[arg(short, long)]
    map: String,

}

pub fn is_touched(coordb:(i32,i32),coordv:(i32,i32)) -> bool{
   ((coordb.0 - coordv.0).abs() <= 1) && ((coordb.1 - coordv.1).abs() <= 1)
            
}

pub fn nbofbmb(voido: (i32,i32),bombe: Vec<(i32,i32)>) -> i32{
    let mut cont = 0;
    for b in bombe{
        if is_touched(b,voido){
            cont +=1;
        }
    }
    cont
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let matrix: Vec<Vec<char>> = minefield.iter().map(|&x| x.chars().collect()).collect();
    let mut matrix2: Vec<Vec<char>> = minefield.iter().map(|&x| x.chars().collect()).collect();
    let mut coordbomb :Vec<(i32,i32)>= vec!();
    let mut coordvoid :Vec<(i32,i32)>=vec!();
    let mut annotations: Vec<i32> = vec!();
    for (i,row) in matrix.iter().enumerate(){
        for (k,c) in row.iter().enumerate(){
            if *c == '*'{
                coordbomb.push((i as i32,k as i32));
            }
        }
    }
    for (i,row) in matrix.iter().enumerate(){
        for (k,c) in row.iter().enumerate(){
            if *c ==' ' || (*c).is_alphanumeric(){
                coordvoid.push((i as i32,k as i32));
            }
        }
    }
    for void in coordvoid{
        annotations.push(nbofbmb(void,coordbomb.clone()));
    }
    for (i,row) in matrix.iter().enumerate(){
        for (k,c) in row.iter().enumerate(){
            if *c ==' '{
                if annotations[0] ==0 {
                    annotations.remove(0);
                }
                else{
                   matrix2[i][k] = std::char::from_u32(annotations.remove(0) as u32).unwrap().escape_debug().to_string().replace("\\u{", "").replace('}', "").chars().next().unwrap(); 
                }
            }
        }
    }
    
    matrix2.iter()
        .map(|char_vec| char_vec.iter().collect())
        .collect()
}


fn main(){ 
    let argomenti:Args = Args::parse();
    let cols = argomenti.cols;
    let parts: Vec<&str> = argomenti.map.as_bytes().chunks(cols as usize).map(|b| std::str::from_utf8(b).unwrap()).collect();
    let mappa: &[&str] = &parts[..];

    let vecsols:Vec<String>= annotate(mappa);
    for stringa in vecsols{
        println!("{}",stringa);
    }
}