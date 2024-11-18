use rand::distributions::{Distribution, Uniform};

const MAP_WIDTH: usize = 64;
const MAP_HEIGHT: usize = 24;
static mut NEIGHBOURS: [[bool; 4]; MAP_WIDTH*MAP_HEIGHT] = [[false; 4]; MAP_WIDTH*MAP_HEIGHT];
static mut OPATH: Vec<usize> = Vec::new();
static mut PATH: [bool; MAP_WIDTH*MAP_HEIGHT] = [false; MAP_WIDTH*MAP_HEIGHT];
static mut VISITED: [bool; MAP_WIDTH*MAP_HEIGHT] = [false; MAP_WIDTH*MAP_HEIGHT];
static mut ORIGIN: usize = MAP_WIDTH*MAP_HEIGHT-1;
static mut PLAYER: usize = 0;
const FINISH: usize = MAP_WIDTH*MAP_HEIGHT-1;

const SHIFTS: usize = 10;

static mut ENABLE_ORIGIN: bool = true;
static mut ENABLE_FINISH: bool = true;
static mut ENABLE_PATH: bool = true;
static mut ENABLE_NODE_NUMBERS: bool = false;

use console::Term;

const KEY_W: char = 'w';
const KEY_S: char = 's';
const KEY_A: char = 'a';
const KEY_D: char = 'd';
const KEY_N: char = 'n';
const KEY_P: char = 'p';
const KEY_F: char = 'f';
const KEY_O: char = 'o';

unsafe fn dfs(a: usize) -> bool{
    OPATH.push(a);
    PATH[a] = true;
    VISITED[a] = true;
    if a == PLAYER {return true;};
    for direction in 0..4 {
        if player_can_move(a, direction){
            match direction {
                0 => {
                    if VISITED[a-1] {continue;}
                    if !dfs(a-1) {PATH[a-1] = false; OPATH.pop();} else {return true;}
                },
                1 => {
                    if VISITED[a-MAP_WIDTH] {continue;}
                    if !dfs(a-MAP_WIDTH) {PATH[a-MAP_WIDTH] = false; OPATH.pop();} else {return true;}
                },
                2 => {
                    if VISITED[a+1] {continue;}
                    if !dfs(a+1) {PATH[a+1] = false; OPATH.pop();} else {return true;}
                },
                3 => {
                    if VISITED[a+MAP_WIDTH] {continue;}
                    if !dfs(a+MAP_WIDTH) {PATH[a+MAP_WIDTH] = false; OPATH.pop();} else {return true;}
                },
                _ => {}
            }
        }
    }
    return false;
}

unsafe fn pause() {
    let term  = Term::stdout();
    let key = term.read_char();
    match key {
        Ok(KEY_W) => {move_player(1);shift_origin(SHIFTS);},
        Ok(KEY_S) => {move_player(3);shift_origin(SHIFTS);},
        Ok(KEY_A) => {move_player(0);shift_origin(SHIFTS);},
        Ok(KEY_D) => {move_player(2);shift_origin(SHIFTS);},
        Ok(KEY_N) => ENABLE_NODE_NUMBERS = !ENABLE_NODE_NUMBERS,
        Ok(KEY_P) => ENABLE_PATH = !ENABLE_PATH,
        Ok(KEY_F) => ENABLE_FINISH = !ENABLE_FINISH,
        Ok(KEY_O) => ENABLE_ORIGIN = !ENABLE_ORIGIN,
        _ => {}
    }
}

fn main() {
    unsafe{
        generate_starting_board();
        shift_origin(MAP_WIDTH*MAP_HEIGHT*MAP_WIDTH*MAP_HEIGHT);
        loop{
            OPATH = Vec::new();
            PATH = [false; MAP_WIDTH*MAP_HEIGHT];
            VISITED = [false; MAP_WIDTH*MAP_HEIGHT];
            dfs(FINISH);
            visualize_board();
            if check_win() {print!("HEY! YOU WON!");break;};
            pause();
        }
    }
}

unsafe fn player_can_move(node: usize, direction: usize) -> bool{
    if can_move(node, direction) {
        match direction{
            0 => return NEIGHBOURS[node][0] || NEIGHBOURS[node-1][2],
            1 => return NEIGHBOURS[node][1] || NEIGHBOURS[node-MAP_WIDTH][3],
            2 => return NEIGHBOURS[node][2] || NEIGHBOURS[node+1][0],
            3 => return NEIGHBOURS[node][3] || NEIGHBOURS[node+MAP_WIDTH][1],
            _ => return false
        }
    }
    return false;
}

unsafe fn check_win() ->  bool{
    return PLAYER == FINISH;
}

unsafe fn move_player(direction: usize){
    if !player_can_move(PLAYER, direction){return;}
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
            let fmt_idx = if ENABLE_NODE_NUMBERS {format!("{:0>2}", idx)} else {format!("{: >2}", "")};
            let mut connection = "\x1b[41m \x1b[0m";
            if NEIGHBOURS[idx][2] 
            {connection = " "}
            else if x < MAP_WIDTH-1 && NEIGHBOURS[idx+1][0] 
            {connection = " "}
            if x < MAP_WIDTH-1 && PATH[idx] && PATH[idx+1] && player_can_move(idx, 2) && ENABLE_PATH
            {connection = "\x1b[43m \x1b[0m"}
            print!("{}{fmt_idx}{}{connection}", 
                if idx == PLAYER {"\x1b[42m"}
                else if idx == ORIGIN && ENABLE_ORIGIN {"\x1b[44m"}
                else if idx == FINISH && ENABLE_FINISH {"\x1b[45m"}
                else if PATH[idx] && ENABLE_PATH {"\x1b[43m"}
                else {""}, 
                if 
                   idx == PLAYER ||
                   (idx == ORIGIN && ENABLE_ORIGIN)||
                   (idx  == FINISH && ENABLE_FINISH) || 
                   (PATH[idx] && ENABLE_PATH)
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
            if y < MAP_HEIGHT-1 && PATH[idx] && PATH[idx+MAP_WIDTH] && player_can_move(idx, 3) && ENABLE_PATH
            {connection = "\x1b[43m  \x1b[0m"}
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

unsafe fn shift_origin(shifts: usize){
    let mut rng = rand::thread_rng();
    let direction = Uniform::from(0..4);
    for _ in 0..shifts{
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
