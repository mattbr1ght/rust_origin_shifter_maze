use rand::distributions::{Distribution, Uniform};
use std::time::SystemTime;
use clap::Parser;
use std::fs;
use console::Term;

//constants
static mut MAP_WIDTH  : usize = 0;
static mut MAP_HEIGHT : usize = 0;
//collections
static mut MAP        : Vec<Vec<char>> = Vec::new();
static mut NEIGHBOURS : Vec<[bool; 4]> = Vec::new();
static mut OPATH      : Vec<usize>     = Vec::new();
static mut PATH       : Vec<bool>      = Vec::new();
static mut VISITED    : Vec<bool>      = Vec::new();
static mut DVISITED   : Vec<bool>      = Vec::new();
static mut DISTANCES  : Vec<usize>     = Vec::new();

//static mut DBEEN      : Vec<bool>     = Vec::new(); //for new feature - shadow memory that
//remembers where you've been and doesn't shadow those areas

//special nodes
static mut ORIGIN : usize = 0;
static mut PLAYER : usize = 0;
static mut FINISH : usize = 0;
// statistics
static mut MOVES_MADE       : usize = 0;
static mut TOTAL_MOVES_MADE : usize = 0;
static mut SHORTEST_PATH    : usize = 0;
//variables and options
static mut SHIFTS               : usize = 1;
static mut ENABLE_SHIFTS        : bool  = false;
static mut SHIFT_ONLY_ON_MOVE   : bool  = false;

static mut ENABLE_ORIGIN        : bool = false;
static mut ENABLE_FINISH        : bool = true;
static mut ENABLE_PATH          : bool = false;
static mut ENABLE_NODE_NUMBERS  : bool = false;

static mut VIEW_DISTANCE        : usize = 2;
static mut ENABLE_SHADOWS       : bool = false;
static mut ENABLE_SHADOW_MEMORY : bool = false;

static mut ENABLE_DEBUG         : bool = false;
static mut ENABLE_INSTRUCTIONS  : bool = false;

//keys
const KEY_W : char = 'w';
const KEY_S : char = 's';
const KEY_A : char = 'a';
const KEY_D : char = 'd';

const KEY_N : char = 'n';
const KEY_P : char = 'p';
const KEY_F : char = 'f';
const KEY_O : char = 'o';
const KEY_I : char = 'i';
const KEY_U : char = 'u';
const KEY_Z : char = 'z';
const KEY_X : char = 'x';
const KEY_K : char = 'k';

//clap struct
#[derive(Parser)]
#[command(author = "Mateusz Wroński (mattbr1ght) => matt@mattbr1ght.dev")]
#[command(after_help = "Author: Mateusz Wroński (mattbr1ght) => matt@mattbr1ght.dev")]
#[command(version = "1.0.0")]
#[command(about = "A program to load, generate and solve mazes. (allows movement)")]
struct Args {
    #[arg(short='W', long, default_value="10")]
    /// Width of the generated maze (ignored if file is specified)
    width: usize,  

    #[arg(short='H', long, default_value="10")]
    /// Height of the generated maze (ignored if file is specified)
    height: usize,  

    #[arg(short, long, value_parser = validate_file_path)]
    /// File to load as the maze
    file: Option<String>, 

    #[arg(short, long)]
    /// When specified displays some extra parameters and options
    debug: bool, 
    
    #[arg(short, long)]
    /// When specified instructions will be turned on
    instructions: bool,

    
    #[arg(short = 'D', long, default_value="2")]
    /// When specified the player can only see up to a given view distance
    view_distance: usize,

    #[arg(short = 'S', long)]
    /// When specified the player can only see up to a given view distance
    shadows: bool,

    #[arg(short, long)]
    /// When specified shifting will be turned on
    shift: bool,

    #[arg(short = 'O', long)]
    /// When specified shifting will occur only on legal move # NOTE: WILL WORK ONLY IF `shift` switch
    /// was used

    shift_on_move: bool,
    #[arg(short, long, default_value="1")]
    /// Number of how many shifts are made by the origin per player move # NOTE: WILL WORK ONLY IF `shift` switch
    /// was used
    number_of_shifts: usize,
}

fn main() {
    let args = Args::parse();

    unsafe{
        ENABLE_DEBUG = args.debug;
        ENABLE_INSTRUCTIONS = args.instructions;
        SHIFT_ONLY_ON_MOVE = args.shift_on_move;
        ENABLE_SHIFTS = args.shift;
        ENABLE_SHADOWS = args.shadows;
        SHIFTS = args.number_of_shifts;
        VIEW_DISTANCE = args.view_distance;

        if let Some(file) = args.file {
            load_maze_from_file(file);
            visualize_board();
        } else{
            initialize(args.width, args.height);
            generate_starting_board();
            shift_origin(MAP_WIDTH*MAP_HEIGHT*MAP_WIDTH*MAP_HEIGHT);
        }

        dfs(FINISH);

        if !ENABLE_SHIFTS {
            for n in PATH.iter() {
                if *n { SHORTEST_PATH += 1 }
            }
            SHORTEST_PATH -= 1;
        }

        if ENABLE_SHADOWS {
            distances(PLAYER, 0);
        }

        let start = SystemTime::now();
        loop{
            visualize_board();
            print_options();

            if check_win() {print!("HEY! YOU WON!");break;};
            
            pause();
            if check_win() && ENABLE_SHADOWS {ENABLE_SHADOWS=false;}

            OPATH = Vec::new();
            PATH = vec![false; MAP_WIDTH*MAP_HEIGHT];
            VISITED = vec![false; MAP_WIDTH*MAP_HEIGHT];
            dfs(FINISH);
            if ENABLE_SHADOWS {
                DVISITED = vec![false; MAP_WIDTH*MAP_HEIGHT];
                distances(PLAYER, 0);
            }
        }
        match start.elapsed() {
            Ok(elapsed) => print!(" {:?} | Moves made: {}/{} (legal/illegal)", elapsed, MOVES_MADE, TOTAL_MOVES_MADE),
            Err(err) => panic!("Error: {err}"),
        }
        if !ENABLE_SHIFTS {
            print!(" | Shortest path: {}", SHORTEST_PATH);
        }
    }
}

fn validate_file_path(name: &str) -> Result<String, String> {
    if name.trim().len() != name.len() {
        Err(String::from(
            "File path cannot have leading and trailing space",
        ))
    } else {
        Ok(name.trim().to_owned())
    }
}

unsafe fn dfs(a: usize) -> bool{
    OPATH.push(a);
    PATH[a] = true;
    VISITED[a] = true;
    if a == PLAYER {return true;};
    for direction in 0..4 {

        if !player_can_move(a, direction){continue;}

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
    return false;
}

unsafe fn distances(a: usize, distance: usize){

    DISTANCES[a] = distance;
    DVISITED[a] = true;
    for direction in 0..4 {

        if !player_can_move(a, direction){continue;}

        match direction {
            0 => {
                if DVISITED[a-1] {continue;}
                distances(a-1, distance+1);
            },
            1 => {
                if DVISITED[a-MAP_WIDTH] {continue;}
                distances(a-MAP_WIDTH, distance+1);
            },
            2 => {
                if DVISITED[a+1] {continue;}
                distances(a+1, distance+1);
            },
            3 => {
                if DVISITED[a+MAP_WIDTH] {continue;}
                distances(a+MAP_WIDTH, distance+1);
            },
            _ => {}
        }
    }
}

unsafe fn pause() {
    let term  = Term::stdout();
    let key = term.read_char();
    match key {
        Ok(KEY_W) => {move_player(1);},
        Ok(KEY_S) => {move_player(3);},
        Ok(KEY_A) => {move_player(0);},
        Ok(KEY_D) => {move_player(2);},
        Ok(KEY_N) => ENABLE_NODE_NUMBERS = !ENABLE_NODE_NUMBERS,
        Ok(KEY_P) => ENABLE_PATH = !ENABLE_PATH,
        Ok(KEY_F) => ENABLE_FINISH = !ENABLE_FINISH,
        Ok(KEY_O) => ENABLE_ORIGIN = !ENABLE_ORIGIN,
        Ok(KEY_I) => if SHIFTS+1<=usize::MAX {SHIFTS+=1;},
        Ok(KEY_U) => if SHIFTS-1>=1 {SHIFTS-=1;},
        Ok(KEY_Z) => ENABLE_SHIFTS = !ENABLE_SHIFTS,
        Ok(KEY_X) => SHIFT_ONLY_ON_MOVE = !SHIFT_ONLY_ON_MOVE,
        Ok(KEY_K) => ENABLE_SHADOWS = !ENABLE_SHADOWS,
        _ => {}
    }
}

unsafe fn initialize(width: usize, height: usize) {
    MAP_WIDTH  = width;
    MAP_HEIGHT = height;
    
    NEIGHBOURS = vec![[false; 4]; MAP_WIDTH*MAP_HEIGHT];
    PATH       = vec![false; MAP_WIDTH*MAP_HEIGHT];
    VISITED    = vec![false; MAP_WIDTH*MAP_HEIGHT];
    
    PLAYER = 0;
    ORIGIN = MAP_WIDTH*MAP_HEIGHT-1;
    FINISH = MAP_WIDTH*MAP_HEIGHT-1;

    OPATH = Vec::new();
    PATH = vec![false; MAP_WIDTH*MAP_HEIGHT];
    VISITED = vec![false; MAP_WIDTH*MAP_HEIGHT];
    DVISITED = vec![false; MAP_WIDTH*MAP_HEIGHT];
    DISTANCES = vec![usize::MAX; MAP_WIDTH*MAP_HEIGHT];
}

unsafe fn load_maze_from_file(file_path: String){
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    MAP = content.lines().map(|line| line.chars().collect()).collect();

    initialize((MAP[0].len()-1)/2, (MAP.len()-1)/2);
    parse_neighbours();
    
}

unsafe fn parse_neighbours(){
    for h in 0..MAP_HEIGHT {
        for w in 0..MAP_WIDTH {
            let index = h * MAP_WIDTH + w;

            let maze_w = 2 * w + 1;
            let maze_h = 2 * h + 1;
            
            if maze_w - 1 > 0 && MAP[maze_h][maze_w-1] == '.' {
                NEIGHBOURS[index][0] = true;
            }
            if maze_h - 1 > 0 && MAP[maze_h-1][maze_w] == '.' {
                NEIGHBOURS[index][1] = true;
            }
            if maze_w + 1 < MAP_WIDTH*2+1 && MAP[maze_h][maze_w+1] == '.' {
                NEIGHBOURS[index][2] = true;
            }
            if maze_h + 1 > MAP_HEIGHT*2+1 && MAP[maze_h+1][maze_w] == '.' {
                NEIGHBOURS[index][3] = true;
            }
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
    PLAYER == FINISH
}

unsafe fn move_player(direction: usize){
    TOTAL_MOVES_MADE += 1;
    if !player_can_move(PLAYER, direction){
        if !SHIFT_ONLY_ON_MOVE && ENABLE_SHIFTS{
            shift_origin(SHIFTS);
        }
        return;
    }
    match direction {
        0 => PLAYER -= 1,
        1 => PLAYER -= MAP_WIDTH,
        2 => PLAYER += 1,
        3 => PLAYER += MAP_WIDTH,
        _ => {}
    }
    if ENABLE_SHIFTS {
        shift_origin(SHIFTS);
    }
    MOVES_MADE += 1;
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

unsafe fn print_options(){
    if ENABLE_DEBUG {
        println!("Shifts per move: {}", SHIFTS);
        println!("Shifts enabled: {}", ENABLE_SHIFTS);
        println!("Shift only on legal move: {}", SHIFT_ONLY_ON_MOVE);
        println!();
        println!("Press <Z> to toggle SHIFTING");
        println!("Press <X> to toggle SHIFTING only on legal move");
        println!("Press <I> to increase the SHIFTS per MOVE");
        println!("Press <U> to decrease the SHIFTS per MOVE");
        println!();
    }
    if ENABLE_INSTRUCTIONS {
        println!("Press <P> to toggle display of the path");
        println!("Press <O> to toggle display of the origin");
        println!("Press <N> to toggle display of node indexes");
        println!("Press <F> to toggle display of the finish node");
        println!("Press <K> to toggle display of shadows");
    }
}

unsafe fn visualize_board(){
    print!("\x1b[2J\x1b[1;1H");
    let WALL_COLOR   = "\x1b[41m";
    let RESET_SEQ    = "\x1b[0m";
    let PATH_COLOR   = "";
    let PLAYER_COLOR = "";
    let FINISH_COLOR = "";
    let ORIGIN_COLOR = "";
    let SHADOW_COLOR = "\x1b[48;5;240m";
    for y in 0..MAP_HEIGHT {
        if y == 0{
            if ENABLE_NODE_NUMBERS{
                for _ in  0..MAP_WIDTH*4+1{
                    print!("\x1b[41m \x1b[0m");
                }
            }else{
                for _ in  0..MAP_WIDTH*3+1{
                    print!("\x1b[41m \x1b[0m");
                }
            }
            print!("\n");
        }
        if NEIGHBOURS[y*MAP_WIDTH][0] {
            print!(" ");
        }else{
            print!("\x1b[41m \x1b[0m");
        }

        for x in 0..MAP_WIDTH {
            let idx: usize = y * MAP_WIDTH + x;
            let fmt_idx = if ENABLE_NODE_NUMBERS {format!("{:0>3}", idx)} else {format!("{: >2}", "")};

            if  
                ENABLE_SHADOWS && 
                DISTANCES[idx] > VIEW_DISTANCE
            {
                if x+1 == MAP_WIDTH {
                    print!("{SHADOW_COLOR}{fmt_idx}{RESET_SEQ}{WALL_COLOR} {RESET_SEQ}");
                    continue;
                }
                if can_move(idx, 2) && DISTANCES[idx+1] < VIEW_DISTANCE{
                    print!("{SHADOW_COLOR}{fmt_idx}{RESET_SEQ}{WALL_COLOR} {RESET_SEQ}");
                    continue;
                }
                print!("{SHADOW_COLOR}{fmt_idx} {RESET_SEQ}");
                continue;
            }

            let mut connection = "\x1b[41m \x1b[0m";
            if player_can_move(idx, 2) //NEIGHBOURS[idx][2] 
            {connection = " "}
            else if x < MAP_WIDTH-1 && player_can_move(idx+1, 0) //NEIGHBOURS[idx+1][0] 
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
            let mut spaces = format!("{: >1}", "");
            let mut connection = if ENABLE_NODE_NUMBERS {format!("\x1b[41m{: >3}\x1b[0m", "")} else {format!("\x1b[41m{: >2}\x1b[0m", "")}; //"\x1b[41m   \x1b[0m";

            if  
                ENABLE_SHADOWS && 
                DISTANCES[idx] > VIEW_DISTANCE && 
                !(can_move(idx, 3) && DISTANCES[idx+MAP_WIDTH] < VIEW_DISTANCE)
            {
                if y+1 == MAP_HEIGHT {
                    if ENABLE_NODE_NUMBERS {
                        print!("{WALL_COLOR}   {spaces}{RESET_SEQ}");
                    } else {
                        print!("{WALL_COLOR}  {spaces}{RESET_SEQ}");
                    }
                    continue;
                }
                if x+1 == MAP_WIDTH {
                    spaces = format!("{RESET_SEQ}{WALL_COLOR} ");
                }
                if can_move(idx, 2) && DISTANCES[idx+1] < VIEW_DISTANCE{
                    spaces = format!("{RESET_SEQ}{WALL_COLOR} ");
                }
                if can_move(idx, 2) && can_move(idx+1, 3) && DISTANCES[idx+1+MAP_WIDTH] < VIEW_DISTANCE{
                    spaces = format!("{RESET_SEQ}{WALL_COLOR} ");
                }
                if ENABLE_NODE_NUMBERS {
                    print!("{SHADOW_COLOR}   {spaces}{RESET_SEQ}");
                } else {
                    print!("{SHADOW_COLOR}  {spaces}{RESET_SEQ}");
                }
                continue;
            }

            if NEIGHBOURS[idx][3] 
            //{connection = format!("{: >3}", "");}
            {if ENABLE_NODE_NUMBERS {connection = format!("{: >3}", "");}else{connection = format!("{: >2}", "");}}
            else if y < MAP_HEIGHT-1 && NEIGHBOURS[idx+MAP_WIDTH][1] 
            {if ENABLE_NODE_NUMBERS {connection = format!("{: >3}", "");}else{connection = format!("{: >2}", "");}}
            if y < MAP_HEIGHT-1 && PATH[idx] && PATH[idx+MAP_WIDTH] && player_can_move(idx, 3) && ENABLE_PATH
            {if ENABLE_NODE_NUMBERS {connection = format!("\x1b[43m{: >3}\x1b[0m", "");} else {connection = format!("\x1b[43m{: >2}\x1b[0m", "");}}
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
