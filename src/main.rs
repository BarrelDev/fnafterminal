use std::collections::HashMap;

use rand::Rng;

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
        Animatronic {
            name,
            location,
            difficulty,
        }
    }
}

struct Map {
    grid: [[Option<Locations>; 9]; 5],
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

        match *location {
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

        let distance = ((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64;
        distance.sqrt() as u8
    }

    fn display_map(&self, states: &HashMap<&Locations, (&Animatronic, Tells)>) {
        let mut map: &str = "
                       [{ss}]
                        ||
              [{a}]==[{dal} {dac} {dar}]==[{k}]
               ||       ||       ||
              [{rr}-------{rr}]      ||
               ||                ||
              [{hl}]==[{sosl} {soa} {sosr}]==[{hr}]
        ";

// holy smokes this is a mess
// i will fix this later :D
//         let map: String = format!("        [{ss}]
//          ||
// [{a}]==[{dal} {dac} {dar}]==[{k}]
// ||       ||       ||
// [{rr}-------{rr}]      ||
// ||                ||
// [{hl}]==[{sosl} {soa} {sosr}]==[{hr}]", 
//             ss = {
//                 let state = states.get(&Locations::ShowStage);
//                 *state.unwrap().1.value()
//             },
//             a = {
//                 let state = states.get(&Locations::Arcade);
//                 *state.unwrap().1.value()
//             },
//             dal = {
//                 let state = states.get(&Locations::DiningAreaL);
//                 *state.unwrap().1.value()
//             },
//             dac = {
//                 let state = states.get(&Locations::DiningAreaC);
//                 *state.unwrap().1.value()
//             },
//             dar = {
//                 let state = states.get(&Locations::DiningAreaR);
//                 *state.unwrap().1.value()
//             },
//             k = {
//                 let state = states.get(&Locations::Kitchen);
//                 *state.unwrap().1.value()
//             },
//             rr = {
//                 let state = states.get(&Locations::Restrooms);
//                 *state.unwrap().1.value()
//             },
//             hl = {
//                 let state = states.get(&Locations::HallwayL);
//                 *state.unwrap().1.value()
//             },
//             hr = {  
//                 let state = states.get(&Locations::HallwayR);
//                 *state.unwrap().1.value()
//             },
//             sosl = {
//                 let state = states.get(&Locations::SecurityOfficeStaticL);
//                 *state.unwrap().1.value()
//             },
//             soa =  {
//                 let state = states.get(&Locations::SecurityOfficeAttack);
//                 *state.unwrap().1.value()
//             },
//             sosr = {
//                 let state = states.get(&Locations::SecurityOfficeStaticR);
//                 *state.unwrap().1.value()
//             },
//         );
        println!("{map}");
    }

}

fn main() {
    const START_TIME: u16 = 0;
    const END_TIME: u16 = 6 * 60;
    const TICK_RATE: u16 = 15; // 15 minutes at a time

    let mut time: u16 = START_TIME;

    let mut battery = Battery::new();

    let mut animatronics: Vec<Animatronic> = Vec::new();

    let mut states: HashMap<&Locations, (&Animatronic, Tells)> = HashMap::new();

    let map = Map::new();

    loop {
        let (hours, minutes) = display_time(time);
        println!("Time: {:02}:{:02}", hours, minutes);

        battery.update_power();

        if battery.power == 0 {
            println!("You ran out of power! Game over!");
            break;
        }

        if time == END_TIME {
            println!("You survived the night! Congratulations!");
            break;
        }

        time += TICK_RATE;
    }

    map.display_map(&states);

    println!("Hello, world!");
}

fn display_time(time: u16) -> (u16, u16) {
    let hours = time / 60;
    let minutes = time % 60;
    (hours, minutes)
}