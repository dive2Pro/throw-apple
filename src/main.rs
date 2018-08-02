extern crate piston_window;

use piston_window::*;
use piston_window::Input;
use std::path::Path;
use std::f64;

struct Game {
    scene: GameMode
}

enum GameMode {
    START,
    ING,
    END,
}

impl Game {
    fn new() -> Self {
        Game {
            scene: GameMode::START
        }
    }

    fn start_game(&mut self) {
        self.scene = GameMode::ING
    }
}


fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new(
        "Hello Piston", [500, 724],
    ).opengl(opengl).exit_on_esc(true).build().unwrap();

    let mut game = Game::new();

    let house_start = Texture::from_path(
        &mut window.factory,
        Path::new("assets/house-start.jpg"),
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
                                            Key::A => {
                                                x = x - x_step;
                                            }
                                            Key::D => {
                                                x = x + x_step;
                                            }
                                            Key::W => {}
                                            Key::S => {}
                                            Key::Space => {}
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
                        });

                    }
                    _ => {}
                }
            }
            _ => {}
        };
    }
}
