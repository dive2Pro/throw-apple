extern crate piston_window;

use piston_window::*;
use piston_window::Input;
use std::path::Path;
use std::f64;
use std::thread::park_timeout;
use std::time::{Instant, Duration};
use std::thread;
use std::sync:: {Arc, Mutex};
use std::marker:: {PhantomData};
struct ItemModel {}

enum AppleStatue {
    LIVE,
    SCORE,
    DIE,
}

struct Apple<'a> {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    status: AppleStatue
}

impl <'a> Apple<'a> {
    fn new(x: usize, y: usize) -> Apple<'a> {
        Self {
            x,
            y,
            w: 25,
            h: 25,
            status: AppleStatue::LIVE,
        }
    }
    /// 检查是否和 other 相遇
    /// 相遇后, 将状态置为 SCORE
    fn encourage(&self, other: &ItemModel) -> bool {
        unimplemented!()
    }
    /// 更新自己的状态
    /// 如果, 非SCORE, 超过屏幕置状态为 DIE
    ///
    fn update(&self) {
        unimplemented!()
    }
}

struct Screen {
    x: usize,
    y: usize,
    w: usize,
    h: usize,

}


impl Screen {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            w: 100,
            h: 100,
        }
    }

    fn move_up(&self) {}
    fn move_down(&self) {}
    fn move_left(&self) {}
    fn move_right(&self) {}
    /// 从当前窗口制造苹果
    fn throw_apple(&self) -> Apple {
        // 扔出的苹果 知道当前的 people有哪些
        // people 走进来时 会告知 苹果
        unimplemented!()
    }
}

struct People<'a> {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    speed: usize,
}

impl <'a> People<'a> {
    /// generator a new People which have his own speed
    fn new(x: usize, y: usize) -> People<'a>{
        Self {
            x,
            y,
            w: 25,
            h: 25,
            speed: 10,
        }
    }
    /// update position
    fn update(&self) {}
}


struct Game<'b>{
    /// 游戏场景
    scene: GameMode,
    /// 初始生命值
    life: usize,
    lives: usize,
    /// 得分的苹果
    scores: Vec<Apple<'b>>,
    /// LIVE 的 苹果
    apples: Vec<Apple<'b>>,
    /// 路上的行人, 得分点
    peoples: Arc<Vec<People<'b>>>,
}

enum GameMode {
    START,
    ING,
    END,
}

impl<'a> Game<'a> {
    fn new(lives: usize) -> Self {
        Game {
            scene: GameMode::START,
            lives,
            life: lives,
            scores: vec![],
            apples: vec![],
            peoples: Arc::new(vec![]),
        }
    }

    fn start_game(&mut self) {
        self.scene = GameMode::ING;
        self.produce_people()
    }
    /// Async
    /// produce some people in a interval time
    fn produce_people(&'a self) {
        thread::spawn( || {
            let interval = Duration::from_secs(2);
            loop {
                park_timeout(interval);
                let p = People::new(30, 720);
                &self.peoples.clone().push(p);
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
    let mut y = 0.0;
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
                                            Key::A => { x = x - x_step; }
                                            Key::D => { x = x + x_step; }
                                            Key::W => {}
                                            Key::S => {}
                                            Key::Space => {
                                                // throw apple
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
