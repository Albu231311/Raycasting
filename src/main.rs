// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

mod line;
mod framebuffer;
mod maze;
mod caster;
mod player;
mod texture;
mod sprites; 

use line::line;
use maze::{Maze, load_maze, extract_sprite_positions, clean_maze}; 
use caster::{cast_ray, Intersect, render_world_with_textures, render_world_with_textures_and_sprites}; 
use framebuffer::Framebuffer;
use player::{Player, process_events};
use texture::TextureManager;
use sprites::SpriteManager; 

use raylib::prelude::*;

use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;

use std::thread;
use std::time::Duration;
use std::f32::consts::PI;

enum GameState {
    Welcome,
    Playing,
    Victory, 
}

fn cell_to_color(cell: char) -> Color {
    match cell {
        '+' => Color::new(139, 69, 19, 255),     
        '-' => Color::new(139, 69, 19, 255),    
        '|' => Color::new(139, 69, 19, 255),     
        'g' => Color::new(0, 128, 0, 255),      
        '1' => Color::new(139, 69, 19, 255),     
        '2' => Color::new(105, 105, 105, 255),   
        '3' => Color::new(255, 215, 0, 255),    
        _ => Color::new(34, 139, 34, 255), 
    }
}

fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
) {
    if cell == ' ' {
        return;
    }
    let color = cell_to_color(cell);
    framebuffer.set_current_color(color);

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.set_pixel(x as u32, y as u32);
        }
    }
}

pub fn render_maze(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }

    framebuffer.set_current_color(Color::WHITESMOKE);

    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, &player, a, block_size, true);
    }
}

fn render_world(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
) {
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;

    // --- CIELO Y SUELO ---
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            if y < hh as u32 {
                framebuffer.set_current_color(Color::SKYBLUE);
            } else {
                framebuffer.set_current_color(Color::DARKGREEN);
            }
            framebuffer.set_pixel(x, y);
        }
    }

    framebuffer.set_current_color(Color::WHITESMOKE);

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        let distance_to_wall = intersect.distance;
        let distance_to_projection_plane = 70.0;
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        for y in stake_top..stake_bottom {
            framebuffer.set_pixel(i, y as u32);
        }
    }
}

fn render_minimap(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    minimap_size: usize,
    block_size: usize,
) {
    let map_rows = maze.len();
    let map_cols = maze[0].len();

    let scale_x = minimap_size as f32 / (map_cols * block_size) as f32;
    let scale_y = minimap_size as f32 / (map_rows * block_size) as f32;

    let offset_x = framebuffer.width as usize - minimap_size - 10;
    let offset_y = 10;

    // Fondo del minimapa - Verde césped NFL más auténtico
    framebuffer.set_current_color(Color::new(34, 139, 34, 255)); // Forest Green
    for x in offset_x..offset_x + minimap_size {
        for y in offset_y..offset_y + minimap_size {
            if x < framebuffer.width as usize && y < framebuffer.height as usize {
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }

    // Dibujar paredes - Color gris oscuro como las líneas del campo
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell != ' ' {
                let x = offset_x + ((col_index * block_size) as f32 * scale_x) as usize;
                let y = offset_y + ((row_index * block_size) as f32 * scale_y) as usize;

                let cell_width = (block_size as f32 * scale_x) as usize;
                let cell_height = (block_size as f32 * scale_y) as usize;

                for dx in 0..cell_width {
                    for dy in 0..cell_height {
                        let px = x + dx;
                        let py = y + dy;
                        if px < framebuffer.width as usize && py < framebuffer.height as usize {
                            // Color según el tipo de pared
                            let color = cell_to_color(cell);
                            framebuffer.set_current_color(color);
                            framebuffer.set_pixel(px as u32, py as u32);
                        }
                    }
                }
            }
        }
    }

    
    framebuffer.set_current_color(Color::WHITE);
   
    for thickness in 0..2 {
        for x in offset_x..offset_x + minimap_size {
            if x < framebuffer.width as usize {
                framebuffer.set_pixel(x as u32, (offset_y + thickness) as u32);
                framebuffer.set_pixel(x as u32, (offset_y + minimap_size - 1 - thickness) as u32);
            }
        }
        for y in offset_y..offset_y + minimap_size {
            if y < framebuffer.height as usize {
                framebuffer.set_pixel((offset_x + thickness) as u32, y as u32);
                framebuffer.set_pixel((offset_x + minimap_size - 1 - thickness) as u32, y as u32);
            }
        }
    }

   
    let px = offset_x as f32 + player.pos.x * scale_x;
    let py = offset_y as f32 + player.pos.y * scale_y;

    if px >= offset_x as f32
        && px < (offset_x + minimap_size) as f32
        && py >= offset_y as f32
        && py < (offset_y + minimap_size) as f32
    {
        framebuffer.set_current_color(Color::new(255, 215, 0, 255)); // Gold

        let arrow_size = 8.0;

        let tip_x = px + arrow_size * player.a.cos();
        let tip_y = py + arrow_size * player.a.sin();

        let base_angle1 = player.a + (3.0 * PI / 4.0);
        let base_angle2 = player.a - (3.0 * PI / 4.0);

        let base1_x = px + (arrow_size * 0.6) * base_angle1.cos();
        let base1_y = py + (arrow_size * 0.6) * base_angle1.sin();

        let base2_x = px + (arrow_size * 0.6) * base_angle2.cos();
        let base2_y = py + (arrow_size * 0.6) * base_angle2.sin();

        line(framebuffer, Vector2::new(px, py), Vector2::new(tip_x, tip_y));
        line(framebuffer, Vector2::new(tip_x, tip_y), Vector2::new(base1_x, base1_y));
        line(framebuffer, Vector2::new(tip_x, tip_y), Vector2::new(base2_x, base2_y));
    }
}

fn main() {
    let window_width = 1000; 
    let window_height = 650; 
    let block_size = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("NFL GAME")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    window.hide_cursor();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    let texture_manager = TextureManager::new();
    
    // <- SPRITES: Inicialización
    let mut sprite_manager = SpriteManager::new();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let welcome_sink = Sink::try_new(&stream_handle).unwrap();
    let intro_file = File::open("audio/intro.mp3").expect("No se pudo abrir intro.mp3");
    let intro_source = Decoder::new(BufReader::new(intro_file)).expect("No se pudo decodificar intro.mp3");
    welcome_sink.append(intro_source.repeat_infinite());
    welcome_sink.set_volume(0.5);
    welcome_sink.play();

    let mut game_sink: Option<Sink> = None;

    // <- SPRITES: Cargar maze y extraer posiciones de sprites
    let mut maze = load_maze("maze.txt");
    let sprite_positions = extract_sprite_positions(&maze, block_size);
    clean_maze(&mut maze); // Limpiar los '.' del maze
    sprite_manager.initialize_from_positions(&sprite_positions);

    let mut player = Player {
        pos: Vector2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0,
    };

    let mut mode = "3D";
    let mut textured = true;

    let welcome_texture = window
        .load_texture(&raylib_thread, "assets/welcome.png")
        .expect("No se pudo cargar welcome.png");
    
    // CARGAR TEXTURA DE VICTORIA
    let victory_texture = window
        .load_texture(&raylib_thread, "assets/victory.png")
        .expect("No se pudo cargar victory.png");
    
    let mut state = GameState::Welcome;

    // <- SPRITES: Variables para el tiempo
    let mut last_time = std::time::Instant::now();

    while !window.window_should_close() {
        framebuffer.clear();

        match state {
            GameState::Welcome => {
                let mut d = window.begin_drawing(&raylib_thread);
                d.clear_background(Color::BLACK);

                d.draw_texture_ex(
                    &welcome_texture,
                    Vector2::new(0.0, 0.0),
                    0.0,
                    window_width as f32 / welcome_texture.width as f32,
                    Color::WHITE,
                );

                if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    state = GameState::Playing;
                    welcome_sink.stop();

                    let sink = Sink::try_new(&stream_handle).unwrap();
                    let game_file = File::open("audio/song.mp3").expect("No se pudo abrir song.mp3");
                    let game_source = Decoder::new(BufReader::new(game_file)).expect("No se pudo decodificar song.mp3");
                    sink.append(game_source.repeat_infinite());
                    sink.set_volume(0.5);
                    sink.play();
                    game_sink = Some(sink);
                }
            }
            GameState::Playing => {
                // <- SPRITES: Actualizar tiempo y animaciones
                let current_time = std::time::Instant::now();
                let elapsed = current_time.duration_since(last_time).as_secs_f32();
                sprite_manager.update(elapsed);
                last_time = current_time;

                process_events(&mut player, &window, &maze, block_size);

                const MOUSE_SENSITIVITY: f32 = 0.005;
                let mouse_delta_x = window.get_mouse_delta().x;
                player.a += mouse_delta_x * MOUSE_SENSITIVITY;
                if player.a < 0.0 { player.a += 2.0 * PI; }
                else if player.a > 2.0 * PI { player.a -= 2.0 * PI; }

                if window.is_key_pressed(KeyboardKey::KEY_M) {
                    mode = if mode == "2D" { "3D" } else { "2D" };
                }

                if window.is_key_pressed(KeyboardKey::KEY_T) {
                    textured = !textured;
                }

                if window.is_key_pressed(KeyboardKey::KEY_P) {
                    if let Some(sink) = &game_sink {
                        if sink.is_paused() { sink.play(); } else { sink.pause(); }
                    }
                }

                // <- SPRITES: Verificar colisiones con sprites Y REPRODUCIR SONIDO DE TOUCHDOWN
                if let Some(_sprite_index) = sprite_manager.check_collision(&player, 30.0) {
                    println!("¡TOUCHDOWN! Balón recolectado! {}/{}", 
                             sprite_manager.get_collected_count(), 
                             sprite_manager.get_total_count());
                    
                    // NUEVO: Reproducir sonido de touchdown
                    if let Ok(touchdown_file) = File::open("audio/touchdown.mp3") {
                        if let Ok(touchdown_source) = Decoder::new(BufReader::new(touchdown_file)) {
                            if let Ok(effect_sink) = Sink::try_new(&stream_handle) {
                                effect_sink.append(touchdown_source);
                                effect_sink.set_volume(0.8);
                                effect_sink.play();
                                effect_sink.detach(); 
                            }
                        }
                    }

                    // VERIFICAR SI SE RECOLECTARON TODOS LOS BALONES
                    if sprite_manager.get_collected_count() == sprite_manager.get_total_count() {
                        println!("¡JUEGO COMPLETADO! Todos los balones recolectados!");
                        state = GameState::Victory;
                        
                        // Parar música del juego
                        if let Some(sink) = &game_sink {
                            sink.stop();
                        }
                        
                        // Reproducir música de victoria (Super Bowl)
                        if let Ok(victory_file) = File::open("audio/superbowl.mp3") {
                            if let Ok(victory_source) = Decoder::new(BufReader::new(victory_file)) {
                                let victory_sink = Sink::try_new(&stream_handle).unwrap();
                                victory_sink.append(victory_source.repeat_infinite()); // Loop infinito
                                victory_sink.set_volume(0.7);
                                victory_sink.play();
                                game_sink = Some(victory_sink); 
                            }
                        }
                    }
                }

                if mode == "2D" {
                    render_maze(&mut framebuffer, &maze, block_size, &player);
                    // <- SPRITES: Renderizar sprites en el minimapa durante modo 2D también
                    sprite_manager.render_minimap_sprites(&mut framebuffer, &maze, 200, block_size);
                } else {
                    if textured {
                        
                        render_world_with_textures_and_sprites(&mut framebuffer, &maze, block_size, &player, &texture_manager, &sprite_manager);
                    } else {
                        render_world(&mut framebuffer, &maze, block_size, &player);
                    }
                }

                render_minimap(&mut framebuffer, &maze, &player, 200, block_size);
                
                
                sprite_manager.render_minimap_sprites(&mut framebuffer, &maze, 200, block_size);

                framebuffer.swap_buffers(&mut window, &raylib_thread);
            }
            GameState::Victory => {
                // PANTALLA DE VICTORIA
                let mut d = window.begin_drawing(&raylib_thread);
                d.clear_background(Color::BLACK);

                d.draw_texture_ex(
                    &victory_texture,
                    Vector2::new(0.0, 0.0),
                    0.0,
                    window_width as f32 / victory_texture.width as f32,
                    Color::WHITE,
                );

                // Texto adicional sobre la imagen
                d.draw_text("Presiona ESC para salir", 
                           window_width / 2 - 100, 
                           window_height - 50, 
                           16, 
                           Color::WHITE);

                // Permitir salir con ESC
                if d.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    break; // Salir del loop principal
                }
            }
        }

        thread::sleep(Duration::from_millis(8));
    }

    if let Some(sink) = game_sink {
        sink.stop();
    }
}