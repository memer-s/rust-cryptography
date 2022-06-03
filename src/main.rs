use colored::Colorize;
use std::io::Write;

fn main() {
    let mut cool = open_file();
    let old = cool.clone();

    let enc = [22];

    read_hex(&cool);
    for i in 0..enc.len() {
        encrypt_mut(&mut cool, enc[i]);
    }
    compare(&cool, &old);
    read_utf8(&cool);
    for i in 0..enc.len() {
        decrypt_mut(&mut cool, enc[enc.len()-1-i]);
    }
    read_hex(&cool);
    compare(&cool, &old);
    read_utf8(&cool);
}

fn encrypt_mut(v: &mut [u8], key: u8) {
    for i in 0..v.len() {
        v[i] = ((v[i] as u16 + key as u16) % 256) as u8;
        v[i] ^= key;
    }
}

fn decrypt_mut(v: &mut [u8], key: u8) {
    for i in 0..v.len() {
        v[i] ^= key;
        if (v[i] as i16 - key as i16) >= 0 {
            v[i] = (v[i] as i16 - key as i16) as u8;
        }
        else {
            v[i] = (v[i] as i16 - key as i16 + 255) as u8;
        }
    }
}

fn open_file() -> Vec<u8> {
    std::fs::read("hell").expect("Ecpi")
}

fn write_file(name: &str, buffer: &[u8]) {
    let mut f = std::fs::File::create(name).expect(":");
    f.write(buffer);
}

fn read_hex(v: &[u8]) {
    print!("--------- HEX ---------\n");
    let mut i = 0;
    for c in v {
        i+=1;
        print!("{:02X} ", c);
        if i % 8 == 0 {
            print!("\n")
        }
    }
    print!("\n--------- END ---------\n");
    print!("\n\n");
} 

fn read_utf8(v: &[u8]) {
    println!(" --- UTF-8 --- ");
    print!("{} ", std::str::from_utf8(v).expect("BRUHHH"));
    println!("\n ---  END  --- ");
    print!("\n")
}

fn compare(v1: &[u8], v2: &[u8]) {
    print!("COMPARE:\n");
    print!("--------- HEX ---------\n");
    let mut right = 0.; 
    for i in 0..v1.len() {
        if v1[i] == v2[i] {
            print!("{} ", format!("{:02X}", v1[i]).green());
            right += 1.;
        }
        else {
            print!("{} ", format!("{:02X}", v1[i]).red());
        }
        if i % 8 == 7 {
            print!("\n")
        }
    }
    print!("\n--------- END ---------\n");
    if 1. - (right / v1.len() as f32) > 0. {
        print!("       {} LOSS\n\n", format!("{:03}%",(1. - (right / v1.len() as f32))*100.).red());
    }
    else {
        print!("        {}", "NO LOSS\n\n".green());
    }
}