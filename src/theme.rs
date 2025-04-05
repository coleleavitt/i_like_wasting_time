use crate::data::JobStatus;
use iced::{Background, Color};

// --- Kraken-inspired Color Palette ---
pub fn kraken_background_gradient() -> Background {
    Background::Color(kraken_background())
}

pub fn kraken_background() -> Color {
    Color::from_rgb(0.078, 0.086, 0.102) // Deep blue-black
}

pub fn kraken_card_bg() -> Color {
    Color::from_rgb(0.098, 0.106, 0.122) // Slightly lighter blue-black
}

pub fn kraken_header_bg() -> Color {
    Color::from_rgb(0.055, 0.063, 0.075) // Darker blue-black for header
}

pub fn kraken_text() -> Color {
    Color::from_rgb(0.88, 0.90, 0.92) // Crisp white
}

pub fn kraken_secondary_text() -> Color {
    Color::from_rgb(0.63, 0.65, 0.67) // Gray for less important text
}

pub fn kraken_highlight() -> Color {
    Color::from_rgb(0.129, 0.737, 0.514) // Kraken's green accent
}

pub fn kraken_highlight_hover() -> Color {
    Color::from_rgb(0.169, 0.847, 0.584) // Brighter green for hover
}

pub fn kraken_highlight_subtle() -> Color {
    Color::from_rgba(0.129, 0.737, 0.514, 0.12) // Very subtle green for backgrounds
}

pub fn kraken_negative() -> Color {
    Color::from_rgb(0.949, 0.267, 0.267) // Red for negative outcomes
}

pub fn kraken_negative_dark() -> Color {
    Color::from_rgb(0.649, 0.137, 0.137) // Darker red for rejected jobs
}

pub fn kraken_warning() -> Color {
    Color::from_rgb(0.945, 0.769, 0.059) // Yellow for warnings/edit actions
}

pub fn kraken_border() -> Color {
    Color::from_rgb(0.149, 0.169, 0.204) // Subtle borders
}

pub fn kraken_card_border() -> Color {
    Color::from_rgb(0.169, 0.189, 0.224) // Slightly brighter card borders
}

// --- Status Colors ---
pub fn status_color(status: JobStatus) -> Color {
    match status {
        JobStatus::Applied => Color::from_rgb(0.22, 0.51, 0.78),      // Blue
        JobStatus::OA => Color::from_rgb(0.90, 0.62, 0.0),            // Orange
        JobStatus::Interview => Color::from_rgb(0.129, 0.737, 0.514), // Kraken Green
        JobStatus::Rejected => Color::from_rgb(0.949, 0.267, 0.267),  // Kraken Red
        JobStatus::Offer => Color::from_rgb(0.608, 0.349, 0.714),     // Purple
        JobStatus::Accepted => Color::from_rgb(0.129, 0.737, 0.514),  // Kraken Green
        JobStatus::Withdrawn => Color::from_rgb(0.5, 0.5, 0.5),       // Gray
    }
}
