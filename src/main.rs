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
use std::cell::{RefCell, RefMut};

struct ItemModel {}

#[derive(PartialEq, Debug, Clone)]
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

#[derive(Debug, Clone)]
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
    fn encourage(&mut self, other: RefMut<People>) -> bool {
        if (self.y + self.h >= other.y) && (self.y + self.h <= other.y + other.h) {
            return (self.x + self.w >= other.x) && (self.x + self.w <= other.x + other.w);
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

    fn marker_score(&mut self) {
        self.status = AppleStatue::SCORE;
    }
}

#[derive(Debug)]
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

#[derive(Debug, Clone)]
struct People {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    speed: f64,
    statue: AppleStatue,
}

impl People {
    /// generator a new People which have his own speed
    fn new(x: f64, y: f64) -> People {
        Self {
            x,
            y,
            w: 25.0,
            h: 25.0,
            speed: 2.0,
            statue: AppleStatue::LIVE,
        }
    }
    /// update position
    fn update(&mut self) {
        self.x = self.x + self.speed;
    }
    fn maker_die(&mut self) {
        self.statue = AppleStatue::DIE
    }
}

#[derive(Debug)]
struct Game {
    /// 游戏场景
    scene: GameMode,
    /// 初始生命值
    life: usize,
    lives: usize,
    /// 得分的苹果
    scores: usize,
    /// LIVE 的 苹果
    apples: Vec<RefCell<Apple>>,
    /// 路上的行人, 得分点
    peoples: Arc<Mutex<Vec<RefCell<People>>>>,
    shooter: Shooter,
}

#[derive(Debug)]
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
            scores: 0,
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
                let p = People::new(30.0, 650.0);
                let mut pps = peoples.lock().unwrap();
                pps.push(RefCell::new(p));
            }
        });
    }
    /// update scores apples peoples
    /// here we got some data which showing on screen
    fn update(&mut self) {
        let apples = self.apples.clone();
        // update apples
        self.apples = apples.iter().filter(|a| {
            let a = a.borrow_mut();
            match a.status {
                AppleStatue::SCORE => {
                    {
                        self.scores += 20;
                    }
                    return false;
                }
                AppleStatue::LIVE => {
                    return true;
                }
                AppleStatue::DIE => {
                    self.lives -= 1;
                    if self.lives == 0 {
                        self.over();
                    }
                    return false;
                }
            }
        })
            .cloned()
            .collect();

        // update peoples
        let peoples = self.peoples.lock().unwrap();
//        let peoples: Vec<RefCell<People>> = peoples.iter().filter(|p| {
//            let p = p.borrow_mut();
//            p.statue == AppleStatue::LIVE
//        })
//            .cloned()
//            .collect();
        println!("{:?} ", peoples);
//        self.peoples = Arc::new(Mutex::from(peoples));
    }
    fn over(&mut self) {
        self.scene = GameMode::END
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

    let apple = Texture::from_path(
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
    game.start_game();
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
                                                let apple = game.shooter.throw_apple();
                                                game.apples.push(RefCell::new(apple));
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
                            // draw apples
                            game.apples.iter().for_each(|a| {
                                let mut a = a.borrow_mut();
                                image(&apple,
                                      c.transform.trans(a.x, a.y)
                                      , g);
                                a.update();
                            });
                            let peoples = game.peoples.lock().unwrap();
                            // draw people
                            peoples.iter().for_each(|p| {
                                let mut p = p.borrow_mut();
                                if p.statue == AppleStatue::LIVE {
                                    rectangle(
                                        [1.0, 0.0, 1.0, 1.0],
                                        [p.x, p.y, p.w, p.h],
                                        c.transform,
                                        g,
                                    );
                                }
                                p.update();
                            });
                            game.apples.iter().for_each(|a| {
                                let mut a = a.borrow_mut();
                                peoples.iter().for_each(|p| {
                                    if a.encourage(p.borrow_mut()) {
                                        a.marker_score();
                                        p.borrow_mut().maker_die();
                                    }
                                });
                            });
                        });

                        game.apples.iter().for_each(|a| {
                            let mut a = a.borrow_mut();
                            let peoples = game.peoples.lock().unwrap();
                            peoples.iter().for_each(|p| {
                                if a.encourage(p.borrow_mut()) {
                                    a.marker_score();
                                    p.borrow_mut().maker_die();
                                }
                            });
                        });

                        game.update();
                    }
                    _ => {}
                }
            }
            _ => {}
        };
    }
}
