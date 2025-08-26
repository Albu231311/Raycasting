// sprites.rs

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::maze::{Maze, SpritePosition};
use crate::player::Player;

#[derive(Clone, Debug)]
pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub texture_frames: Vec<Vec<Color>>,
    pub scale: f32,
    pub animation_frame: usize,
    pub animation_timer: f32,
    pub collected: bool,
}

#[derive(Clone, Debug)]
pub struct SpriteAnimation {
    pub frame_duration: f32,
    pub loop_animation: bool,
}

pub struct SpriteManager {
    pub sprites: Vec<Sprite>,
    pub animation: SpriteAnimation,
    last_frame_time: f32,
}

impl SpriteManager {
    pub fn new() -> Self {
        Self {
            sprites: Vec::new(),
            animation: SpriteAnimation {
                frame_duration: 1.0, 
                loop_animation: true,
            },
            last_frame_time: 0.0,
        }
    }

    pub fn initialize_from_positions(&mut self, positions: &[SpritePosition]) {
        self.sprites.clear();
        
        for position in positions {
            let sprite = Sprite {
                x: position.x,
                y: position.y,
                texture_frames: Self::create_football_frames(),
                scale: 0.5,
                animation_frame: 0,
                animation_timer: 0.0,
                collected: false,
            };
            self.sprites.push(sprite);
        }
    }

    fn create_football_frames() -> Vec<Vec<Color>> {
        let mut frames = Vec::new();
        
        // Frame 1 - Balón normal
        let mut texture_normal = vec![Color::new(0, 0, 0, 0); 32 * 32];
        
        for y in 0..32 {
            for x in 0..32 {
                let center_x = 16.0;
                let center_y = 16.0;
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                
                let ellipse = (dx * dx / 15.0 / 15.0) + (dy * dy / 10.0 / 10.0);
                
                if ellipse <= 1.0 {
                    // Color base normal del balón 
                    texture_normal[y * 32 + x] = Color::new(100, 50, 15, 255);
                } else if ellipse <= 1.2 {
                    // Borde oscuro
                    texture_normal[y * 32 + x] = Color::new(70, 35, 10, 255);
                }
            }
        }
        
        // Líneas del balón normales
        for x in 6..26 {
            if y_in_bounds(16) { texture_normal[16 * 32 + x] = Color::new(200, 200, 200, 255); }
            if y_in_bounds(10) { texture_normal[10 * 32 + x] = Color::new(200, 200, 200, 255); }
            if y_in_bounds(22) { texture_normal[22 * 32 + x] = Color::new(200, 200, 200, 255); }
        }
        
        for y in 12..21 {
            if x_in_bounds(8) { texture_normal[y * 32 + 8] = Color::new(200, 200, 200, 255); }
            if x_in_bounds(24) { texture_normal[y * 32 + 24] = Color::new(200, 200, 200, 255); }
        }

        // Frame 2 - Balón brillante 
        let mut texture_bright = vec![Color::new(0, 0, 0, 0); 32 * 32];
        
        for y in 0..32 {
            for x in 0..32 {
                let center_x = 16.0;
                let center_y = 16.0;
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                
                let ellipse = (dx * dx / 15.0 / 15.0) + (dy * dy / 10.0 / 10.0);
                
                if ellipse <= 1.0 {
                    // Color muy brillante del balón
                    texture_bright[y * 32 + x] = Color::new(220, 140, 80, 255);
                } else if ellipse <= 1.2 {
                    // Borde brillante
                    texture_bright[y * 32 + x] = Color::new(180, 110, 60, 255);
                }
            }
        }
        
        // Líneas del balón muy brillantes
        for x in 6..26 {
            if y_in_bounds(16) { texture_bright[16 * 32 + x] = Color::WHITE; }
            if y_in_bounds(10) { texture_bright[10 * 32 + x] = Color::WHITE; }
            if y_in_bounds(22) { texture_bright[22 * 32 + x] = Color::WHITE; }
        }
        
        for y in 12..21 {
            if x_in_bounds(8) { texture_bright[y * 32 + 8] = Color::WHITE; }
            if x_in_bounds(24) { texture_bright[y * 32 + 24] = Color::WHITE; }
        }

        // Frame 3 - Estado muy oscuro 
        let mut texture_dark = vec![Color::new(0, 0, 0, 0); 32 * 32];
        
        for y in 0..32 {
            for x in 0..32 {
                let center_x = 16.0;
                let center_y = 16.0;
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                
                let ellipse = (dx * dx / 15.0 / 15.0) + (dy * dy / 10.0 / 10.0);
                
                if ellipse <= 1.0 {
                    // Color muy oscuro del balón
                    texture_dark[y * 32 + x] = Color::new(60, 30, 8, 255);
                } else if ellipse <= 1.2 {
                    // Borde muy oscuro
                    texture_dark[y * 32 + x] = Color::new(40, 20, 5, 255);
                }
            }
        }
        
        // Líneas muy tenues
        for x in 6..26 {
            if y_in_bounds(16) { texture_dark[16 * 32 + x] = Color::new(120, 120, 120, 255); }
            if y_in_bounds(10) { texture_dark[10 * 32 + x] = Color::new(120, 120, 120, 255); }
            if y_in_bounds(22) { texture_dark[22 * 32 + x] = Color::new(120, 120, 120, 255); }
        }
        
        for y in 12..21 {
            if x_in_bounds(8) { texture_dark[y * 32 + 8] = Color::new(120, 120, 120, 255); }
            if x_in_bounds(24) { texture_dark[y * 32 + 24] = Color::new(120, 120, 120, 255); }
        }

        // Secuencia de parpadeo más lenta y notoria:
        // Normal -> Brillante -> Normal -> Muy Oscuro -> Normal -> Brillante
        frames.push(texture_normal.clone());    // 1 segundo normal
        frames.push(texture_bright.clone());    // 1 segundo muy brillante
        frames.push(texture_normal.clone());    // 1 segundo normal
        frames.push(texture_dark.clone());      // 1 segundo muy oscuro
        frames.push(texture_normal.clone());    // 1 segundo normal
        frames.push(texture_bright.clone());    // 1 segundo muy brillante

        frames
    }

    pub fn update(&mut self, delta_time: f32) {
        for sprite in &mut self.sprites {
            if !sprite.collected {
                sprite.animation_timer += delta_time;
                if sprite.animation_timer >= self.animation.frame_duration {
                    sprite.animation_timer = 0.0;
                    sprite.animation_frame = (sprite.animation_frame + 1) % sprite.texture_frames.len();
                }
            }
        }
    }

    pub fn render_sprites_3d(
        &self, 
        framebuffer: &mut Framebuffer, 
        player: &Player, 
        _maze: &Maze,
        _block_size: usize
    ) {
        // Crear z-buffer para evitar que sprites oculten paredes
        let z_buffer = vec![f32::INFINITY; (framebuffer.width as usize) * (framebuffer.height as usize)];

        // Ordenar sprites por distancia (más lejanos primero)
        let mut sprites_with_distance: Vec<(usize, f32)> = Vec::new();
        for (index, sprite) in self.sprites.iter().enumerate() {
            if !sprite.collected {
                let dx = sprite.x - player.pos.x;
                let dy = sprite.y - player.pos.y;
                let distance = (dx * dx + dy * dy).sqrt();
                sprites_with_distance.push((index, distance));
            }
        }

        sprites_with_distance.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        for (sprite_index, distance) in sprites_with_distance {
            let sprite = &self.sprites[sprite_index];
            self.render_single_sprite(framebuffer, sprite, player, &z_buffer, distance);
        }
    }

    fn render_single_sprite(
        &self,
        framebuffer: &mut Framebuffer,
        sprite: &Sprite,
        player: &Player,
        z_buffer: &[f32],
        distance: f32
    ) {
        if distance < 10.0 || distance > 800.0 { return; }

        let dx = sprite.x - player.pos.x;
        let dy = sprite.y - player.pos.y;

        // Transformar a coordenadas de cámara
        let cos_angle = player.a.cos();
        let sin_angle = player.a.sin();
        let transformed_x = dx * cos_angle + dy * sin_angle;
        let transformed_y = -dx * sin_angle + dy * cos_angle;
        
        if transformed_y <= 0.1 { return; } // Sprite detrás del jugador

        let hh = framebuffer.height as f32 / 2.0;
        let corrected_distance = transformed_y;
        let distance_to_projection_plane = 250.0;
        let sprite_height = (hh / corrected_distance) * distance_to_projection_plane;
        let wall_height_factor = 0.65;
        let adjusted_sprite_height = sprite_height * wall_height_factor * sprite.scale;

        // Calcular posición en pantalla
        let fov = player.fov;
        let rays_per_screen = framebuffer.width as f32;
        let angle_per_ray = fov / rays_per_screen;
        let sprite_angle = (transformed_x / transformed_y).atan();
        let screen_x = (framebuffer.width as f32 / 2.0) + (sprite_angle / angle_per_ray);

        let sprite_width = adjusted_sprite_height;
        let start_x = (screen_x - sprite_width / 2.0) as i32;
        let end_x = (screen_x + sprite_width / 2.0) as i32;
        let start_y = (hh - adjusted_sprite_height / 2.0).max(0.0) as i32;
        let end_y = (hh + adjusted_sprite_height / 2.0).min(framebuffer.height as f32) as i32;

        if start_x >= framebuffer.width as i32 || end_x < 0 ||
           start_y >= framebuffer.height as i32 || end_y < 0 { return; }

        let texture = &sprite.texture_frames[sprite.animation_frame];
        
        for screen_y in start_y.max(0)..end_y.min(framebuffer.height as i32) {
            for screen_x in start_x.max(0)..end_x.min(framebuffer.width as i32) {
                let buffer_index = screen_x as usize + screen_y as usize * framebuffer.width as usize;

                // Solo renderizar si el sprite está más cerca que lo que hay en el z-buffer
                if buffer_index < z_buffer.len() && corrected_distance < z_buffer[buffer_index] {
                    let tex_x = if end_x > start_x {
                        ((screen_x - start_x) * 32 / (end_x - start_x)).clamp(0, 31) as usize
                    } else { 16 };
                    let tex_y = if end_y > start_y {
                        ((screen_y - start_y) * 32 / (end_y - start_y)).clamp(0, 31) as usize
                    } else { 16 };

                    let pixel = texture[tex_y * 32 + tex_x];
                    if pixel.a > 0 { // Solo píxeles no transparentes
                        framebuffer.set_current_color(pixel);
                        framebuffer.set_pixel(screen_x as u32, screen_y as u32);
                    }
                }
            }
        }
    }

    pub fn render_minimap_sprites(
        &self, 
        framebuffer: &mut Framebuffer,
        _maze: &Maze,
        minimap_size: usize,
        block_size: usize
    ) {
        let map_cols = 15;
        let map_rows = 11;

        let scale_x = minimap_size as f32 / (map_cols * block_size) as f32;
        let scale_y = minimap_size as f32 / (map_rows * block_size) as f32;

        let offset_x = framebuffer.width as usize - minimap_size - 10;
        let offset_y = 10;

        for sprite in &self.sprites {
            if sprite.collected { continue; }

            let sprite_minimap_x = offset_x + (sprite.x * scale_x) as usize;
            let sprite_minimap_y = offset_y + (sprite.y * scale_y) as usize;

            framebuffer.set_current_color(Color::GOLD);
            let radius = 4;
            for dy in -(radius as i32)..=(radius as i32) {
                for dx in -(radius as i32)..=(radius as i32) {
                    if dx * dx + dy * dy <= radius * radius {
                        let x = (sprite_minimap_x as i32 + dx) as u32;
                        let y = (sprite_minimap_y as i32 + dy) as u32;
                        if x < framebuffer.width && y < framebuffer.height {
                            framebuffer.set_pixel(x, y);
                        }
                    }
                }
            }
        }
    }

    pub fn check_collision(&mut self, player: &Player, collision_distance: f32) -> Option<usize> {
        for (i, sprite) in self.sprites.iter_mut().enumerate() {
            if !sprite.collected {
                let dx = sprite.x - player.pos.x;
                let dy = sprite.y - player.pos.y;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance < collision_distance {
                    sprite.collected = true;
                     return Some(i);
                }
            }
        }
        None
    }

    pub fn get_collected_count(&self) -> usize {
        self.sprites.iter().filter(|s| s.collected).count()
    }

    pub fn get_total_count(&self) -> usize {
        self.sprites.len()
    }
}

// Funciones auxiliares
fn x_in_bounds(x: usize) -> bool {
    x < 32
}

fn y_in_bounds(y: usize) -> bool {
    y < 32
}