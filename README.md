# NFL Raycasting - Laberinto 3D Temático de Fútbol Americano

Un juego de laberinto 3D desarrollado en Rust utilizando técnicas de raycasting, ambientado en el mundo del fútbol americano de la NFL.

## Descripción del Proyecto

Este proyecto implementa un motor de raycasting desde cero en Rust para crear una experiencia de laberinto 3D inmersiva. El jugador debe navegar por un estadio laberíntico recolectando balones de fútbol americano que parpadean con animaciones, todo mientras explora un entorno renderizado en tiempo real con texturas temáticas de la NFL.

### Características Principales

- **Motor de Raycasting Personalizado**: Implementación completa de raycasting para renderizado 3D en tiempo real
- **Texturas Temáticas**: Paredes con texturas de estadio y suelo de césped realista
- **Sistema de Sprites Animados**: Balones de fútbol americano con animaciones de parpadeo
- **Audio Inmersivo**: Música de introducción, gameplay y efectos de sonido de touchdown
- **Minimapa Funcional**: Sistema de navegación con vista cenital del laberinto
- **Detección de Colisiones**: Sistema robusto de colisiones para paredes y objetos coleccionables
- **Estados de Juego**: Pantalla de bienvenida, gameplay y pantalla de victoria
- **Controles Intuitivos**: Movimiento WASD + ratón para rotación de cámara

### Tecnologías Utilizadas

- **Lenguaje**: Rust
- **Gráficos**: Raylib-rs para manejo de ventanas y renderizado base
- **Audio**: Rodio para reproducción de música y efectos de sonido
- **Procesamiento de Imágenes**: Image crate para carga de texturas
- **Matemáticas**: Implementación personalizada de algoritmos trigonométricos para raycasting

### Mecánicas de Juego

- **Objetivo**: Recolectar todos los balones de fútbol americano dispersos por el laberinto
- **Progresión**: El juego termina cuando se recolectan todos los balones
- **Feedback Visual**: Los balones parpadean para mayor visibilidad
- **Feedback Audio**: Sonidos de touchdown al recolectar balones

### Algoritmos Implementados

- **Raycasting**: Algoritmo de lanzamiento de rayos para renderizado 3D pseudo-realista
- **Z-Buffer**: Sistema de profundidad para renderizado correcto de sprites
- **Detección de Colisiones**: Algoritmos de intersección para movimiento del jugador y recolección de objetos
- **Mapeo de Texturas**: Aplicación de texturas en tiempo real a superficies 3D

## Requisitos del Sistema

- **Rust**: Versión 1.70 o superior
- **Cargo**: Incluido con Rust
- **Sistema Operativo**: Windows, macOS o Linux
- **Archivos de Assets**: El proyecto incluye texturas, audio y mapas necesarios

## Pasos para Ejecutar

1. **Clonar el repositorio**
```bash
git clone https://github.com/Albu231311/Raycasting.git
```

2. **Navegar al directorio del proyecto**
```bash
cd Raycasting
```

3. **Ejecutar el programa**
```bash
cargo run --release
```

## Controles

- **W/A/S/D**: Movimiento (adelante/izquierda/atrás/derecha)
- **Ratón**: Rotación de cámara
- **Flechas Izquierda/Derecha**: Rotación y movimiento alternativa
- **T**: Alternar texturas
- **P**: Pausar/reproducir música
- **Enter**: Iniciar juego (desde pantalla de bienvenida)
- **ESC**: Salir del juego (desde pantalla de victoria)
