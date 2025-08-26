//texture.rs
use image::GenericImageView;

pub struct ImageTexture {
    pub data: Vec<Vec<[u8; 3]>>,
    pub width: usize,
    pub height: usize,
}

impl ImageTexture {
    pub fn from_file(file_path: &str) -> Self {
        match image::open(file_path) {
            Ok(img) => {
                let img = img.to_rgb8();
                let (width, height) = img.dimensions();
                let mut data = vec![vec![[0, 0, 0]; width as usize]; height as usize];
                
                for y in 0..height {
                    for x in 0..width {
                        let pixel = img.get_pixel(x, y);
                        data[y as usize][x as usize] = [pixel[0], pixel[1], pixel[2]];
                    }
                }
                
                Self { data, width: width as usize, height: height as usize }
            },
            Err(e) => {
                println!("Error cargando {}: {}", file_path, e);
                // Textura de fallback más visible
                let size = 64;
                let mut data = vec![vec![[255, 0, 255]; size]; size]; 
                for y in 0..size {
                    for x in 0..size {
                        if (x/8 + y/8) % 2 == 0 {
                            data[y][x] = [255, 255, 0]; 
                        }
                    }
                }
                Self { data, width: size, height: size }
            }
        }
    }
    
    pub fn get_color(&self, tex_x: f32, tex_y: f32) -> [u8; 3] {
        
        let tex_x = tex_x.clamp(0.0, 1.0);
        let tex_y = tex_y.clamp(0.0, 1.0);
        
        let x = ((tex_x * (self.width - 1) as f32) as usize).min(self.width - 1);
        let y = ((tex_y * (self.height - 1) as f32) as usize).min(self.height - 1);
        
        self.data[y][x]
    }
}

pub struct TextureManager {
    pub stadium_texture: ImageTexture,
    pub grass_texture: ImageTexture, 
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            stadium_texture: ImageTexture::from_file("assets/stadium.png"),
            grass_texture: ImageTexture::from_file("assets/grass.png"),  
        }
    }
    
    pub fn get_texture(&self, _wall_char: char) -> &ImageTexture {
        // Usar stadium.png para todas las paredes
        &self.stadium_texture
    }
    
    
    pub fn get_grass_texture(&self) -> &ImageTexture {
        &self.grass_texture
    }
}