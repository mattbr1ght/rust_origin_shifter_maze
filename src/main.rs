// To be like the rock that the waves keep crashing over. It stands unmoved and the raging of the sea falls still around it.
//     -- Marcus Aurelius

use rand::Rng;
use rand::distributions::{Distribution, Uniform};

const MAP_SIZE: usize = 8;
static mut NEIGHBOURS: [[bool; 4]; MAP_SIZE*MAP_SIZE] = [[false; 4]; MAP_SIZE*MAP_SIZE];
static mut VISITED: [bool; MAP_SIZE*MAP_SIZE] = [false; MAP_SIZE*MAP_SIZE];
static mut ORIGIN: usize = MAP_SIZE*MAP_SIZE-1;

use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    unsafe{
        for y in 0..MAP_SIZE{
            for x in 0..MAP_SIZE{
                let idx: usize = y * MAP_SIZE + x;
                if x < MAP_SIZE-1{
                    f1(idx, idx+1);
                }else if y < MAP_SIZE-1{
                    f1(idx, idx+MAP_SIZE);
                    //println!("[{x};{y}] : {idx} | {}", MAP_SIZE-1);
                }
                //println!("[{x};{y}] : {idx} | {}", MAP_SIZE-1);
            }
        }
        //return;

        loop{
        print!("\x1b[2J\x1b[1;1H");
        print!("\x1b[41m{}\x1b[0m\n", format!("{: >25}", ""));
        for y in 0..MAP_SIZE {
            print!("\x1b[41m \x1b[0m");
            for x in 0..MAP_SIZE {
                let idx: usize = y * MAP_SIZE + x;
                //let fmt_idx = format!("{:0>2}", idx);
                let fmt_idx = format!("{: >2}", "");
                let mut connection = "\x1b[41m \x1b[0m";
                if NEIGHBOURS[idx][2] 
                {connection = " "}
                else if x < MAP_SIZE-1 && NEIGHBOURS[idx+1][0] 
                {connection = " "}
                print!("{}{fmt_idx}{}{connection}", if idx == ORIGIN {"\x1b[44m"} else {""}, if idx == ORIGIN {"\x1b[0m"} else {""});
            }
            print!("\n");
            print!("\x1b[41m \x1b[0m");
            for x in 0..MAP_SIZE {
                let idx: usize = y * MAP_SIZE + x;
                let spaces = format!("{: >1}", "");
                let mut connection = "\x1b[41m  \x1b[0m";
                if NEIGHBOURS[idx][3] 
                {connection = "  "}
                else if y < MAP_SIZE-1 && NEIGHBOURS[idx+MAP_SIZE][1] 
                {connection = "  "}
                print!("{connection}\x1b[41m{spaces}\x1b[0m");
            }
            print!("\n");
        }
        pause();
        shift_origin();
        }
    }
}

unsafe fn can_move(direction: usize) -> bool {
    if ORIGIN % MAP_SIZE == 0 && direction == 0 {return false;}
    else if ORIGIN % MAP_SIZE == MAP_SIZE-1 && direction == 2 {return false;}
    else if i32::try_from(ORIGIN).unwrap() - i32::try_from(MAP_SIZE).unwrap() < 0 && direction == 1 {return false;}
    else if i32::try_from(ORIGIN).unwrap() + i32::try_from(MAP_SIZE).unwrap() >= i32::try_from(MAP_SIZE*MAP_SIZE).unwrap() && direction == 3 {return false;}
    return true;
}

unsafe fn shift_origin(){
    let mut rng = rand::thread_rng();
    let direction = Uniform::from(0..4);
    let mut throw;
    loop {
        throw = direction.sample(&mut rng);
        println!("throw: {throw}");
        if can_move(throw) {
            println!("{} can move in direction {}", ORIGIN, throw);
            break;
        }
    }
    NEIGHBOURS[ORIGIN][throw] = true;
    if throw == 0 {
        NEIGHBOURS[ORIGIN - 1]  = [false; 4];
        ORIGIN -= 1;
    }
    if throw == 1 {
        NEIGHBOURS[ORIGIN - MAP_SIZE] = [false; 4];
        ORIGIN -= MAP_SIZE;
    }
    if throw == 2 {
        NEIGHBOURS[ORIGIN + 1] = [false; 4];
        ORIGIN += 1;
    }
    if throw == 3 {
        NEIGHBOURS[ORIGIN + MAP_SIZE] = [false; 4];
        ORIGIN += MAP_SIZE;
    }
    //print!("{}", throw);
}

unsafe fn f1(origin: usize, destination: usize){
    //NEIGHBOURS[origin].push(destination);
    if origin == destination+1 {
        NEIGHBOURS[origin][0] = true;
    }else if origin == destination+MAP_SIZE {
        NEIGHBOURS[origin][1] = true;
    }else if origin == destination-1 {
        NEIGHBOURS[origin][2] = true;
    }else if origin == destination-MAP_SIZE{
        NEIGHBOURS[origin][3] = true;
    }
}

// unsafe fn f(a: usize, b: usize){
//     //NEIGHBOURS[a].push(b);
//     //NEIGHBOURS[b].push(a);
// }

// unsafe fn dfs(a: usize){
//     VISITED[a] = true;
//     print!("{} ", a/MAP_SIZE + a%MAP_SIZE);
//     for b in NEIGHBOURS[a] {
//         if VISITED[b] == true{
//             continue;
//         }
//         dfs(b);
//     }
// }
