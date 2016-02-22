use tile_engine::{TileEngine, TileRect};
use rand::{SeedableRng, StdRng, Rng};
use std::collections::VecDeque;

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

#[derive(Default, Clone, Copy, Debug)]
struct SubDungeon {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub deep: u32,
    pub childs: Option<[usize; 2]>,
    pub parent: Option<usize>,
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
        }
    }
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
            if (w > DN_MIN_SP_SIZE) && (rng.gen_range(0, 2) == 1) {
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
    let mut rooms = Vec::<Room>::new();
    queue.clear();
    for d in dungeon.iter() {
        if d.childs.is_none() {
            tile_engine.add_tile(d.x as f64, d.y as f64, d.width as i32, d.height as i32, 1);
            // generation rooms
            let room_w = rng.gen_range(d.width/2, 4*d.width/5 - d.width/20);
            let room_h = rng.gen_range(d.height/2, 4*d.height/5 - d.height/20);
            let room_x = rng.gen_range(d.x + d.width/20, d.x + d.width - room_w - d.width/20);
            let room_y = rng.gen_range(d.y + d.height/20, d.y + d.height - room_h - d.height/20);
            tile_engine.add_tile(room_x as f64, room_y as f64, room_w as i32, room_h as i32, 2);
            rooms.push([room_x, room_y, room_w, room_h]);
            // push parent to generate corridor
            queue.push_back(d.parent.unwrap());
        }
    }
    while !queue.is_empty() {
        let p = dungeon[queue.pop_front().unwrap()];
        if let Some(pp) = p.parent {
            queue.push_back(pp);
        }
        let (ch1, ch2) = (dungeon[p.childs.unwrap()[0]], dungeon[p.childs.unwrap()[1]]);
        let mut room = [0,0,0,0];
        if ch1.x != ch2.x {
            room[0] = ch1.x + ch1.width/2;
            room[1] = ((ch1.y*2 + ch1.height)/2 + (ch2.y*2 + ch2.height)/2)/2;
            room[2] = ch2.x - ch1.x + ch2.width/2 - ch1.width/2;
            room[3] = 10;
        }
        else {
            room[0] = ((ch1.x*2 + ch1.width)/2 + (ch2.x*2 + ch2.width)/2)/2;
            room[1] = ch1.y + ch1.height/2;
            room[2] = 10;
            room[3] = ch2.y - ch1.y + ch2.height/2 - ch1.height/2;
        }
        tile_engine.add_tile(room[0] as f64, room[1] as f64, room[2] as i32, room[3] as i32, 3);
    }
    //print!("{:?}", dungeon);
    //print!("{:?}", rooms);
    rooms
}
