// audio.rs - Sistema de audio optimizado

use rodio::{Decoder, OutputStream, Sink, Source};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct AudioManager {
    _stream: OutputStream,
    stream_handle: rodio::OutputStreamHandle,
    music_sink: Arc<Mutex<Option<Sink>>>,
    music_playing: bool,
    music_volume: f32,
    current_music_type: Option<MusicType>,
    sfx_volume: f32,
    sound_data: HashMap<String, Vec<u8>>,
    footsteps_timer: f32,
    footsteps_interval: f32,
    is_walking: bool,
    last_footstep_time: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MusicType {
    Menu,
    Background,
}

#[derive(Debug, Clone, Copy)]
pub enum GameAudioEvent {
    KeyPickup,
    TrapTriggered,
    PlayerHurt,
    CheckpointReached,
    LevelComplete,
}

impl AudioManager {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default()
            .expect("Failed to initialize audio output stream");
        
        let mut audio_manager = AudioManager {
            _stream,
            stream_handle,
            music_sink: Arc::new(Mutex::new(None)),
            music_playing: false,
            music_volume: 0.2,
            current_music_type: None,
            sfx_volume: 0.8,
            sound_data: HashMap::new(),
            footsteps_timer: 0.0,
            footsteps_interval: 0.4,
            is_walking: false,
            last_footstep_time: Instant::now(),
        };
        
        audio_manager.load_all_audio();
        audio_manager
    }

    fn load_all_audio(&mut self) {
        // Cargar música de fondo
        self.load_audio_file("menu_music", "assets/sounds/music/menu_sound.ogg");
        self.load_audio_file("background_music", "assets/sounds/music/background_ambience.ogg");
        
        // Cargar efectos de sonido
        let sound_files = [
            ("key_pickup", "assets/sounds/sfx/key_pickup.ogg"),
            ("footsteps", "assets/sounds/sfx/footsteps.ogg"),
            ("trap_trigger", "assets/sounds/sfx/trap_trigger.ogg"),
            ("player_hurt", "assets/sounds/sfx/player_hurt.ogg"),
            ("portal_reached", "assets/sounds/sfx/portal_reached.ogg"),
            ("level_complete", "assets/sounds/sfx/level_complete.ogg"),
        ];

        for (name, path) in sound_files.iter() {
            self.load_audio_file(name, path);
        }
    }

    fn load_audio_file(&mut self, name: &str, path: &str) {
        match std::fs::read(path) {
            Ok(data) => {
                self.sound_data.insert(name.to_string(), data);
            }
            Err(_) => {
                // Crear datos silenciosos como fallback
                self.sound_data.insert(name.to_string(), vec![0; 1024]);
            }
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Actualizar timer de pasos
        if self.is_walking {
            self.footsteps_timer += delta_time;
            if self.footsteps_timer >= self.footsteps_interval {
                self.play_footstep_sound();
                self.footsteps_timer = 0.0;
            }
        } else {
            self.footsteps_timer = 0.0;
        }

        // Verificar si la música de fondo necesita reiniciarse (loop)
        if self.music_playing {
            let music_sink = self.music_sink.clone();
            if let Ok(sink_option) = music_sink.try_lock() {
                if let Some(ref sink) = *sink_option {
                    if sink.empty() {
                        drop(sink_option);
                        self.restart_current_music();
                    }
                }
            };
        }
    }

    pub fn play_menu_music(&mut self) {
        self.play_music("menu_music", MusicType::Menu);
    }

    pub fn play_background_music(&mut self) {
        self.play_music("background_music", MusicType::Background);
    }

    fn play_music(&mut self, music_key: &str, music_type: MusicType) {
        // Si ya está sonando la misma música, no hacer nada
        if self.music_playing && self.current_music_type == Some(music_type) {
            return;
        }

        self.stop_background_music();

        if let Some(music_data) = self.sound_data.get(music_key) {
            let cursor = Cursor::new(music_data.clone());
            
            if let Ok(source) = Decoder::new(cursor) {
                if let Ok(sink) = Sink::try_new(&self.stream_handle) {
                    sink.set_volume(self.music_volume);
                    sink.append(source);
                    
                    *self.music_sink.lock().unwrap() = Some(sink);
                    self.music_playing = true;
                    self.current_music_type = Some(music_type);
                }
            }
        }
    }

    fn restart_current_music(&mut self) {
        if !self.music_playing || self.current_music_type.is_none() {
            return;
        }

        let music_key = match self.current_music_type.unwrap() {
            MusicType::Menu => "menu_music",
            MusicType::Background => "background_music",
        };

        if let Some(music_data) = self.sound_data.get(music_key) {
            let cursor = Cursor::new(music_data.clone());
            
            if let Ok(source) = Decoder::new(cursor) {
                if let Ok(sink_option) = self.music_sink.try_lock() {
                    if let Some(ref sink) = *sink_option {
                        sink.set_volume(self.music_volume);
                        sink.append(source);
                    }
                }
            }
        }
    }

    pub fn stop_background_music(&mut self) {
        if let Ok(mut sink_option) = self.music_sink.lock() {
            if let Some(sink) = sink_option.take() {
                sink.stop();
            }
        }
        self.music_playing = false;
        self.current_music_type = None;
    }

    // Pausar música temporalmente para SFX importantes
    pub fn stop_music_for_sfx(&mut self) {
        if self.music_playing {
            if let Ok(sink_option) = self.music_sink.try_lock() {
                if let Some(ref sink) = *sink_option {
                    sink.pause();
                }
            }
        }
    }

    // Reanudar música después de SFX importantes
    pub fn resume_music_after_sfx(&mut self) {
        if self.music_playing {
            if let Ok(sink_option) = self.music_sink.try_lock() {
                if let Some(ref sink) = *sink_option {
                    sink.play();
                }
            }
        }
    }

    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
        
        if let Ok(sink_option) = self.music_sink.try_lock() {
            if let Some(ref sink) = *sink_option {
                sink.set_volume(self.music_volume);
            }
        }
    }

    pub fn play_sound_effect(&self, sound_name: &str) {
        if let Some(sound_data) = self.sound_data.get(sound_name) {
            let cursor = Cursor::new(sound_data.clone());
            
            if let Ok(source) = Decoder::new(cursor) {
                // Crear un nuevo sink temporal para cada SFX
                if let Ok(temp_sink) = Sink::try_new(&self.stream_handle) {
                    temp_sink.set_volume(self.sfx_volume);
                    temp_sink.append(source);
                    temp_sink.detach(); // Importante: dejar que se reproduzca independientemente
                }
            }
        }
    }

    // SFX importante que detiene música temporalmente
    pub fn play_important_sfx(&mut self, sound_name: &str) {
        self.stop_music_for_sfx();
        self.play_sound_effect(sound_name);
        
        // Programar reanudación de música después de un delay
        let music_sink = self.music_sink.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(3));
            if let Ok(sink_option) = music_sink.try_lock() {
                if let Some(ref sink) = *sink_option {
                    sink.play();
                }
            }
        });
    }

    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
    }

    // Funciones específicas de eventos del juego
    pub fn play_key_pickup_sound(&self) {
        self.play_sound_effect("key_pickup");
    }

    pub fn play_trap_sound(&self) {
        self.play_sound_effect("trap_trigger");
    }

    pub fn play_damage_sound(&self) {
        self.play_sound_effect("player_hurt");
    }

    pub fn play_portal_sound(&self) {
        self.play_sound_effect("portal_reached");
    }

    // SFX de victoria que detiene la música
    pub fn play_victory_sound(&mut self) {
        self.stop_music_for_sfx();
        self.play_sound_effect("level_complete");
    }

    // Reanudar música del juego después de victoria
    pub fn resume_game_music_after_victory(&mut self) {
        std::thread::spawn({
            let music_sink = self.music_sink.clone();
            let stream_handle = self.stream_handle.clone();
            let music_volume = self.music_volume;
            let sound_data = self.sound_data.get("background_music").cloned();
            
            move || {
                std::thread::sleep(Duration::from_secs(4));
                
                if let Some(music_data) = sound_data {
                    let cursor = Cursor::new(music_data);
                    
                    if let Ok(source) = Decoder::new(cursor) {
                        if let Ok(new_sink) = Sink::try_new(&stream_handle) {
                            new_sink.set_volume(music_volume);
                            new_sink.append(source);
                            
                            if let Ok(mut sink_option) = music_sink.lock() {
                                *sink_option = Some(new_sink);
                            }
                        }
                    }
                }
            }
        });
    }

    // Sistema de pasos mejorado
    pub fn start_walking(&mut self) {
        if !self.is_walking {
            self.is_walking = true;
            self.footsteps_timer = 0.0;
            self.last_footstep_time = Instant::now();
        }
    }

    pub fn stop_walking(&mut self) {
        self.is_walking = false;
        self.footsteps_timer = 0.0;
    }

    fn play_footstep_sound(&mut self) {
        // Evitar spam de pasos
        if self.last_footstep_time.elapsed() < Duration::from_millis(300) {
            return;
        }
        
        self.last_footstep_time = Instant::now();

        if let Some(footstep_data) = self.sound_data.get("footsteps") {
            let cursor = Cursor::new(footstep_data.clone());
            
            if let Ok(source) = Decoder::new(cursor) {
                // Variar pitch ligeramente para más realismo
                let pitch_variation = 0.9 + (rand::random::<f32>() * 0.2);
                let volume_variation = self.sfx_volume * 0.4; // Pasos más suaves
                
                if let Ok(temp_sink) = Sink::try_new(&self.stream_handle) {
                    // Truncar el sonido a solo 1-2 segundos
                    let adjusted_source = source
                        .amplify(volume_variation)
                        .speed(pitch_variation)
                        .take_duration(Duration::from_millis(1200));
                    
                    temp_sink.append(adjusted_source);
                    temp_sink.detach();
                }
            }
        }
    }

    // Funciones de utilidad
    pub fn is_music_playing(&self) -> bool {
        self.music_playing
    }

    pub fn get_current_music_type(&self) -> Option<MusicType> {
        self.current_music_type
    }

    pub fn get_music_volume(&self) -> f32 {
        self.music_volume
    }

    pub fn get_sfx_volume(&self) -> f32 {
        self.sfx_volume
    }

    // Control general de audio
    pub fn pause_all(&mut self) {
        if let Ok(sink_option) = self.music_sink.try_lock() {
            if let Some(ref sink) = *sink_option {
                sink.pause();
            }
        }
        self.stop_walking();
    }

    pub fn resume_all(&mut self) {
        if let Ok(sink_option) = self.music_sink.try_lock() {
            if let Some(ref sink) = *sink_option {
                sink.play();
            }
        }
    }

    // Para usar en game_state.rs cuando el jugador se mueve
    pub fn handle_player_movement(&mut self, is_moving: bool) {
        if is_moving {
            self.start_walking();
        } else {
            self.stop_walking();
        }
    }

    // Para eventos del juego
    pub fn play_game_event(&mut self, event: GameAudioEvent) {
        match event {
            GameAudioEvent::KeyPickup => self.play_key_pickup_sound(),
            GameAudioEvent::TrapTriggered => self.play_trap_sound(),
            GameAudioEvent::PlayerHurt => self.play_damage_sound(),
            GameAudioEvent::CheckpointReached => self.play_key_pickup_sound(),
            GameAudioEvent::LevelComplete => {
                self.play_victory_sound();
                self.schedule_music_resume();
            }
        }
    }

    // Programar reanudación de música después de victoria
    fn schedule_music_resume(&mut self) {
        let music_sink = self.music_sink.clone();
        let stream_handle = self.stream_handle.clone();
        let music_volume = self.music_volume;
        let sound_data = self.sound_data.get("background_music").cloned();
        
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(4));
            
            if let Some(music_data) = sound_data {
                let cursor = Cursor::new(music_data);
                
                if let Ok(source) = Decoder::new(cursor) {
                    if let Ok(new_sink) = Sink::try_new(&stream_handle) {
                        new_sink.set_volume(music_volume);
                        new_sink.append(source);
                        
                        if let Ok(mut sink_option) = music_sink.lock() {
                            *sink_option = Some(new_sink);
                        }
                    }
                }
            }
        });
    }

    // Forzar reanudación inmediata de música de juego
    pub fn resume_background_music_now(&mut self) {
        if !self.music_playing || self.current_music_type != Some(MusicType::Background) {
            self.play_background_music();
        }
    }
}

impl Drop for AudioManager {
    fn drop(&mut self) {
        self.stop_background_music();
    }
}