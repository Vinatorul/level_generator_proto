use tile_engine::{TileEngine, TileRect};
use rand::{SeedableRng, StdRng, Rng};
use std::collections::VecDeque;
use std::cmp::{min, max};

pub type Room = [u32; 4];

// Max recursion deep
const DN_MAX_DEEP: u32 = 5;
// First recursion level with non 100% split chance
const DN_ALLOW_FULL_FROM: u32 = 3;
// Split chance dec per recursion level starting from DN_ALLOW_FULL_FROM
const DN_SPLIT_CHANCE_DEC: u32 = 30;
// Split coeff in percents
const DN_SPLIT_COEFF: u32 = 70;
// Default split chance
const DN_DEFAULT_CHANCE: u32 = 100;
// Min size splittable
const DN_MIN_SP_SIZE: u32 = 150;
// Coridor width
const DN_CORIDOR_WIDTH: u32 = 15;

#[derive(Default, Clone, Copy, Debug)]
struct SubDungeon {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub deep: u32,
    pub childs: Option<[usize; 2]>,
    pub parent: Option<usize>,
    pub bounding_rect: Option<Room>,
}

impl SubDungeon {
    pub fn new(x: u32, y: u32, w: u32, h: u32, deep: u32, parent: Option<usize>) -> SubDungeon {
        SubDungeon {
            x: x,
            y: y,
            width: w,
            height: h,
            deep: deep,
            childs: None,
            parent: parent,
            bounding_rect: None,
        }
    }
}

fn generate_coridor_vert(tile_engine: &mut TileEngine, bx1: Room, bx2: Room, rng: &mut StdRng) {
    let max_min_x = max(bx1[0], bx2[0]);
    let min_max_x = min(bx1[0]+bx1[2], bx2[0]+bx2[2])-DN_CORIDOR_WIDTH;
    let mut coridor = [0,0,DN_CORIDOR_WIDTH,0];
    coridor[1] = bx1[1] + bx1[3];
    coridor[3] = bx2[1] - bx1[1] - bx1[3];
    if max_min_x < min_max_x {
        // I like coridor
        coridor[0] = rng.gen_range(max_min_x, min_max_x);
    }
    else {
        // L like coridor
        coridor[0] = rng.gen_range(bx1[0], bx1[0]+bx1[2]);
        coridor[3] += bx2[3]/2;
        let mut h_coridor = [0,coridor[1]+coridor[3]-DN_CORIDOR_WIDTH,0,DN_CORIDOR_WIDTH];
        if max_min_x == bx1[0] {
            h_coridor[0] = min_max_x;
            h_coridor[2] = coridor[0] - h_coridor[0];
        }
        else {
            h_coridor[0] = coridor[0];
            h_coridor[2] = max_min_x - coridor[0];
        }
        tile_engine.add_tile(h_coridor[0] as f64, h_coridor[1] as f64, h_coridor[2] as i32, h_coridor[3] as i32, 3);
    }
    tile_engine.add_tile(coridor[0] as f64, coridor[1] as f64, coridor[2] as i32, coridor[3] as i32, 3);
}

fn generate_coridor_hor(tile_engine: &mut TileEngine, bx1: Room, bx2: Room, rng: &mut StdRng) {
    let max_min_y = max(bx1[1], bx2[1]);
    let min_max_y = min(bx1[1]+bx1[3], bx2[1]+bx2[3])-DN_CORIDOR_WIDTH;
    let mut coridor = [0,0,0,DN_CORIDOR_WIDTH];
    coridor[0] = bx1[0] + bx1[2];
    coridor[2] = bx2[0] - bx1[0] - bx1[2];
    if max_min_y < min_max_y {
        // - like coridor
        coridor[1] = rng.gen_range(max_min_y, min_max_y);
    }
    else {
        // L like coridor
        coridor[1] = rng.gen_range(bx1[1], bx1[1]+bx1[3]);
        coridor[2] += bx2[2]/2;
        let mut v_coridor = [coridor[0]+coridor[2]-DN_CORIDOR_WIDTH,0,DN_CORIDOR_WIDTH,0];
        if max_min_y == bx1[1] {
            v_coridor[1] = min_max_y;
            v_coridor[3] = coridor[1] - v_coridor[1];
        }
        else {
            v_coridor[1] = coridor[1];
            v_coridor[3] = max_min_y - coridor[1];
        }
        tile_engine.add_tile(v_coridor[0] as f64, v_coridor[1] as f64, v_coridor[2] as i32, v_coridor[3] as i32, 3);
    }
    tile_engine.add_tile(coridor[0] as f64, coridor[1] as f64, coridor[2] as i32, coridor[3] as i32, 3);
}

pub fn generate_level(tile_engine: &mut TileEngine, seed: &[usize], w: u32, h: u32) -> Vec<Room> {
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut dungeon = Vec::<SubDungeon>::new();
    dungeon.push(SubDungeon::new(0, 0, w, h, 0, None));
    let mut queue = VecDeque::<usize>::new();
    queue.push_back(0);
    // generating subdungeons
    while !queue.is_empty() {
        // current index in queue
        let i = queue.pop_front().unwrap();
        // checking now deep we are
        let deep = dungeon[i].deep;
        // counting split chance
        let mut sp = DN_DEFAULT_CHANCE;
        if deep >= DN_ALLOW_FULL_FROM {
            if deep > DN_MAX_DEEP {
                break;
            }
            sp = sp - (deep - DN_ALLOW_FULL_FROM + 1)*DN_SPLIT_CHANCE_DEC;
        }
        let w = dungeon[i].width;
        let h = dungeon[i].height;
        // will it be splited, or not?
        let b = ((w > DN_MIN_SP_SIZE) || (h > DN_MIN_SP_SIZE)) && (rng.gen_range(0, 100) < sp);
        if b {
            // How fair will be split
            let coef = rng.gen_range(100 - DN_SPLIT_COEFF, DN_SPLIT_COEFF);
            let x1 = dungeon[i].x;
            let y1 = dungeon[i].y;
            let mut x2 = x1;
            let mut y2 = y1;
            let mut w1 = w;
            let mut h1 = h;
            // Split vertical or not
            if (h < DN_MIN_SP_SIZE) || ((w > DN_MIN_SP_SIZE) && (rng.gen_range(0, 2) == 1)) {
                x2 = x1 + w*coef/100;
                w1 = x2 - x1;
            }
            else {
                y2 = y1 + h*coef/100;
                h1 = y2 - y1;
            }
            dungeon.push(SubDungeon::new(x1, y1, w1, h1, deep+1, Some(i)));
            dungeon.push(SubDungeon::new(x2, y2, w-(x2-x1), h-(y2-y1), deep+1, Some(i)));
            dungeon[i].childs = Some([dungeon.len()-2, dungeon.len()-1]);
            queue.push_back(dungeon.len()-2);
            queue.push_back(dungeon.len()-1);
        }
    }
    // generation rooms
    let mut rooms = Vec::<Room>::new();
    queue.clear();
    for i in 0..dungeon.len() {
        //let has_childs = dungeon[i].childs.is_some();
        if dungeon[i].childs.is_none() {
            let parent = {
                let d = dungeon[i];
                tile_engine.add_tile(d.x as f64, d.y as f64, d.width as i32, d.height as i32, 1);
                // generation rooms
                let mut room = [0,0,0,0];
                room[2] = rng.gen_range(15*d.width/20, 18*d.width/20);
                room[3] = rng.gen_range(15*d.height/20, 18*d.height/20);
                room[0] = rng.gen_range(d.x + d.width/20, d.x + d.width - room[2] - d.width/20);
                room[1] = rng.gen_range(d.y + d.height/20, d.y + d.height - room[3] - d.height/20);
                tile_engine.add_tile(room[0] as f64, room[1] as f64, room[2] as i32, room[3] as i32, 2);
                rooms.push(room);
                dungeon[i].bounding_rect = Some(room.clone());
                dungeon[i].parent.unwrap()
            };
            // push parent to generate corridor
            let childs = dungeon[parent].childs.unwrap();
            //println!("{:?}", dungeon[childs[0]]);
            //println!("{:?}", dungeon[childs[1]]);
            if (dungeon[childs[0]].bounding_rect.is_some()) && (dungeon[childs[1]].bounding_rect.is_some()) {
                queue.push_back(parent);
            }
        }
    }
    // generating coridors
    while !queue.is_empty() {
        let i = queue.pop_front().unwrap();
        let p = dungeon[i];
        //println!("{:?}", p);
        let childs = p.childs.unwrap();
        let (ch1, ch2) = (dungeon[childs[0]], dungeon[childs[1]]);
        //println!("{:?}", ch1);
        //println!("{:?}", ch2);
        let (bx1, bx2) = (ch1.bounding_rect.unwrap(), ch2.bounding_rect.unwrap());
        if ch1.x == ch2.x {
            generate_coridor_vert(tile_engine, bx1, bx2, &mut rng);
        }
        else {
            generate_coridor_hor(tile_engine, bx1, bx2, &mut rng);
        }
        // generation bounding box for parent subdungeon
        dungeon[i].bounding_rect = {
            if rng.gen_range(0, 2) == 1 {
                Some(bx1.clone())
            }
            else {
                Some(bx2.clone())
            }
        };
        // if both childs have bouinding rect => add parent to queue
        if let Some(pp) = dungeon[i].parent {
            let childs = dungeon[pp].childs.unwrap();
            if (dungeon[childs[0]].bounding_rect.is_some()) && (dungeon[childs[1]].bounding_rect.is_some()) {
                queue.push_back(pp);
            }
        }
    }
    //print!("{:?}", dungeon);
    //print!("{:?}", rooms);
    rooms
}
