use rand::distributions::{Distribution, Uniform};

const MAP_WIDTH: usize = 64;
const MAP_HEIGHT: usize = 24;
static mut NEIGHBOURS: [[bool; 4]; MAP_WIDTH*MAP_HEIGHT] = [[false; 4]; MAP_WIDTH*MAP_HEIGHT];
static mut ORIGIN: usize = MAP_WIDTH*MAP_HEIGHT-1;
static mut PLAYER: usize = 0;
const FINISH: usize = MAP_WIDTH*MAP_HEIGHT-1;

use console::Term;

const KEY_W: char = 'w';
const KEY_S: char = 's';
const KEY_A: char = 'a';
const KEY_D: char = 'd';

unsafe fn pause() {
    let term  = Term::stdout();
    let key = term.read_char();
    match key {
        Ok(KEY_W) => move_player(1),
        Ok(KEY_S) => move_player(3),
        Ok(KEY_A) => move_player(0),
        Ok(KEY_D) => move_player(2),
        _ => {}
    }
}

fn main() {
    unsafe{
        generate_starting_board();
        for _ in 0..MAP_WIDTH*MAP_HEIGHT*MAP_WIDTH*MAP_HEIGHT{
            shift_origin();
        }
        loop{
            visualize_board();
            if check_win() {print!("HEY! YOU WON!");break;};
            pause();
            shift_origin();
        }
    }
}

unsafe fn player_can_move(direction: usize) -> bool{
    if can_move(PLAYER, direction) {
        match direction{
            0 => return NEIGHBOURS[PLAYER][0] || NEIGHBOURS[PLAYER-1][2],
            1 => return NEIGHBOURS[PLAYER][1] || NEIGHBOURS[PLAYER-MAP_WIDTH][3],
            2 => return NEIGHBOURS[PLAYER][2] || NEIGHBOURS[PLAYER+1][0],
            3 => return NEIGHBOURS[PLAYER][3] || NEIGHBOURS[PLAYER+MAP_WIDTH][1],
            _ => return false
        }
    }
    return false;
}

unsafe fn check_win() ->  bool{
    return PLAYER == FINISH;
}

unsafe fn move_player(direction: usize){
    if !player_can_move(direction){return;}
    match direction {
        0 => PLAYER -= 1,
        1 => PLAYER -= MAP_WIDTH,
        2 => PLAYER += 1,
        3 => PLAYER += MAP_WIDTH,
        _ => {}
    }
}

unsafe fn generate_starting_board(){
    for y in 0..MAP_HEIGHT{
        for x in 0..MAP_WIDTH{
            let idx: usize = y * MAP_WIDTH + x;
            if x < MAP_WIDTH-1{
                f1(idx, idx+1);
            }else if y < MAP_HEIGHT-1{
                f1(idx, idx+MAP_WIDTH);
            }
        }
    }
}

unsafe fn visualize_board(){
    print!("\x1b[2J\x1b[1;1H");
    for y in 0..MAP_HEIGHT {
        if y == 0{
            for _ in  0..MAP_WIDTH*3+1{
                print!("\x1b[41m \x1b[0m");
            }
            print!("\n");
        }
        print!("\x1b[41m \x1b[0m");
        for x in 0..MAP_WIDTH {
            let idx: usize = y * MAP_WIDTH + x;
            //let fmt_idx = format!("{:0>2}", idx);
            let fmt_idx = format!("{: >2}", "");
            let mut connection = "\x1b[41m \x1b[0m";
            if NEIGHBOURS[idx][2] 
            {connection = " "}
            else if x < MAP_WIDTH-1 && NEIGHBOURS[idx+1][0] 
            {connection = " "}
            print!("{}{fmt_idx}{}{connection}", 
                if idx == PLAYER {"\x1b[42m"}
                else if idx == ORIGIN {"\x1b[44m"}
                else if idx == FINISH {"\x1b[45m"}
                else {""}, 
                if 
                   idx == ORIGIN ||
                   idx == PLAYER ||
                   idx  == FINISH 
                   {"\x1b[0m"}
                else {""});
        }
        print!("\n");
        print!("\x1b[41m \x1b[0m");
        for x in 0..MAP_WIDTH {
            let idx: usize = y * MAP_WIDTH + x;
            let spaces = format!("{: >1}", "");
            let mut connection = "\x1b[41m  \x1b[0m";
            if NEIGHBOURS[idx][3] 
            {connection = "  "}
            else if y < MAP_HEIGHT-1 && NEIGHBOURS[idx+MAP_WIDTH][1] 
            {connection = "  "}
            print!("{connection}\x1b[41m{spaces}\x1b[0m");
        }
        print!("\n");
    }
}

unsafe fn can_move(node: usize, direction: usize) -> bool {
    if node % MAP_WIDTH == 0 && direction == 0 {return false;}
    else if node % MAP_WIDTH == MAP_WIDTH-1 && direction == 2 {return false;}
    else if i32::try_from(node).unwrap() - i32::try_from(MAP_WIDTH).unwrap() < 0 && direction == 1 {return false;}
    else if i32::try_from(node).unwrap() + i32::try_from(MAP_WIDTH).unwrap() >= i32::try_from(MAP_WIDTH*MAP_HEIGHT).unwrap() && direction == 3 {return false;}
    return true;
}

unsafe fn shift_origin(){
    let mut rng = rand::thread_rng();
    let direction = Uniform::from(0..4);
    let mut throw;
    loop {
        throw = direction.sample(&mut rng);
        if can_move(ORIGIN, throw) {
            break;
        }
    }
    NEIGHBOURS[ORIGIN][throw] = true;
    if throw == 0 {
        NEIGHBOURS[ORIGIN - 1]  = [false; 4];
        ORIGIN -= 1;
    }
    if throw == 1 {
        NEIGHBOURS[ORIGIN - MAP_WIDTH] = [false; 4];
        ORIGIN -= MAP_WIDTH;
    }
    if throw == 2 {
        NEIGHBOURS[ORIGIN + 1] = [false; 4];
        ORIGIN += 1;
    }
    if throw == 3 {
        NEIGHBOURS[ORIGIN + MAP_WIDTH] = [false; 4];
        ORIGIN += MAP_WIDTH;
    }
}

unsafe fn f1(origin: usize, destination: usize){
    if origin == destination+1 {
        NEIGHBOURS[origin][0] = true;
    }else if origin == destination+MAP_WIDTH {
        NEIGHBOURS[origin][1] = true;
    }else if origin == destination-1 {
        NEIGHBOURS[origin][2] = true;
    }else if origin == destination-MAP_WIDTH{
        NEIGHBOURS[origin][3] = true;
    }
}
