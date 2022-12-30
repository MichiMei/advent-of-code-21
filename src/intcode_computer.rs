//use std::io::{BufRead, stdin};

/*fn execute() {
    let intcode = read();
    run(intcode);
}

fn read() -> Vec<u8> {
    let mut res = vec![];
    for word in stdin().lock().split(',' as u8) {
        let str = match word {
            Ok(x) => match String::from_utf8(x) {
                Ok(y) => y,
                Err(x) => {
                    println!("parse error '{:?}'", x);
                    continue;
                }
            },
            Err(_) => continue
        };
        let str = str.trim();
        let code = match str.parse() {
            Ok(x) => x,
            Err(_) => {
                println!("could not parse '{:?}'", str);
                continue;
            }
        };
        res.push(code);
    }
    res
}

fn run(intcode: &mut Vec<u8>) {
    let mut ip = 0;
    loop {
        match intcode[ip] {
            1 => {
                // 1, x, y, z
                // add [x]+[y]->[z]
                let src0 = intcode[ip+1];
                let src1 = intcode[ip+2];
                let dest = intcode[ip+3];
                intcode[dest] = intcode[src0]+intcode[src1];
                ip += 4;
            }
            2 => {
                // 2, x, y, z
                // mul [x]+[y]->[z]
                let src0 = intcode[ip+1];
                let src1 = intcode[ip+2];
                let dest = intcode[ip+3];
                intcode[dest] = intcode[src0]*intcode[src1];
                ip += 4;
            }
            99 => {
                ip += 1;
                break;
            }
            x => {
                println!("unsupported opcode {}", x);
                return;
            }
        }
    }
}*/