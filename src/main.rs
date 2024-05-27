use std::{collections::HashMap, panic::Location};

use rand::Rng;

use std::cmp;



#[derive(PartialEq, Hash, Clone, Copy, Eq)]
enum Locations {
    ShowStage,
    DiningAreaL,
    DiningAreaR,
    DiningAreaC,
    Restrooms,
    Kitchen,
    Arcade,
    SecurityOfficeStaticR,
    SecurityOfficeStaticL,
    SecurityOfficeAttack,
    HallwayL,
    HallwayR,
}

enum Tells {
    Laughing,
    Noise,
    Footsteps,
    Static,
    Visual,
}

impl Tells {
    fn value(&self) -> &str {
        match self {
            Tells::Laughing => "L",
            Tells::Noise => "N",
            Tells::Footsteps => "F",
            Tells::Static => "S",
            Tells::Visual => "V",
        }
    }
}

#[derive(Clone, Copy)]
enum PowerDraw {
    Camera,
    Lights,
    Doors,
}

impl PowerDraw {
    fn value(&self) -> u8 {
        match self {
            PowerDraw::Camera => 1,
            PowerDraw::Lights => 2,
            PowerDraw::Doors => 3,
        }
    }
}

impl PartialEq for PowerDraw {
    fn eq(&self, other: &Self) -> bool {
        *self as u8 == *other as u8
    }
}

struct Battery {
    power: u8,
    power_draw: Vec<PowerDraw>,
}

impl Battery {
    fn new() -> Battery {
        Battery {
            power: 100,
            power_draw: Vec::new(),
        }
    }

    fn add_power_draw(&mut self, power_draw: PowerDraw) {
        let index = self.power_draw.iter().position(|&x| x == power_draw);
        match index {
            Some(_) => {}
            None => {
                self.power_draw.push(power_draw);
            }
        }
    }

    fn remove_power_draw(&mut self, power_draw: PowerDraw) {
        let index = self.power_draw.iter().position(|&x| x == power_draw);
        match index {
            Some(i) => {
                self.power_draw.remove(i);
            }
            None => {}
        }
    }

    fn update_power(&mut self) {
        let mut rng = rand::thread_rng();
        for (i, &draw) in self.power_draw.iter().enumerate() {
            let random_tick: u8 = rng.gen_range(1..20);
            if draw as u8 == random_tick {
                if self.power > 0 {
                    self.power -= draw.value();
                } else {
                    self.power = 0;
                }
            }
        }
    }
}

struct Animatronic {
    name: String,
    location: Locations,
    difficulty: u8,
}

impl Animatronic {
    fn new(name: String, location: Locations, difficulty: u8) -> Animatronic {
        let clamped = cmp::min::<u8>(difficulty, 20);
        
        Animatronic {
            name,
            location,
            difficulty: clamped,
        }
    }

    fn move_tick(&mut self, map: &Map) {
        // move the animatronic
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..20);
        
        if random_index <= self.difficulty {
            let adjacent_rooms = map.find_adjacent_room(&self.location);
            let locations = adjacent_rooms.iter().map(|(x, y)| map.grid[*x as usize][*y as usize]);
            let mut closest: Locations = Locations::ShowStage;

            for (i, loc) in locations.enumerate() {
                match loc {
                    Some(l) => {
                        let dist = map.distance_from_office_attack(&loc.unwrap());
                        let closest_dist = map.distance_from_office_attack(&closest);
                        if dist < closest_dist {
                            if loc == Some(Locations::SecurityOfficeStaticL) && !map.left_door_closed {
                                closest = loc.unwrap();
                            } else if loc == Some(Locations::SecurityOfficeStaticR) && !map.right_door_closed {
                                closest = loc.unwrap();
                            } else {
                                closest = loc.unwrap();
                            }
                        }
                    }
                    None => {}
                }
            }

            self.location = closest;
        }
    }
}

struct Map {
    grid: [[Option<Locations>; 9]; 5],
    left_door_closed: bool,
    right_door_closed: bool,
}

impl Map {
    fn new() -> Map {
        Map {
            grid: [
                [None, None, None, Some(Locations::ShowStage), Some(Locations::ShowStage), Some(Locations::ShowStage), None, None, None],
                [None, None, Some(Locations::Arcade), Some(Locations::DiningAreaL), Some(Locations::DiningAreaC), Some(Locations::DiningAreaR), Some(Locations::Kitchen), None, None],
                [None, None, Some(Locations::Restrooms), Some(Locations::Restrooms), Some(Locations::Restrooms), Some(Locations::Restrooms), Some(Locations::HallwayR), None, None],
                [None, None, Some(Locations::HallwayL), None, None, None, Some(Locations::HallwayR), None, None],
                [None, None, Some(Locations::HallwayL), Some(Locations::SecurityOfficeStaticR), Some(Locations::SecurityOfficeAttack), Some(Locations::SecurityOfficeStaticR), Some(Locations::HallwayR), None, None],
            ],
            left_door_closed: false,
            right_door_closed: false,
        }
    }

    fn find_adjacent_room(&self, location: &Locations) -> Vec<(u8, u8)> {
        let mut ret: Vec<(u8, u8)> = Vec::new();

        let (x, y) = self.find_location(location);

        // lol made by a map that i drew up
        // why do things automatically when you can hard code them!

        /*
          Map btw
                                                        [Show Stage]
            [Arcade]   <===  [Dining Area L]    <===   [Dining Area C]    ===>   [Dining Area R]  ===>  [Kitchen]
            [Restrooms]----------------------------------[Restrooms]                                   [Hallway R] 
            [Hallway L]                                                                                [Hallway R]
            [Hallway L] [Security Office Static R] [Security Office Attack] [Security Office Static R] [Hallway R]
         */

        match location {
            Locations::HallwayL => {
                ret.push(self.find_location(&Locations::Restrooms));
                ret.push(self.find_location(&Locations::SecurityOfficeStaticL));
            },
            Locations::HallwayR => {
                ret.push(self.find_location(&Locations::Kitchen));
                ret.push(self.find_location(&Locations::SecurityOfficeStaticR));
            },
            Locations::ShowStage => {
                ret.push(self.find_location(&Locations::DiningAreaC));
            },
            Locations::DiningAreaL => {
                ret.push(self.find_location(&Locations::Arcade));
                ret.push(self.find_location(&Locations::DiningAreaC));
            },
            Locations::DiningAreaC => {
                ret.push(self.find_location(&Locations::DiningAreaL));
                ret.push(self.find_location(&Locations::DiningAreaR));
                ret.push(self.find_location(&Locations::ShowStage));
                ret.push(self.find_location(&Locations::Restrooms));
            },
            Locations::DiningAreaR => {
                ret.push(self.find_location(&Locations::DiningAreaC));
                ret.push(self.find_location(&Locations::Kitchen));
            },
            Locations::Restrooms => {
                ret.push(self.find_location(&Locations::DiningAreaC));
                ret.push(self.find_location(&Locations::HallwayL));
                ret.push(self.find_location(&Locations::Arcade));
            },
            Locations::Kitchen => {
                ret.push(self.find_location(&Locations::DiningAreaR));
                ret.push(self.find_location(&Locations::HallwayR));
            },
            Locations::Arcade => {
                ret.push(self.find_location(&Locations::Restrooms));
                ret.push(self.find_location(&Locations::DiningAreaL));
                ret.push(self.find_location(&Locations::HallwayL));
            },
            Locations::SecurityOfficeStaticR => { 
                ret.push(self.find_location(&Locations::SecurityOfficeAttack)); 
            },
            Locations::SecurityOfficeStaticL => { 
                ret.push(self.find_location(&Locations::SecurityOfficeAttack)); 
            },
            Locations::SecurityOfficeAttack => { 
                ret.push(self.find_location(location)); 
            },
        }

        ret
    }

    fn find_location(&self, location: &Locations) -> (u8, u8) {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, loc) in row.iter().enumerate() {
                match loc {
                    Some(l) => {
                        if *l == *location {
                            return (i as u8, j as u8);
                        }
                    }
                    None => {}
                }
            }
        }
        (0, 0)
    }

    fn distance_from_office_attack(&self, location: &Locations) -> u8 {
        let office_attack = &Locations::SecurityOfficeAttack;
        let (x1, y1) = self.find_location(office_attack);
        let (x2, y2) = self.find_location(location);

        let distance = ((x1 as f64 - x2 as f64).powf(2.0) + (y1 as f64 - y2 as f64).powf(2.0)) as f64;
        distance.sqrt() as u8
    }

    fn display_map(&self, states: &HashMap<Locations, (u8, Tells)>) {
        let mut map: String = String::from("
        [{ss}]
        | |
[{a}]==[{dal}--{dac}--{dar}]==[{k}]
| |     | |     | |
[{rr}-------{rr}]     | |
| |             | |
[{hl}]==[{sosl}--{soa}--{sosr}]==[{hr}]
");

        /*
                      [{ss}]
                      | |
              [{a}]==[{dal}--{dac}--{dar}]==[{k}]
              | |     | |     | |
              [{rr}-------{rr}}]     | |
              | |             | |
              [{hl}]==[{sosl}--{soa}--{sosr}]==[{hr}]
        "
        
         */

        let mut map = map.replace(
            "{ss}",
            match states.get(&Locations::ShowStage) {
                Some((_, t)) => t.value(),
                None => " ",
            },
        );

        let mut map = map.replace(
            "{a}",
            match states.get(&Locations::Arcade) {
                Some((_, t)) => t.value(),
                None => " ",
            },
        );

        let mut map = map.replace(
            "{dal}",
            match states.get(&Locations::DiningAreaL) {
                Some((_, t)) => t.value(),
                None => " ",
            },
        );

        let mut map = map.replace(
            "{dac}",
            match states.get(&Locations::DiningAreaC) {
                Some((_, t)) => t.value(),
                None => " ",
            },
        );

        let mut map = map.replace(
            "{dar}",
            match states.get(&Locations::DiningAreaR) {
                Some((_, t)) => t.value(),
                None => " ",
            },
        );

        let mut map = map.replace(
            "{k}",
            match states.get(&Locations::Kitchen) {
                Some((_, t)) => t.value(),
                None => " ",
            },
        );

        let mut map = map.replace(
            "{rr}",
            match states.get(&Locations::Restrooms) {
                Some((_, t)) => t.value(),
                None => " ",
            },
        );

        let mut map = map.replace(
            "{hl}",
            match states.get(&Locations::HallwayL) {
                Some((_, t)) => t.value(),
                None => " ",
            },
        );

        let mut map = map.replace(
            "{hr}",
            match states.get(&Locations::HallwayR) {
                Some((_, t)) => t.value(),
                None => " ",
            },
        );

        let mut map = map.replace(
            "{sosl}",
            match states.get(&Locations::SecurityOfficeStaticL) {
                Some((_, t)) => t.value(),
                None => " ",
            },
        );

        let mut map = map.replace(
            "{soa}",
            match states.get(&Locations::SecurityOfficeAttack) {
                Some((_, t)) => t.value(),
                None => " ",
            },
        );

        let map = map.replace(
            "{sosr}",
            match states.get(&Locations::SecurityOfficeStaticR) {
                Some((_, t)) => t.value(),
                None => " ",
            },
        );
        
        println!("{map}");
    }

}

fn main() {
    const START_TIME: u32 = 0;
    const END_TIME: u32 = 6 * 60;
    const TICK_RATE: u32 = 15; // 15 minutes at a time

    let mut time: u32 = START_TIME;

    let mut battery = Battery::new();

    let mut animatronics: Vec<Animatronic> = Vec::new();

    let freddy = Animatronic::new("Freddy".to_string(), Locations::ShowStage, 13);
    let bonnie = Animatronic::new("Bonnie".to_string(), Locations::ShowStage, 15);
    let chica = Animatronic::new("Chica".to_string(), Locations::ShowStage, 19);
    animatronics.push(freddy);
    animatronics.push(bonnie);
    animatronics.push(chica);

    let mut states: HashMap<Locations, (u8, Tells)> = HashMap::new();

    states.insert(Locations::ShowStage, (0, Tells::Visual));
    states.insert(Locations::ShowStage, (1, Tells::Visual));
    states.insert(Locations::ShowStage, (2, Tells::Visual));


    let map = Map::new();


    loop {
        let (hours, minutes) = display_time(time);
        println!("Time: {:02}:{:02}", hours, minutes);

        battery.update_power();

        for (i, anim) in animatronics.iter_mut().enumerate() {
            let curr_loc: Locations = anim.location;
            
            anim.move_tick(&map);

            let new_loc: Locations = anim.location;

            if curr_loc != new_loc {
                if let Some(state) = states.remove(&curr_loc) {
                    states.insert(new_loc, (state.0, Tells::Visual));
                }
            }
        }

        map.display_map(&states);

        if battery.power == 0 {
            println!("You ran out of power! Game over!");
            break;
        }

        if time >= END_TIME {
            println!("You survived the night! Congratulations!");
            break;
        }

        time += TICK_RATE;
    }

    println!("Hello, world!");
}

fn display_time(time: u32) -> (u32, u32) {
    let hours = time / 60;
    let minutes = time % 60;
    (hours, minutes)
}