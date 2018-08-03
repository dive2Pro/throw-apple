extern crate piston_window;

use piston_window::*;
use piston_window::Input;
use std::path::Path;
use std::f64;
use std::thread::park_timeout;
use std::time::{Instant, Duration};
use std::thread;
use std::sync::{Arc, Mutex};
use std::marker::PhantomData;
use std::cmp::PartialEq;

struct ItemModel {}

#[derive(PartialEq)]
enum AppleStatue {
    LIVE,
    SCORE,
    DIE,
}

//impl PartialEd for AppleStatue {
//    fn eq(&self, other:AppleStatue) {
//
//    }
//}

struct Apple {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    status: AppleStatue,
    v: f64,
}

impl Apple {
    fn new(x: f64, y: f64) -> Apple {
        Self {
            x,
            y,
            w: 25.0,
            h: 25.0,
            status: AppleStatue::LIVE,
            v: 10.0,
        }
    }
    /// 检查是否和 other 相遇
    /// 相遇后, 将状态置为 SCORE
    fn encourage(&mut self, other: People) -> bool {
        if (self.y + self.h >= other.y) && (self.y + self.h <= other.y + other.h) {
            let is = (self.x + self.w >= other.x) && (self.x + self.w <= other.x + other.w);
            if is {
                self.status = AppleStatue::SCORE;
            }
            return is;
        }
        false
    }

    /// 更新自己的状态
    /// 如果, 非SCORE, 超过屏幕置状态为 DIE
    ///
    fn update(&mut self) {
        self.y = self.y + self.v;
        if (self.y > 720.0) && (self.status != AppleStatue::SCORE) {
            self.status = AppleStatue::DIE
        }
    }
}

struct Shooter {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    p_x: usize,
    p_y: usize,
    position: Vec<(usize, usize)>,
}


impl Shooter {
    /// 0 <= p_x & p_y <= 2
    fn new(p_x: usize, p_y: usize) -> Self {
        let position = vec![
            (115, 120),
            (220, 120),
            (335, 120),

            (115, 220),
            (220, 220),
            (335, 220),

            (115, 325),
            (220, 325),
            (330, 325),

            (110, 450),
            (220, 450),
            (330, 450),
        ];
        if (p_y > 3) | (p_y < 0) | (p_x < 0) | (p_x > 3) {
            panic!("wrong params, please check doc")
        }

        let index = p_y * 3 + p_x;
        let c_p = position.clone();
        let (x, y) = position.get(index).unwrap();
        Self {
            p_x,
            p_y,
            x: *x as f64,
            y: *y as f64,
            w: 60.0,
            h: 60.0,
            position: c_p,
        }
    }

    fn move_up(&mut self) {
        if self.p_y > 0 {
            self.p_y = self.p_y - 1;
            self.update()
        }
    }

    fn move_down(&mut self) {
        if self.p_y < 3 {
            self.p_y = self.p_y + 1;
            self.update()
        }
    }
    fn move_left(&mut self) {
        if self.p_x > 0 {
            self.p_x = self.p_x - 1;
            self.update()
        }
    }
    fn move_right(&mut self) {
        if self.p_x < 2 {
            self.p_x = self.p_x + 1;
            self.update()
        }
    }
    /// 从当前窗口制造苹果
    fn throw_apple(&self) -> Apple {
        // 扔出的苹果 知道当前的 people有哪些
        // people 走进来时 会告知 苹果
        Apple::new(
            self.x,
            self.y,
        )
    }

    fn update(&mut self) {
        let index = self.p_y * 3 + self.p_x;
        let (x, y) = self.position.get(index).unwrap();
        self.x = *x as f64;
        self.y = *y as f64;
    }
}

#[derive(Debug)]
struct People {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    speed: f64,
}

impl People {
    /// generator a new People which have his own speed
    fn new(x: f64, y: f64) -> People {
        Self {
            x,
            y,
            w: 25.0,
            h: 25.0,
            speed: 10.0,
        }
    }
    /// update position
    fn update(&mut self) {
        self.y = self.y + self.speed;
    }
}


struct Game {
    /// 游戏场景
    scene: GameMode,
    /// 初始生命值
    life: usize,
    lives: usize,
    /// 得分的苹果
    scores: Vec<Apple>,
    /// LIVE 的 苹果
    apples: Vec<Apple>,
    /// 路上的行人, 得分点
    peoples: Arc<Mutex<Vec<People>>>,
    shooter: Shooter,
}

enum GameMode {
    START,
    ING,
    END,
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn game_on() {
//        let mut game = Game::new(19);
//
//        assert!(game.scene == GameMode::START);
//        &game.start_game();
//        thread::park_timeout(Duration::new(4, 0));
//        assert!(game.peoples.len() >= 2);
//    }
//}

impl Game {
    fn new(lives: usize) -> Self {
        Game {
            scene: GameMode::START,
            lives,
            life: lives,
            scores: vec![],
            apples: vec![],
            peoples: Arc::new(Mutex::new(vec![])),
            shooter: Shooter::new(2, 2),
        }
    }

    fn start_game(&mut self) {
        self.scene = GameMode::ING;
        self.produce_people()
    }
    /// Async
    /// produce some people in a interval time
    fn produce_people(&self) {
        let peoples = Arc::clone(&self.peoples);
        thread::spawn(move || {
            let interval = Duration::from_secs(2);
            loop {
                park_timeout(interval);
                let p = People::new(30.0, 720.0);
                let mut pps = peoples.lock().unwrap();
                pps.push(p);
            }
        });
    }
}


fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new(
        "Hello Piston", [500, 724],
    ).opengl(opengl).exit_on_esc(true).build().unwrap();

    let mut game = Game::new(10);

    let house_start = Texture::from_path(
        &mut window.factory,
        Path::new("assets/house-start.jpg"),
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let app = Texture::from_path(
        &mut window.factory,
        Path::new("assets/apple.png"),
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let house = Texture::from_path(
        &mut window.factory,
        Path::new("assets/house.jpg"),
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let black = [0.0, 0.0, 0.0, 1.0];
    let white = [1.0, 1.0, 1.0, 1.0];

    let ref font = Path::new("assets/Amatic-Bold.ttf");
    let mut glyphs = Glyphs::new(font, window.factory.clone(),
                                 TextureSettings::new(),
    ).unwrap();
    let mut x = 50.0;
    let y = 0.0;
    let x_step = 1.0;
    while let Some(e) = window.next() {
        match e {
            Event::Input(Input::Button(key)) => {
                let ButtonArgs { state, button, .. } = key;
                println!("{:?}, {:?}", state, button);
                match game.scene {
                    GameMode::START => {
                        match state {
                            ButtonState::Press => {
                                match button {
                                    Button::Keyboard(key) => {
                                        match key {
                                            Key::P => {
                                                game.start_game();
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    GameMode::ING => {
                        match state {
                            ButtonState::Press => {
                                match button {
                                    Button::Keyboard(key) => {
                                        match key {
                                            Key::A => { game.shooter.move_left(); }
                                            Key::D => { game.shooter.move_right(); }
                                            Key::W => { game.shooter.move_up(); }
                                            Key::S => { game.shooter.move_down(); }
                                            Key::Space => {
                                                // throw apple
                                                game.shooter.throw_apple();
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Event::Loop(Loop::Render(_)) => {
                match game.scene {
                    GameMode::START => {
                        window.draw_2d(&e, |c, g| {
                            clear(white, g);
                            image(&house_start, c.transform.scale(0.5, 0.5), g);
// draw 一些语句
                            Text::new_color(black, 50)
                                .draw(
                                    &"APPLE-RANDOM",
                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform.trans(135.0, 100.0),
                                    g,
                                ).unwrap();
                            let ts = vec![
                                &"use <A> to left",
                                &"use <W> to up",
                                &"use <D> to right",
                                &"use <S> to down",
                            ];

                            ts.iter().enumerate().for_each(|(i, &s)| {
                                let diff_y = i * 50;
                                Text::new_color(black, 40)
                                    .draw(
                                        s,
                                        &mut glyphs,
                                        &c.draw_state,
                                        c.transform.trans(150.0, 200.0 + (diff_y as f64)),
                                        g,
                                    ).unwrap();
                            });
                            Text::new_color(black, 40)
                                .draw(
                                    &"use <Space> to throw apple!",
                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform.trans(100.0, 450.0),
                                    g,
                                ).unwrap();

                            Text::new_color(black, 45)
                                .draw(
                                    &"use <P> to start the game!",
                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform.trans(90.0, 550.0),
                                    g,
                                ).unwrap();
                        });
                    }
                    GameMode::ING => {
                        window.draw_2d(&e, |c, g| {
                            clear(white, g);
                            image(&house, c.transform.scale(0.5, 0.5), g);

                            // draw open screen
                            let Shooter { x, y, w, h, .. } = game.shooter;
                            rectangle(
                                [1.0, 0.0, 1.0, 1.0],
                                [x, y, w, h],
                                c.transform,
                                g,
                            );
                            // draw people
                            // draw apples
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        };
    }
}
