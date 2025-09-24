use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::{Duration, Instant};
use rand::Rng;

const WINDOW_SIZE: i32 = 800;
const ROAD_WIDTH: i32 = 80;
const TRAFFIC_LIGHT_SIZE: i32 = 25;
const CAR_WIDTH: i32 = 15;
const CAR_HEIGHT: i32 = 15;
const MIN_VELOCITY: i32 = 2;
const MAX_VELOCITY: i32 = 3;
const SECURITY_DISTANCE: i32 = 30;
const SPAWN_COOLDOWN_MS: u64 = 500;
const CENTER: i32 = WINDOW_SIZE / 2;
const STOP_LINE_NORTH: i32 = CENTER - ROAD_WIDTH / 2 - CAR_HEIGHT / 2;
const STOP_LINE_SOUTH: i32 = CENTER + ROAD_WIDTH / 2 + CAR_HEIGHT / 2;
const STOP_LINE_WEST: i32 = CENTER - ROAD_WIDTH / 2 - CAR_WIDTH / 2;
const STOP_LINE_EAST: i32 = CENTER + ROAD_WIDTH / 2 + CAR_WIDTH / 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LightColor { Red, Green }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side { FromSouth, FromNorth, FromWest, FromEast }

struct TrafficLight {
    x: i32,
    y: i32,
    color: LightColor,
}

#[derive(Debug, Clone, Copy)]
struct Car {
    x: i32,
    y: i32,
    velocity: i32,
    side: Side,
    color: Color,
    stopped: bool,
}

impl Car {
    fn should_stop(&self, front_car: Option<&Car>, stop_line: i32, light: LightColor) -> bool {
        if let Some(car) = front_car {
            let distance = match self.side {
                Side::FromNorth => car.y - self.y - CAR_HEIGHT,
                Side::FromSouth => self.y - car.y - CAR_HEIGHT,
                Side::FromWest => car.x - self.x - CAR_WIDTH,
                Side::FromEast => self.x - car.x - CAR_WIDTH,
            };
            if distance < SECURITY_DISTANCE {
                return true;
            }
        }

        match self.side {
            Side::FromNorth => self.y + CAR_HEIGHT < stop_line && light == LightColor::Red,
            Side::FromSouth => self.y - CAR_HEIGHT > stop_line && light == LightColor::Red,
            Side::FromWest => self.x + CAR_WIDTH < stop_line && light == LightColor::Red,
            Side::FromEast => self.x - CAR_WIDTH > stop_line && light == LightColor::Red,
        }
    }

    fn update_position(&mut self, front_car: Option<&Car>, stop_line: i32, light: LightColor) {
        self.stopped = self.should_stop(front_car, stop_line, light);

        if !self.stopped {
            match self.side {
                Side::FromNorth => self.y += self.velocity,
                Side::FromSouth => self.y -= self.velocity,
                Side::FromWest => self.x += self.velocity,
                Side::FromEast => self.x -= self.velocity,
            }
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(
            self.x - CAR_WIDTH / 2,
            self.y - CAR_HEIGHT / 2,
            CAR_WIDTH as u32,
            CAR_HEIGHT as u32,
        ))?;
        Ok(())
    }
}

impl TrafficLight {
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let color = match self.color {
            LightColor::Red => Color::RGB(255, 0, 0),
            LightColor::Green => Color::RGB(0, 255, 0),
        };
        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(
            self.x,
            self.y,
            TRAFFIC_LIGHT_SIZE as u32,
            TRAFFIC_LIGHT_SIZE as u32,
        ))?;
        Ok(())
    }
}

struct Intersection {
    lights: [TrafficLight; 4],
    light_change_time: Instant,
    cars: Vec<Car>,
    last_spawn: Instant,
}

impl Intersection {
    fn new() -> Self {
        let half_road = ROAD_WIDTH / 2;

        Intersection {
            lights: [
                TrafficLight {
                    x: CENTER - half_road - TRAFFIC_LIGHT_SIZE,
                    y: CENTER - half_road - TRAFFIC_LIGHT_SIZE,
                    color: LightColor::Red,
                },
                TrafficLight {
                    x: CENTER + half_road,
                    y: CENTER + half_road,
                    color: LightColor::Red,
                },
                TrafficLight {
                    x: CENTER - half_road - TRAFFIC_LIGHT_SIZE,
                    y: CENTER + half_road,
                    color: LightColor::Green,
                },
                TrafficLight {
                    x: CENTER + half_road,
                    y: CENTER - half_road - TRAFFIC_LIGHT_SIZE,
                    color: LightColor::Green,
                },
            ],
            light_change_time: Instant::now(),
            cars: Vec::new(),
            last_spawn: Instant::now() - Duration::from_millis(SPAWN_COOLDOWN_MS),
        }
    }

    fn spawn_car(&mut self, side: Side) {
        if Instant::now().duration_since(self.last_spawn).as_millis() < SPAWN_COOLDOWN_MS as u128 {
            return;
        }

        let mut rng = rand::thread_rng();
        let velocity = rng.gen_range(MIN_VELOCITY..=MAX_VELOCITY);
        let color = match side {
            Side::FromNorth => Color::RGB(255, 0, 0),
            Side::FromSouth => Color::RGB(0, 255, 0),
            Side::FromWest => Color::RGB(0, 0, 255),
            Side::FromEast => Color::RGB(255, 255, 0),
        };
        let (x, y) = match side {
            Side::FromNorth => (CENTER - ROAD_WIDTH / 4, 0),
            Side::FromSouth => (CENTER + ROAD_WIDTH / 4, WINDOW_SIZE),
            Side::FromWest => (0, CENTER + ROAD_WIDTH / 4),
            Side::FromEast => (WINDOW_SIZE, CENTER - ROAD_WIDTH / 4),
        };

        self.cars.push(Car { x, y, velocity, side, color, stopped: false });
        self.last_spawn = Instant::now();
    }

    fn update_lights(&mut self) {
        if Instant::now().duration_since(self.light_change_time) > Duration::from_secs(5) {
            let (ns, ew) = match self.lights[0].color {
                LightColor::Red => (LightColor::Green, LightColor::Red),
                LightColor::Green => (LightColor::Red, LightColor::Green),
            };
            self.lights[0].color = ns;
            self.lights[1].color = ns;
            self.lights[2].color = ew;
            self.lights[3].color = ew;
            self.light_change_time = Instant::now();
        }
    }

    fn get_stop_line(&self, side: Side) -> i32 {
        match side {
            Side::FromNorth => STOP_LINE_NORTH,
            Side::FromSouth => STOP_LINE_SOUTH,
            Side::FromWest => STOP_LINE_WEST,
            Side::FromEast => STOP_LINE_EAST,
        }
    }

    fn get_light_for_car(&self, car: &Car) -> LightColor {
        match car.side {
            Side::FromNorth => self.lights[0].color,
            Side::FromSouth => self.lights[1].color,
            Side::FromWest => self.lights[2].color,
            Side::FromEast => self.lights[3].color,
        }
    }

    fn update_cars(&mut self) {
        let len = self.cars.len();
        for i in 0..len {
            let stop_line = self.get_stop_line(self.cars[i].side);
            let light = self.get_light_for_car(&self.cars[i]);

            let (left, right) = self.cars.split_at_mut(i);
            let (car, right) = right.split_first_mut().unwrap();

            let front_car = left.iter()
                .chain(right.iter())
                .filter(|c| c.side == car.side)
                .filter(|c| match car.side {
                    Side::FromNorth => c.y > car.y,
                    Side::FromSouth => c.y < car.y,
                    Side::FromWest => c.x > car.x,
                    Side::FromEast => c.x < car.x,
                })
                .min_by_key(|c| match car.side {
                    Side::FromNorth => c.y,
                    Side::FromSouth => -c.y,
                    Side::FromWest => c.x,
                    Side::FromEast => -c.x,
                });

            car.update_position(front_car, stop_line, light);
        }

        self.cars.retain(|c| c.x >= -50 && c.x <= WINDOW_SIZE + 50 && c.y >= -50 && c.y <= WINDOW_SIZE + 50);
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(40, 40, 40));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.fill_rect(Rect::new(CENTER - ROAD_WIDTH / 2, 0, ROAD_WIDTH as u32, WINDOW_SIZE as u32))?;
        canvas.fill_rect(Rect::new(0, CENTER - ROAD_WIDTH / 2, WINDOW_SIZE as u32, ROAD_WIDTH as u32))?;
        for light in &self.lights {
            light.render(canvas)?;
        }
        for car in &self.cars {
            car.render(canvas)?;
        }

        Ok(())
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Traffic Simulation", WINDOW_SIZE as u32, WINDOW_SIZE as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut intersection = Intersection::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running Ok(()),
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => intersection.spawn_car(Side::FromSouth),
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => intersection.spawn_car(Side::FromNorth),
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => intersection.spawn_car(Side::FromEast),
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => intersection.spawn_car(Side::FromWest),
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    let mut rng = rand::thread_rng();
                    let side = match rng.gen_range(0..4) {
                        0 => Side::FromNorth,
                        1 => Side::FromSouth,
                        2 => Side::FromWest,
                        _ => Side::FromEast,
                    };
                    intersection.spawn_car(side);
                }
                _ => {}
            }
        }

        intersection.update_lights();
        intersection.update_cars();
        intersection.render(&mut canvas)?;
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}
