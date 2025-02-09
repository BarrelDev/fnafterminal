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

#[derive(PartialEq, Hash, Clone, Copy, Eq)]
enum Tells {
    Laughing,
    Noise,
    Footsteps,
    Static,
    Visual,
    Breathing,
}

impl Tells {
    fn value(&self) -> &str {
        match self {
            Tells::Laughing => "l",
            Tells::Noise => "n",
            Tells::Footsteps => "f",
            Tells::Static => "s",
            Tells::Visual => "v",
            Tells::Breathing => "b",
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
    fn value(&self) -> i8 {
        match self {
            PowerDraw::Camera => 2,
            PowerDraw::Lights => 4,
            PowerDraw::Doors => 7,
        }
    }
}

impl PartialEq for PowerDraw {
    fn eq(&self, other: &Self) -> bool {
        *self as i8 == *other as i8
    }
}

struct Battery {
    power: i8,
    power_draw: Vec<PowerDraw>,
    is_online: bool,
}

impl Battery {
    fn new() -> Battery {
        Battery {
            power: 100,
            power_draw: Vec::new(),
            is_online: true,
        }
    }

    fn add_power_draw(&mut self, power_draw: PowerDraw) {
        if self.is_online == false {
            return;
        }

        let index = self.power_draw.iter().position(|&x| x == power_draw);
        match index {
            Some(_) => {}
            None => {
                self.power_draw.push(power_draw);
            }
        }
    }

    fn remove_power_draw(&mut self, power_draw: PowerDraw) {
        if self.is_online == false {
            return;
        }

        let index = self.power_draw.iter().position(|&x| x == power_draw);
        match index {
            Some(i) => {
                self.power_draw.remove(i);
            }
            None => {}
        }
    }

    fn update_power(&mut self) {
        if self.is_online == false {
            return;
        }

        let mut rng = rand::thread_rng();
        for (_i, &draw) in self.power_draw.iter().enumerate() {
            let random_tick: u8 = rng.gen_range(1..20);
            if draw as u8 * 2 <= random_tick {
                if self.power > 0 {
                    self.power -= draw.value();
                } else {
                    self.power = 0;
                }
            }
        }
    }

    fn shutdown(&mut self) {
        self.power = -1;
        self.power_draw.clear();
        self.is_online = false;
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
        if right_door_closed && self.location == Locations::SecurityOfficeStaticR {
            self.location = Locations::HallwayR;
        }

        if left_door_closed && self.location == Locations::SecurityOfficeStaticL {
            self.location = Locations::HallwayL;
        }

        // move the animatronic
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..20);

        if random_index <= self.difficulty {
            {
                // move the animatronic
                let adjacent_rooms = adjacent_loc;
                // let mut closest: Locations = Locations::ShowStage;

                let random_index = rng.gen_range(0..adjacent_rooms.len());

                // Check to see if the animatronic is trying to move to the security office
                // if it is, check if the door is closed
                // if it is, don't move there
                if adjacent_rooms[random_index] == Locations::SecurityOfficeStaticR
                    && right_door_closed
                {
                    return;
                }

                if adjacent_rooms[random_index] == Locations::SecurityOfficeStaticL
                    && left_door_closed
                {
                    return;
                }

                self.location = adjacent_rooms[random_index];
            }

            {
                // set the tell
                let random_tell = rng.gen_range(0..5);
                match random_tell {
                    0 => {
                        self.current_tell = Tells::Laughing;
                    }
                    1 => {
                        self.current_tell = Tells::Noise;
                    }
                    2 => {
                        self.current_tell = Tells::Footsteps;
                    }
                    3 => {
                        self.current_tell = Tells::Static;
                    }
                    4 => {
                        self.current_tell = Tells::Visual;
                    }
                    _ => {
                        self.current_tell = Tells::Visual;
                    }
                }

                if self.name == "Freddy" && self.current_tell == Tells::Noise {
                    self.current_tell = Tells::Laughing;
                } else if self.current_tell == Tells::Laughing {
                    self.current_tell = Tells::Noise;
                }

                if self.name == "Chica" && self.location == Locations::Kitchen && random_tell > 2 {
                    self.current_tell = Tells::Static;
                }

                if self.location == Locations::SecurityOfficeStaticL
                    || self.location == Locations::SecurityOfficeStaticR
                {
                    self.current_tell = Tells::Breathing;
                }
            }
        }
    }
}

struct Map {
    grid: [[Option<Locations>; 9]; 5],
    left_door_closed: bool,
    right_door_closed: bool,
    left_light_on: bool,
    right_light_on: bool,
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
            left_light_on: false,
            right_light_on: false,
            anim_states: Vec::new(),
            is_dead: false,
            killer: String::from("MissingNo."),
        }
    }

    fn night_reset(&mut self) {
        // reset the map
        self.left_door_closed = false;
        self.right_door_closed = false;
        self.left_light_on = false;
        self.right_light_on = false;
        self.is_dead = false;
        self.killer = String::from("MissingNo.");

        for (_i, anim) in self.anim_states.iter_mut().enumerate() {
            anim.location = Locations::ShowStage;
            anim.current_tell = Tells::Visual;
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

            if anim.location == Locations::HallwayL && self.left_light_on {
                println!("You see {} is at the left door!", anim.name);
            }

            if anim.location == Locations::HallwayR && self.right_light_on {
                println!("You see {} is at the right door!", anim.name);
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
                        format!("{}", {
                            if anim.current_tell.value() == "v" {
                                anim.name.chars().next().unwrap().to_string()
                            } else {
                                anim.current_tell.value().to_string()
                            }
                        })
                        .as_str(),
                    );
                }
                Locations::Arcade => {
                    map = map.replace(
                        "{a}",
                        format!("{}", {
                            if anim.current_tell.value() == "v" {
                                anim.name.chars().next().unwrap().to_string()
                            } else {
                                anim.current_tell.value().to_string()
                            }
                        })
                        .as_str(),
                    );
                }
                Locations::DiningAreaL => {
                    map = map.replace(
                        "{dal}",
                        format!("{}", {
                            if anim.current_tell.value() == "v" {
                                anim.name.chars().next().unwrap().to_string()
                            } else {
                                anim.current_tell.value().to_string()
                            }
                        })
                        .as_str(),
                    );
                }
                Locations::DiningAreaC => {
                    map = map.replace(
                        "{dac}",
                        format!("{}", {
                            if anim.current_tell.value() == "v" {
                                anim.name.chars().next().unwrap().to_string()
                            } else {
                                anim.current_tell.value().to_string()
                            }
                        })
                        .as_str(),
                    );
                }
                Locations::DiningAreaR => {
                    map = map.replace(
                        "{dar}",
                        format!("{}", {
                            if anim.current_tell.value() == "v" {
                                anim.name.chars().next().unwrap().to_string()
                            } else {
                                anim.current_tell.value().to_string()
                            }
                        })
                        .as_str(),
                    );
                }
                Locations::Kitchen => {
                    map = map.replace(
                        "{k}",
                        format!("{}", {
                            if anim.current_tell.value() == "v" {
                                anim.name.chars().next().unwrap().to_string()
                            } else {
                                anim.current_tell.value().to_string()
                            }
                        })
                        .as_str(),
                    );
                }
                Locations::Restrooms => {
                    map = map.replace(
                        "{rr}",
                        format!("{}", {
                            if anim.current_tell.value() == "v" {
                                anim.name.chars().next().unwrap().to_string()
                            } else {
                                anim.current_tell.value().to_string()
                            }
                        })
                        .as_str(),
                    );
                }
                Locations::HallwayL => {
                    map = map.replace(
                        "{hl}",
                        format!("{}", {
                            if anim.current_tell.value() == "v" {
                                anim.name.chars().next().unwrap().to_string()
                            } else {
                                anim.current_tell.value().to_string()
                            }
                        })
                        .as_str(),
                    );
                }
                Locations::HallwayR => {
                    map = map.replace(
                        "{hr}",
                        format!("{}", {
                            if anim.current_tell.value() == "v" {
                                anim.name.chars().next().unwrap().to_string()
                            } else {
                                anim.current_tell.value().to_string()
                            }
                        })
                        .as_str(),
                    );
                }
                Locations::SecurityOfficeStaticL => {
                    map = map.replace(
                        "{sosl}",
                        format!("{}", {
                            if anim.current_tell.value() == "v" {
                                anim.name.chars().next().unwrap().to_string()
                            } else {
                                anim.current_tell.value().to_string()
                            }
                        })
                        .as_str(),
                    );
                }
                Locations::SecurityOfficeAttack => {
                    map = map.replace(
                        "{soa}",
                        format!("{}", { anim.name.chars().next().unwrap().to_string() }).as_str(),
                    );
                }
                Locations::SecurityOfficeStaticR => {
                    map = map.replace(
                        "{sosr}",
                        format!("{}", {
                            if anim.current_tell.value() == "v" {
                                anim.name.chars().next().unwrap().to_string()
                            } else {
                                anim.current_tell.value().to_string()
                            }
                        })
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

    let mut night: u8 = 1;

    let mut freddy_state_power_down: u8 = 0;

    let mut battery = Battery::new();

    let mut map = Map::new();

    let mut turn_input: String = String::new();

    println!("Welcome to Five Nights at Freddy's. ");
    loop {
        loop {
            println!("Main Menu: ");
            println!(
                "Please select an option.\n\t New Game \n\t Custom Night \n\t Explain \n\t Exit"
            );
            turn_input.clear();
            let _ = std::io::stdin().read_line(&mut turn_input).unwrap();

            match turn_input.trim().to_lowercase().as_str() {
                "new game" => {
                    night = 1;
                    let mut animatronics = Vec::new();

                    let freddy = Animatronic::new("Freddy".to_string(), Locations::ShowStage, 5);
                    let bonnie = Animatronic::new("Bonnie".to_string(), Locations::ShowStage, 3);
                    let chica = Animatronic::new("Chica".to_string(), Locations::ShowStage, 3);
                    animatronics.push(freddy);
                    animatronics.push(bonnie);
                    animatronics.push(chica);

                    map.anim_states = animatronics;

                    // five night cycle.
                    loop {
                        println!("Dusk of Night {night}", night = night);
                        time = START_TIME;
                        battery = Battery::new();
                        freddy_state_power_down = 0;
                        map.night_reset();

                        // night loop.
                        loop {
                            let (hours, minutes) = display_time(time);

                            turn_input.clear();

                            println!(
                                "Time: {:02}:{:02}\nBattery: {}%",
                                hours, minutes, battery.power
                            );

                            println!("Office State: \n\tLeft Door: {}\n\tRight Door: {}\n\tLeft Light: {}\n\tRight Light: {}",
                            if map.left_door_closed { "Closed" } else { "Open" },
                            if map.right_door_closed { "Closed" } else { "Open" },
                            if map.left_light_on { "On" } else { "Off" },
                            if map.right_light_on { "On" } else { "Off" });

                            loop {
                                println!("What is your move this turn? : ");

                                let _ = std::io::stdin().read_line(&mut turn_input).unwrap();

                                if battery.is_online {
                                    match turn_input.trim() {
                                        "left door" => {
                                            map.left_door_closed = !map.left_door_closed;
                                            if map.left_door_closed {
                                                battery.add_power_draw(PowerDraw::Doors);
                                            } else {
                                                battery.remove_power_draw(PowerDraw::Doors);
                                            }
                                            break;
                                        }
                                        "left light" => {
                                            map.left_light_on = !map.left_light_on;
                                            if map.left_light_on {
                                                battery.add_power_draw(PowerDraw::Lights);
                                            } else {
                                                battery.remove_power_draw(PowerDraw::Lights);
                                            }
                                            break;
                                        }
                                        "right door" => {
                                            map.right_door_closed = !map.right_door_closed;
                                            if map.right_door_closed {
                                                battery.add_power_draw(PowerDraw::Doors);
                                            } else {
                                                battery.remove_power_draw(PowerDraw::Doors);
                                            }
                                            break;
                                        }
                                        "right light" => {
                                            map.right_light_on = !map.right_light_on;
                                            if map.right_light_on {
                                                battery.add_power_draw(PowerDraw::Lights);
                                            } else {
                                                battery.remove_power_draw(PowerDraw::Lights);
                                            }
                                            break;
                                        }
                                        "camera" => {
                                            battery.add_power_draw(PowerDraw::Camera);
                                            map.display_map();
                                            break;
                                        }
                                        "sit" => {
                                            break;
                                        }
                                        _ => {
                                            println!("Invalid command!");
                                            turn_input.clear();
                                        }
                                    }
                                } else {
                                    match turn_input.trim() {
                                        "sit" => {
                                            break;
                                        }
                                        _ => {
                                            println!("Invalid command!");
                                            turn_input.clear();
                                        }
                                    }
                                }
                            }

                            battery.update_power();
                            battery.remove_power_draw(PowerDraw::Camera);

                            if battery.power == 0 {
                                println!("You ran out of power! All systems are down!");
                                battery.shutdown();
                                map.left_door_closed = false;
                                map.right_door_closed = false;
                                map.left_light_on = false;
                                map.right_light_on = false;
                                map.anim_states[0].location = Locations::HallwayR;
                            }

                            if battery.is_online == false
                                && map.anim_states[0].location == Locations::HallwayR
                            {
                                match freddy_state_power_down {
                                    0 => {
                                        println!("You see glowing eyes to your right.");
                                    }
                                    1 => {
                                        println!("You hear a voice say, 'It's me.'");
                                    }
                                    2 => {
                                        println!("You hear a voice say, 'I am still here.'");
                                    }
                                    3 => {
                                        println!("You hear a voice say, 'I am always here.'");
                                    }
                                    4 => {
                                        println!("You hear a voice say, 'I am always watching.'");
                                    }
                                    5 => {
                                        println!(
                                            "You hear a voice say, 'I am always watching you.'"
                                        );
                                    }
                                    _ => {
                                        println!("There is silence.");
                                        map.anim_states[0].location =
                                            Locations::SecurityOfficeAttack;
                                        map.killer = String::from("Freddy");
                                        map.is_dead = true;
                                    }
                                }
                                freddy_state_power_down += 1;
                            }

                            if map.is_dead {
                                println!(
                                    "You were attacked by {name}! Game over!",
                                    name = map.killer
                                );
                                break;
                            }

                            time += TICK_RATE;

                            if time >= END_TIME {
                                println!("You survived the night! Congratulations! \n");
                                night += 1;
                                break;
                            }

                            map.map_tick();
                        }
                        if map.is_dead {
                            break;
                        }
                        if night > 5 {
                            println!("You survived all 5 nights! Congratulations! \n");
                            night = 1;
                            break;
                        }
                        for anim in map.anim_states.iter_mut() {
                            let mut rng = rand::thread_rng();
                            anim.difficulty += rng.gen_range(1..3);
                        }
                    }
                    break;
                }
                "custom night" => {
                    turn_input.clear();

                    let mut animatronics = Vec::new();
                    println!("Please enter the difficulty for Freddy: ");
                    let _ = std::io::stdin().read_line(&mut turn_input).unwrap();
                    let freddy_difficulty: u8 = match turn_input.trim().parse::<u8>() {
                        Ok(val) => val,
                        Err(_) => 5,
                    };
                    turn_input.clear();

                    // println!("freedddy diff: {}", freddy_difficulty);

                    println!("Please enter the difficulty for Bonnie: ");
                    let _ = std::io::stdin().read_line(&mut turn_input).unwrap();
                    let bonnie_difficulty: u8 = match turn_input.trim().parse::<u8>() {
                        Ok(val) => val,
                        Err(_) => 3,
                    };
                    turn_input.clear();
                    // println!("bonniee diff: {}", bonnie_difficulty);

                    println!("Please enter the difficulty for Chica: ");
                    let _ = std::io::stdin().read_line(&mut turn_input).unwrap();
                    let chica_difficulty: u8 = match turn_input.trim().parse::<u8>() {
                        Ok(val) => val,
                        Err(_) => 3,
                    };
                    turn_input.clear();
                    // println!("chica diff: {}", chica_difficulty);

                    let freddy = Animatronic::new(
                        "Freddy".to_string(),
                        Locations::ShowStage,
                        freddy_difficulty,
                    );
                    let bonnie = Animatronic::new(
                        "Bonnie".to_string(),
                        Locations::ShowStage,
                        bonnie_difficulty,
                    );
                    let chica = Animatronic::new(
                        "Chica".to_string(),
                        Locations::ShowStage,
                        chica_difficulty,
                    );
                    animatronics.push(freddy);
                    animatronics.push(bonnie);
                    animatronics.push(chica);

                    map.anim_states = animatronics;

                    println!("Dusk of Custom Night");
                    turn_input.clear();

                    time = START_TIME;
                    battery = Battery::new();
                    freddy_state_power_down = 0;
                    map.night_reset();

                    loop {
                        let (hours, minutes) = display_time(time);

                        turn_input.clear();

                        println!(
                            "Time: {:02}:{:02}\nBattery: {}%",
                            hours, minutes, battery.power
                        );

                        println!("Office State: \n\tLeft Door: {}\n\tRight Door: {}\n\tLeft Light: {}\n\tRight Light: {}",
                        if map.left_door_closed { "Closed" } else { "Open" },
                        if map.right_door_closed { "Closed" } else { "Open" },
                        if map.left_light_on { "On" } else { "Off" },
                        if map.right_light_on { "On" } else { "Off" });

                        loop {
                            println!("What is your move this turn? : ");

                            let _ = std::io::stdin().read_line(&mut turn_input).unwrap();

                            if battery.is_online {
                                match turn_input.trim() {
                                    "left door" => {
                                        map.left_door_closed = !map.left_door_closed;
                                        if map.left_door_closed {
                                            battery.add_power_draw(PowerDraw::Doors);
                                        } else {
                                            battery.remove_power_draw(PowerDraw::Doors);
                                        }
                                        break;
                                    }
                                    "left light" => {
                                        map.left_light_on = !map.left_light_on;
                                        if map.left_light_on {
                                            battery.add_power_draw(PowerDraw::Lights);
                                        } else {
                                            battery.remove_power_draw(PowerDraw::Lights);
                                        }
                                        break;
                                    }
                                    "right door" => {
                                        map.right_door_closed = !map.right_door_closed;
                                        if map.right_door_closed {
                                            battery.add_power_draw(PowerDraw::Doors);
                                        } else {
                                            battery.remove_power_draw(PowerDraw::Doors);
                                        }
                                        break;
                                    }
                                    "right light" => {
                                        map.right_light_on = !map.right_light_on;
                                        if map.right_light_on {
                                            battery.add_power_draw(PowerDraw::Lights);
                                        } else {
                                            battery.remove_power_draw(PowerDraw::Lights);
                                        }
                                        break;
                                    }
                                    "camera" => {
                                        battery.add_power_draw(PowerDraw::Camera);
                                        map.display_map();
                                        break;
                                    }
                                    "sit" => {
                                        break;
                                    }
                                    _ => {
                                        println!("Invalid command!");
                                        turn_input.clear();
                                    }
                                }
                            } else {
                                match turn_input.trim() {
                                    "sit" => {
                                        break;
                                    }
                                    _ => {
                                        println!("Invalid command!");
                                        turn_input.clear();
                                    }
                                }
                            }
                        }

                        battery.update_power();
                        battery.remove_power_draw(PowerDraw::Camera);

                        if battery.power == 0 {
                            println!("You ran out of power! All systems are down!");
                            battery.shutdown();
                            map.left_door_closed = false;
                            map.right_door_closed = false;
                            map.left_light_on = false;
                            map.right_light_on = false;
                            map.anim_states[0].location = Locations::HallwayR;
                        }

                        if battery.is_online == false
                            && map.anim_states[0].location == Locations::HallwayR
                        {
                            match freddy_state_power_down {
                                0 => {
                                    println!("You see glowing eyes to your right.");
                                }
                                1 => {
                                    println!("You hear a voice say, 'It's me.'");
                                }
                                2 => {
                                    println!("You hear a voice say, 'I am still here.'");
                                }
                                3 => {
                                    println!("You hear a voice say, 'I am always here.'");
                                }
                                4 => {
                                    println!("You hear a voice say, 'I am always watching.'");
                                }
                                5 => {
                                    println!("You hear a voice say, 'I am always watching you.'");
                                }
                                _ => {
                                    println!("There is silence.");
                                    map.anim_states[0].location = Locations::SecurityOfficeAttack;
                                    map.killer = String::from("Freddy");
                                    map.is_dead = true;
                                }
                            }
                            freddy_state_power_down += 1;
                        }

                        if map.is_dead {
                            println!("You were attacked by {name}! Game over!", name = map.killer);
                            break;
                        }

                        time += TICK_RATE;

                        if time >= END_TIME {
                            println!("You survived the night! Congratulations!");
                            night += 1;
                            break;
                        }

                        map.map_tick();
                    }

                    break;
                }
                "explain" => {
                    println!("Five Nights at Freddy's is a survival horror game where you play as a security guard at Freddy Fazbear's Pizza. \n You must survive the night by managing your power and keeping the animatronics at bay. \n The animatronics will move around the pizzeria and try to attack you. You must use the cameras and doors to keep them away. \n If you run out of power, you will be attacked and the game will be over. Good luck!\n");

                    println!("Commands: \n\t left door -- open/close left door \n\t right door -- open/close right door \n\t left light -- turn on/off left light \n\t right light -- turn on/off right light \n\t camera -- check cameras \n\t sit -- do nothing \n");

                    println!("Tells: \n\t l -- laughing \n\t n -- noise \n\t f -- footsteps \n\t s -- static \n\t v -- visual \n\t b -- breathing \n");

                    println!("Locations: \n\t Show Stage \n\t Dining Area L \n\t Dining Area R \n\t Dining Area C \n\t Restrooms \n\t Kitchen \n\t Arcade \n\t Security Office Static R \n\t Security Office Static L \n\t Security Office Attack \n\t Hallway L \n\t Hallway R \n");

                    println!("Animatronics: \n\t F - Freddy \n\t B - Bonnie \n\t C - Chica \n");

                    break;
                }
                "exit" => {
                    return;
                }
                _ => {
                    println!("Invalid command!");
                    turn_input.clear();
                }
            }
        }
    }
}

fn display_time(time: u32) -> (u32, u32) {
    let hours = time / 60;
    let minutes = time % 60;
    (hours, minutes)
}
