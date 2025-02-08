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
            PowerDraw::Camera => 2,
            PowerDraw::Lights => 4,
            PowerDraw::Doors => 7,
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
        for (_i, &draw) in self.power_draw.iter().enumerate() {
            let random_tick: u8 = rng.gen_range(1..10);
            if draw as u8 <= random_tick {
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
    current_tell: Tells,
}

impl Animatronic {
    fn new(name: String, location: Locations, difficulty: u8) -> Animatronic {
        let clamped = cmp::min::<u8>(difficulty, 20);

        Animatronic {
            name,
            location,
            difficulty: clamped,
            current_tell: Tells::Visual,
        }
    }

    fn find_adjacent_room(&mut self) -> Vec<Locations> {
        let mut ret: Vec<Locations> = Vec::new();

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

        match self.location {
            Locations::HallwayL => {
                ret.push(Locations::Restrooms);
                ret.push(Locations::SecurityOfficeStaticL);
            }
            Locations::HallwayR => {
                ret.push(Locations::Kitchen);
                ret.push(Locations::SecurityOfficeStaticR);
            }
            Locations::ShowStage => {
                ret.push(Locations::DiningAreaC);
            }
            Locations::DiningAreaL => {
                ret.push(Locations::Arcade);
                ret.push(Locations::DiningAreaC);
            }
            Locations::DiningAreaC => {
                ret.push(Locations::DiningAreaL);
                ret.push(Locations::DiningAreaR);
                ret.push(Locations::ShowStage);
                ret.push(Locations::Restrooms);
            }
            Locations::DiningAreaR => {
                ret.push(Locations::DiningAreaC);
                ret.push(Locations::Kitchen);
            }
            Locations::Restrooms => {
                ret.push(Locations::DiningAreaC);
                ret.push(Locations::HallwayL);
                ret.push(Locations::Arcade);
            }
            Locations::Kitchen => {
                ret.push(Locations::DiningAreaR);
                ret.push(Locations::HallwayR);
            }
            Locations::Arcade => {
                ret.push(Locations::Restrooms);
                ret.push(Locations::DiningAreaL);
                ret.push(Locations::HallwayL);
            }
            Locations::SecurityOfficeStaticR => {
                ret.push(Locations::SecurityOfficeAttack);
            }
            Locations::SecurityOfficeStaticL => {
                ret.push(Locations::SecurityOfficeAttack);
            }
            Locations::SecurityOfficeAttack => {
                ret.push(self.location);
            }
        }

        ret
    }

    fn move_tick(
        &mut self,
        adjacent_loc: Vec<Locations>,
        right_door_closed: bool,
        left_door_closed: bool,
    ) {
        // move the animatronic
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..20);

        if random_index <= self.difficulty {
            let adjacent_rooms = adjacent_loc;
            // let mut closest: Locations = Locations::ShowStage;

            let random_index = rng.gen_range(0..adjacent_rooms.len());

            // Check to see if the animatronic is trying to move to the security office
            // if it is, check if the door is closed
            // if it is, don't move there
            if adjacent_rooms[random_index] == Locations::SecurityOfficeStaticR && right_door_closed
            {
                return;
            }

            if adjacent_rooms[random_index] == Locations::SecurityOfficeStaticL && left_door_closed
            {
                return;
            }

            self.location = adjacent_rooms[random_index];

            // pathfinding is stupid random numbers are good!
            // for (i, loc) in locations.enumerate() {
            //     match loc {
            //         Some(l) => {
            //             let dist = map.distance_from_office_attack(&loc.unwrap());
            //             let closest_dist = map.distance_from_office_attack(&closest);
            //             if dist < closest_dist {
            //                 if loc == Some(Locations::SecurityOfficeStaticL) && !map.left_door_closed {
            //                     closest = loc.unwrap();
            //                 } else if loc == Some(Locations::SecurityOfficeStaticR) && !map.right_door_closed {
            //                     closest = loc.unwrap();
            //                 } else {
            //                     closest = loc.unwrap();
            //                 }
            //             }
            //         }
            //         None => {}
            //     }
            // }

            // self.location = closest;
        }
    }
}

struct Map {
    grid: [[Option<Locations>; 9]; 5],
    left_door_closed: bool,
    right_door_closed: bool,
    anim_states: Vec<Animatronic>,
    is_dead: bool,
    killer: String,
}

impl Map {
    fn new() -> Map {
        Map {
            grid: [
                [
                    None,
                    None,
                    None,
                    Some(Locations::ShowStage),
                    Some(Locations::ShowStage),
                    Some(Locations::ShowStage),
                    None,
                    None,
                    None,
                ],
                [
                    None,
                    None,
                    Some(Locations::Arcade),
                    Some(Locations::DiningAreaL),
                    Some(Locations::DiningAreaC),
                    Some(Locations::DiningAreaR),
                    Some(Locations::Kitchen),
                    None,
                    None,
                ],
                [
                    None,
                    None,
                    Some(Locations::Restrooms),
                    Some(Locations::Restrooms),
                    Some(Locations::Restrooms),
                    Some(Locations::Restrooms),
                    Some(Locations::HallwayR),
                    None,
                    None,
                ],
                [
                    None,
                    None,
                    Some(Locations::HallwayL),
                    None,
                    None,
                    None,
                    Some(Locations::HallwayR),
                    None,
                    None,
                ],
                [
                    None,
                    None,
                    Some(Locations::HallwayL),
                    Some(Locations::SecurityOfficeStaticR),
                    Some(Locations::SecurityOfficeAttack),
                    Some(Locations::SecurityOfficeStaticR),
                    Some(Locations::HallwayR),
                    None,
                    None,
                ],
            ],
            left_door_closed: false,
            right_door_closed: false,
            anim_states: Vec::new(),
            is_dead: false,
            killer: String::from("MissingNo."),
        }
    }

    fn map_tick(&mut self) {
        for (_i, anim) in self.anim_states.iter_mut().enumerate() {
            let locations = anim.find_adjacent_room();
            anim.move_tick(locations, self.right_door_closed, self.left_door_closed);

            if anim.location == Locations::SecurityOfficeAttack {
                self.is_dead = true;
                self.killer = anim.name.clone();
            }
        }
    }

    fn find_adjacent_room(&mut self, location: Locations) -> Vec<Locations> {
        let mut ret: Vec<Locations> = Vec::new();

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
                ret.push(Locations::Restrooms);
                ret.push(Locations::SecurityOfficeStaticL);
            }
            Locations::HallwayR => {
                ret.push(Locations::Kitchen);
                ret.push(Locations::SecurityOfficeStaticR);
            }
            Locations::ShowStage => {
                ret.push(Locations::DiningAreaC);
            }
            Locations::DiningAreaL => {
                ret.push(Locations::Arcade);
                ret.push(Locations::DiningAreaC);
            }
            Locations::DiningAreaC => {
                ret.push(Locations::DiningAreaL);
                ret.push(Locations::DiningAreaR);
                ret.push(Locations::ShowStage);
                ret.push(Locations::Restrooms);
            }
            Locations::DiningAreaR => {
                ret.push(Locations::DiningAreaC);
                ret.push(Locations::Kitchen);
            }
            Locations::Restrooms => {
                ret.push(Locations::DiningAreaC);
                ret.push(Locations::HallwayL);
                ret.push(Locations::Arcade);
            }
            Locations::Kitchen => {
                ret.push(Locations::DiningAreaR);
                ret.push(Locations::HallwayR);
            }
            Locations::Arcade => {
                ret.push(Locations::Restrooms);
                ret.push(Locations::DiningAreaL);
                ret.push(Locations::HallwayL);
            }
            Locations::SecurityOfficeStaticR => {
                ret.push(Locations::SecurityOfficeAttack);
            }
            Locations::SecurityOfficeStaticL => {
                ret.push(Locations::SecurityOfficeAttack);
            }
            Locations::SecurityOfficeAttack => {
                ret.push(location);
            }
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

        let distance =
            ((x1 as f64 - x2 as f64).powf(2.0) + (y1 as f64 - y2 as f64).powf(2.0)) as f64;
        distance.sqrt() as u8
    }

    fn display_map(&self) {
        let mut map: String = String::from(
            "
        [{ss}]
        | |
[{a}]==[{dal}--{dac}--{dar}]==[{k}]
| |     | |     | |
[{rr}-------{rr}]     | |
| |             | |
[{hl}]==[{sosl}--{soa}--{sosr}]==[{hr}]
",
        );

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

        for (_i, anim) in self.anim_states.iter().enumerate() {
            match &anim.location {
                Locations::ShowStage => {
                    map = map.replace(
                        "{ss}",
                        format!(
                            "{}{}",
                            anim.name.chars().next().unwrap(),
                            anim.current_tell.value()
                        )
                        .as_str(),
                    );
                }
                Locations::Arcade => {
                    map = map.replace(
                        "{a}",
                        format!(
                            "{}{}",
                            anim.name.chars().next().unwrap(),
                            anim.current_tell.value()
                        )
                        .as_str(),
                    );
                }
                Locations::DiningAreaL => {
                    map = map.replace(
                        "{dal}",
                        format!(
                            "{}{}",
                            anim.name.chars().next().unwrap(),
                            anim.current_tell.value()
                        )
                        .as_str(),
                    );
                }
                Locations::DiningAreaC => {
                    map = map.replace(
                        "{dac}",
                        format!(
                            "{}{}",
                            anim.name.chars().next().unwrap(),
                            anim.current_tell.value()
                        )
                        .as_str(),
                    );
                }
                Locations::DiningAreaR => {
                    map = map.replace(
                        "{dar}",
                        format!(
                            "{}{}",
                            anim.name.chars().next().unwrap(),
                            anim.current_tell.value()
                        )
                        .as_str(),
                    );
                }
                Locations::Kitchen => {
                    map = map.replace(
                        "{k}",
                        format!(
                            "{}{}",
                            anim.name.chars().next().unwrap(),
                            anim.current_tell.value()
                        )
                        .as_str(),
                    );
                }
                Locations::Restrooms => {
                    map = map.replace(
                        "{rr}",
                        format!(
                            "{}{}",
                            anim.name.chars().next().unwrap(),
                            anim.current_tell.value()
                        )
                        .as_str(),
                    );
                }
                Locations::HallwayL => {
                    map = map.replace(
                        "{hl}",
                        format!(
                            "{}{}",
                            anim.name.chars().next().unwrap(),
                            anim.current_tell.value()
                        )
                        .as_str(),
                    );
                }
                Locations::HallwayR => {
                    map = map.replace(
                        "{hr}",
                        format!(
                            "{}{}",
                            anim.name.chars().next().unwrap(),
                            anim.current_tell.value()
                        )
                        .as_str(),
                    );
                }
                Locations::SecurityOfficeStaticL => {
                    map = map.replace(
                        "{sosl}",
                        format!(
                            "{}{}",
                            anim.name.chars().next().unwrap(),
                            anim.current_tell.value()
                        )
                        .as_str(),
                    );
                }
                Locations::SecurityOfficeAttack => {
                    map = map.replace(
                        "{soa}",
                        format!(
                            "{}{}",
                            anim.name.chars().next().unwrap(),
                            anim.current_tell.value()
                        )
                        .as_str(),
                    );
                }
                Locations::SecurityOfficeStaticR => {
                    map = map.replace(
                        "{sosr}",
                        format!(
                            "{}{}",
                            anim.name.chars().next().unwrap(),
                            anim.current_tell.value()
                        )
                        .as_str(),
                    );
                }
            }
        }

        map = map.replace("{ss}", " ");
        map = map.replace("{a}", " ");
        map = map.replace("{dal}", " ");
        map = map.replace("{dac}", " ");
        map = map.replace("{dar}", " ");
        map = map.replace("{k}", " ");
        map = map.replace("{rr}", " ");
        map = map.replace("{hl}", " ");
        map = map.replace("{hr}", " ");
        map = map.replace("{sosl}", " ");
        map = map.replace("{soa}", " ");
        map = map.replace("{sosr}", " ");

        println!("{map}");
    }
}

fn main() {
    const START_TIME: u32 = 0;
    const END_TIME: u32 = 6 * 60;
    const TICK_RATE: u32 = 15; // 15 minutes at a time

    let mut time: u32 = START_TIME;

    let mut battery = Battery::new();
    battery.add_power_draw(PowerDraw::Camera);

    let mut animatronics: Vec<Animatronic> = Vec::new();

    let freddy = Animatronic::new("Freddy".to_string(), Locations::ShowStage, 13);
    let bonnie = Animatronic::new("Bonnie".to_string(), Locations::ShowStage, 15);
    let chica = Animatronic::new("Chica".to_string(), Locations::ShowStage, 19);
    animatronics.push(freddy);
    animatronics.push(bonnie);
    animatronics.push(chica);

    let mut map = Map::new();

    map.anim_states = animatronics;

    loop {
        let (hours, minutes) = display_time(time);
        println!(
            "Time: {:02}:{:02}\nBattery: {}%",
            hours, minutes, battery.power
        );

        battery.update_power();

        map.map_tick();

        map.display_map();

        if battery.power == 0 {
            println!("You ran out of power! Game over!");
            break;
        }

        if map.is_dead {
            println!("You were attacked by {name}! Game over!", name = map.killer);
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
