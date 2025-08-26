# 🏃‍♂️ The Backrooms Escape

Un juego 3D de raycasting desarrollado en Rust que te sumerge en los inquietantes pasillos amarillos de los Backrooms. 

*Proyecto desarrollado para el curso de Gráficas por Computadora - Universidad del Valle de Guatemala* 🎓

## 🌟 ¿Qué es The Backrooms Escape?

Te has "noclipeado" fuera de la realidad y has caído en los **Backrooms** - espacios liminales infinitos llenos de pasillos amarillos, luces fluorescentes parpadeantes, y el constante zumbido de la electricidad barata. Tu única esperanza es encontrar las llaves escondidas y escapar antes de que algo te encuentre... 👻

### 🎮 Características del Juego

#### 🖥️ Motor Gráfico 3D
- **Ray casting en tiempo real** con renderizado de texturas
- **Sistema de iluminación dinámico** con atenuación por distancia
- **Múltiples tipos de paredes texturizadas**:
  - 🟡 Paredes amarillas - Los clásicos Backrooms
  - 🔴 Paredes rojas - Zonas de alta peligrosidad  
  - 🔵 Paredes azules - Áreas con propiedades especiales
  - 🟢 Paredes verdes - Zonas seguras
- **60 FPS estables** con optimizaciones de rendimiento

#### ✨ Sprites Animados
- 🗝️ **Llaves doradas** que rotan y brillan
- 📍 **Checkpoints** con efectos de pulsación luminosa
- 🌀 **Portales de salida** con animaciones hipnóticas
- 💜 **Vidas extra** con latidos como corazón
- ⚠️ **Trampas mortales** con patrones amenazantes

#### 🎵 Experiencia de Audio Inmersiva
- 🎶 **Música adaptativa** que cambia según el contexto
- 🔊 **Efectos de sonido realistas**:
  - Pasos que varían según la velocidad
  - Sonidos metálicos al recoger llaves
  - Efectos ominosos de trampas
  - Sonidos de victoria épicos
- 🎚️ **Control de volumen** en tiempo real

#### 🕹️ Controles Intuitivos
- **Movimiento fluido** con WASD o flechas
- **Control de cámara** con mouse para inmersión total
- **Sistema de colisiones** que previene glitches
- **Movimiento lateral** para navegación táctica

## 🗺️ Los Niveles

### 🟡 Nivel 1: "The Yellow Halls" 
*Dificultad: Principiante*
- Introducción gentil a los Backrooms
- 1 llave dorada para encontrar
- Diseño simple pero atmosférico

### 🔴 Nivel 2: "The Red Chambers"
*Dificultad: Intermedio* 
- Los pasillos se vuelven más peligrosos
- 2 llaves necesarias + 1 checkpoint obligatorio
- Primeras trampas mortales aparecen

### 🟣 Nivel 3: "The Final Escape"
*Dificultad: Avanzado*
- Tu última oportunidad de escape
- 3 llaves críticas + 2 checkpoints esenciales
- Máxima densidad de peligros

## 🎯 Mecánicas de Supervivencia

### 💖 Sistema de Vidas
- Empiezas con **3 vidas**
- Las trampas rojas te quitan una vida
- Encuentra corazones morados para recuperar vidas
- Sin vidas = Game Over 💀

### 🗝️ Recolección de Llaves
- Cada nivel requiere un número específico de llaves
- Las llaves doradas desbloquean la salida final
- Sin todas las llaves, no puedes escapar

### 📍 Checkpoints Estratégicos  
- Puntos de control obligatorios en niveles avanzados
- Cruces cian que debes visitar antes de la salida
- Estrategia requerida para completar niveles difíciles

## 🖥️ Interfaz de Usuario

### 📊 HUD Informativo
- **Contador de vidas** con código de colores
- **Progreso de llaves** actualizado en tiempo real  
- **Estado de la salida** (bloqueada/desbloqueada)
- **Contador de FPS** para monitoreo de rendimiento

### 🗺️ Minimapa Táctico
- Vista cenital del laberinto actual
- Tu posición y orientación en tiempo real
- Ubicación de objetos importantes
- Indispensable para navegación estratégica

### 💬 Notificaciones Dinámicas
- Alertas cuando recoges objetos
- Advertencias sobre requisitos no cumplidos
- Mensajes de estado del juego
- Sistema de fade-out automático

## 🛠️ Instalación y Ejecución

### Prerrequisitos
```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clonar el proyecto
git clone https://github.com/Emadlgg/proyecto1_raycasting.git
cd proyecto1
```

### 🚀 Ejecutar el Juego
```bash
# Modo desarrollo (con debug info)
cargo run

# Modo optimizado (recomendado)
cargo run --release
```

## 🎨 Stack Tecnológico

- **🦀 Rust** - Lenguaje principal (seguridad + rendimiento)
- **📐 Raylib** - Motor gráfico y manejo de entrada
- **🎵 Rodio** - Sistema de audio multi-stream
- **🎲 Rand** - Generación de variaciones procedurales

## 🏗️ Arquitectura del Código

El proyecto utiliza una **arquitectura modular** que separa responsabilidades:

- `caster.rs` - 🔍 Motor de ray casting y renderizado 3D
- `audio.rs` - 🎵 Sistema de audio y música dinámica  
- `sprite_manager.rs` - ✨ Gestión y animación de sprites
- `game_state.rs` - 🎮 Lógica de juego y progresión
- `player.rs` - 🚶‍♂️ Movimiento y controles del jugador
- `collision.rs` - 💥 Sistema de detección de colisiones
- `ui.rs` - 🖼️ Interfaz de usuario y HUD

## 🎓 Conceptos de Gráficas Implementados

- **Ray Casting**: Proyección de rayos para renderizado 3D
- **Transformaciones**: Matrices de rotación y traslación  
- **Texturas**: Sampling y mapping de coordenadas UV
- **Z-Buffering**: Ordenamiento correcto de profundidad
- **Rasterización**: Conversión de vectores a píxeles
- **Framebuffer**: Manipulación directa de píxeles

## 🚨 Survival Tips

1. **👀 Mantén la calma** - Los Backrooms desorientan fácilmente
2. **🗺️ Usa el minimapa** - Tu mejor herramienta de navegación
3. **🎧 Escucha atentamente** - El audio te dará pistas importantes
4. **⚡ Evita las trampas** - Una vida perdida puede ser crítica
5. **📍 Marca tu progreso** - Los checkpoints son obligatorios
6. **🗝️ No olvides las llaves** - Sin ellas, no hay escape

---

*"If you're not careful and noclip out of reality in the wrong areas, you'll end up in the Backrooms..."* 🌀

**¿Puedes escapar de la monotonía infinita?** 🏃‍♂️💨