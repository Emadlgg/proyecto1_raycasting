// notification.rs - Sistema de notificaciones

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub color: Color,
    pub duration: f32,
    pub remaining_time: f32,
    pub notification_type: NotificationType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotificationType {
    Info,
    Warning,
    Error,
    Success,
    Special,
}

impl NotificationType {
    pub fn get_color(&self) -> Color {
        match self {
            NotificationType::Info => Color::new(100, 150, 255, 255),
            NotificationType::Warning => Color::new(255, 200, 50, 255),
            NotificationType::Error => Color::new(255, 80, 80, 255),
            NotificationType::Success => Color::new(100, 255, 100, 255),
            NotificationType::Special => Color::new(200, 100, 255, 255),
        }
    }
}

impl Notification {
    pub fn new(message: String, notification_type: NotificationType, duration: f32) -> Self {
        Notification {
            message,
            color: notification_type.get_color(),
            duration,
            remaining_time: duration,
            notification_type,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
        self.remaining_time -= delta_time;
        self.remaining_time > 0.0
    }

    pub fn get_alpha(&self) -> f32 {
        let fade_time = 1.0; // Últimos 1 segundo con fade
        if self.remaining_time <= fade_time {
            self.remaining_time / fade_time
        } else {
            1.0
        }
    }
}

#[derive(Clone)]
pub struct NotificationManager {
    notifications: VecDeque<Notification>,
    max_notifications: usize,
}

impl NotificationManager {
    pub fn new() -> Self {
        NotificationManager {
            notifications: VecDeque::new(),
            max_notifications: 5, // Máximo 5 notificaciones en pantalla
        }
    }

    pub fn add_notification(&mut self, message: &str, notification_type: NotificationType, duration: f32) {
        let notification = Notification::new(message.to_string(), notification_type, duration);
        
        // Remover notificaciones más antiguas si hay demasiadas
        while self.notifications.len() >= self.max_notifications {
            self.notifications.pop_back();
        }
        
        self.notifications.push_front(notification);
    }

    // Funciones específicas para diferentes tipos de eventos
    pub fn show_life_lost(&mut self, lives_remaining: i32) {
        let message = if lives_remaining > 0 {
            format!("VIDA PERDIDA! {} VIDAS RESTANTES", lives_remaining)
        } else {
            "GAME OVER - NO TE QUEDAN MAS VIDAS!".to_string()
        };
        self.add_notification(&message, NotificationType::Error, 3.0);
    }

    pub fn show_key_collected(&mut self, keys_collected: i32, keys_needed: i32) {
        if keys_collected >= keys_needed {
            self.add_notification("JUNTASTE TODAS LAS LLAVES, PORTAL DESBLOQUEADO!", NotificationType::Success, 3.0);
        } else {
            let message = format!("LLAVE ENCONTRADA! ({}/{})", keys_collected, keys_needed);
            self.add_notification(&message, NotificationType::Success, 2.5);
        }
    }

    pub fn show_checkpoint_reached(&mut self, checkpoints: usize, level: usize) {
        let message = format!("CHECKPOINT ALCANZADO! ({}/{})", checkpoints, level.saturating_sub(1));
        self.add_notification(&message, NotificationType::Special, 2.5);
    }

    pub fn show_exit_blocked(&mut self, reason: &str) {
        let message = match reason {
            "no_key" => "SALIDA BLOQUEADA - ENCUENTRA LAS LLAVES PRIMERO!",
            "no_checkpoints" => "SALIDA BLOQUEADA - CHECKPOINTS INCOMPLETOS!",
            "missing_both" => "SALIDA BLOQUEADA - SE NECESITA LLAVES Y CHECKPOINTS!",
            _ => "SALIDA BLOQUEADA - RTE FALTA COMPLETAR OBJETIVOS!",
        };
        self.add_notification(message, NotificationType::Warning, 3.0);
    }

    pub fn show_extra_life(&mut self, total_lives: i32) {
        let message = format!("VIDA EXTRA! TOTAL: {}", total_lives);
        self.add_notification(&message, NotificationType::Success, 2.5);
    }

    pub fn show_trap_activated(&mut self) {
        self.add_notification("TRAMPA ACTIVADA!", NotificationType::Error, 2.0);
    }

    pub fn update(&mut self, delta_time: f32) {
        // Actualizar todas las notificaciones y remover las expiradas
        self.notifications.retain_mut(|notification| {
            notification.update(delta_time)
        });
    }

    pub fn render(&self, framebuffer: &mut Framebuffer) {
        let notification_height = 25;
        let notification_width = 400;
        let start_x = (framebuffer.width - notification_width) / 2;
        let start_y = 50;

        for (i, notification) in self.notifications.iter().enumerate() {
            let y_offset = i as u32 * (notification_height + 5);
            let notification_y = start_y + y_offset;

            if notification_y + notification_height > framebuffer.height {
                break; // No renderizar fuera de pantalla
            }

            self.render_notification(framebuffer, notification, start_x, notification_y, notification_width, notification_height);
        }
    }

    fn render_notification(
        &self,
        framebuffer: &mut Framebuffer,
        notification: &Notification,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) {
        let alpha = notification.get_alpha();
        
        // Fondo de la notificación con transparencia
        let bg_color = Color::new(0, 0, 0, (150.0 * alpha) as u8);
        framebuffer.set_current_color(bg_color);
        for py in y..(y + height) {
            for px in x..(x + width) {
                if px < framebuffer.width && py < framebuffer.height {
                    framebuffer.set_pixel(px, py);
                }
            }
        }

        // Borde de la notificación
        let border_color = Color::new(
            notification.color.r,
            notification.color.g,
            notification.color.b,
            (255.0 * alpha) as u8,
        );
        self.render_border(framebuffer, x, y, width, height, border_color);

        // Texto de la notificación
        let text_color = Color::new(
            notification.color.r,
            notification.color.g,
            notification.color.b,
            (255.0 * alpha) as u8,
        );
        framebuffer.set_current_color(text_color);
        
        // Centrar el texto en la notificación
        let text_x = x + 10;
        let text_y = y + (height - 7) / 2; // 7 es la altura de la fuente
        crate::ui::render_text(framebuffer, &notification.message, text_x, text_y);
    }

    fn render_border(&self, framebuffer: &mut Framebuffer, x: u32, y: u32, width: u32, height: u32, color: Color) {
        framebuffer.set_current_color(color);
        
        // Líneas horizontales
        for px in x..=(x + width) {
            if px < framebuffer.width {
                if y < framebuffer.height {
                    framebuffer.set_pixel(px, y);
                }
                if y + height < framebuffer.height {
                    framebuffer.set_pixel(px, y + height);
                }
            }
        }
        
        // Líneas verticales
        for py in y..=(y + height) {
            if py < framebuffer.height {
                if x < framebuffer.width {
                    framebuffer.set_pixel(x, py);
                }
                if x + width < framebuffer.width {
                    framebuffer.set_pixel(x + width, py);
                }
            }
        }
    }

    pub fn clear_all(&mut self) {
        self.notifications.clear();
    }
}