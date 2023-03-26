use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long)]
   stringa: String,
}

fn conv(c: char) -> char {
    const SUBS_I : &str ="àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O: &str ="aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";
    let mut flag : bool = false;
    let mut i = 0;
    for char in SUBS_I.chars(){
        if c == char{
            flag = true;
            break;
        }
        i+=1;
    }
    if flag == true{
        return SUBS_O.chars().nth(i).unwrap();
    }
    else{
        return c;
    }
}

fn slugify(s: &str)-> String{
    if s == ""{
        return s.to_string();
    }
    let mut result = String::new();
    for c in s.chars(){
        let c_modificato = match c{
            'A'..='Z' if c.is_uppercase() => c.to_ascii_lowercase(),
            c if !c.is_alphanumeric() => '-',
            _ => conv(c),
        };
        result.push(c_modificato);
    }
    //let mut last_char = None;
    //let mut i =0;
    //let mut my_chars: Vec<char> = result.chars().collect();
    //for c in result.chars() {
        //if let Some(last) = last_char {
            //if c == last && c =='-'{
               // my_chars.remove(i);
             //   break;
           // }
         //   i+=1;
       // }
     //   last_char = Some(c);
   // }
    let mut my_chars: Vec<char> = result.chars().collect();
    let mut i = 0;

    while i < my_chars.len() - 1 {
    if my_chars[i] == '-' && my_chars[i + 1] == '-' {
        my_chars.remove(i);
    } else {
        i += 1;
        }
    }




    let mut stringfin: String = my_chars.iter().collect();

    if stringfin.len() > 1 && stringfin.ends_with('-'){
        stringfin.pop();
    }

    return stringfin.to_string();
}


fn main(){
    let original_string = &Args::parse().stringa;

    //let original_string ="mèprù))dàLP?";
    let toprint: String = slugify(original_string);
    println!("{}",toprint);
}

#[test]
fn test_della_casa(){
    let argomenti = "mèprù))dàLP?";
    assert_eq!(slugify(argomenti),"mepru-dalp");
}

#[test]
fn many_palabras(){
    let argomenti = "Hello World";
    assert_eq!(slugify(argomenti), "hello-world");
}
#[test]
fn accenti(){
    let argomenti = "Café au lait";
    assert_eq!(slugify(argomenti),"cafe-au-lait");
}
#[test]
fn strvuot(){
    let argomenti = "";
    assert_eq!(slugify(argomenti),"");
}
#[test]
fn invalidcharsnspaces(){
    let argomenti = " This !is #an invalid %string ";
    assert_eq!(slugify(argomenti),"-this-is-an-invalid-string");
}
#[test]
fn invalidchars(){
    let argomenti = "#%$&^*()";
    assert_eq!(slugify(argomenti),"-");
}