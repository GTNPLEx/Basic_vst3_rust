use nih_plug_iced::widget::svg;
use nih_plug_iced::Color;
use std::collections::HashMap;
use std::sync::OnceLock;

// Store SVG handles globally so they're only created once
static SVG_ASSETS: OnceLock<HashMap<String, svg::Handle>> = OnceLock::new();

/// Get an SVG handle by name
pub fn get_svg(name: &str) -> svg::Handle {
    let assets = SVG_ASSETS.get_or_init(|| {
        let mut assets = HashMap::new();
        
        // Load all SVG assets
        assets.insert("mono".to_string(), load_mono_svg());
        assets.insert("stereo".to_string(), load_stereo_svg());
        assets.insert("mute".to_string(), load_mute_svg());
        assets.insert("bypass".to_string(), load_bypass_svg());
        
        assets
    });
    
    // Return a clone of the handle or a placeholder if not found
    assets.get(name)
        .cloned()
        .unwrap_or_else(|| load_placeholder_svg())
}

// Button status - can be used to toggle LED states
pub enum ButtonStatus {
    Active,
    Inactive
}

// Function to generate SVG with different LED states
pub fn get_svg_with_status(name: &str, status: ButtonStatus) -> svg::Handle {
    match name {
        "mono" => load_mono_svg_with_status(status),
        "stereo" => load_stereo_svg_with_status(status),
        "mute" => load_mute_svg_with_status(status),
        "bypass" => load_bypass_svg_with_status(status),
        _ => load_placeholder_svg(),
    }
}

// Helper function for Mono button with toggled LED state
fn load_mono_svg_with_status(status: ButtonStatus) -> svg::Handle {
    let opacity = match status {
        ButtonStatus::Active => "0.8",
        ButtonStatus::Inactive => "0.2",
    };
    
    svg::Handle::from_memory(format!(r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 80 60">
          <!-- Button background -->
          <rect x="0" y="0" width="80" height="60" rx="5" fill="\#2a2a2a" />
          <!-- Button surface -->
          <rect x="5" y="5" width="70" height="50" rx="4" fill="url(#buttonGradient)" />
          <!-- LED Indicator -->
          <circle cx="15" cy="15" r="4" fill="\#33ff33" fill-opacity="{}" />
          <!-- Label -->
          <text x="40" y="35" font-family="Arial" font-size="12" fill="\#e0e0e0" text-anchor="middle">MONO</text>
          <!-- Gradients -->
          <defs>
            <!-- Normal Button Gradient -->
            <linearGradient id="buttonGradient" x1="0%" y1="0%" x2="0%" y2="100%">
              <stop offset="0%" stop-color="\#4a4a4a" />
              <stop offset="100%" stop-color="\#3a3a3a" />
            </linearGradient>
          </defs>
        </svg>
    "#, opacity).as_bytes().to_vec())
}

// Base SVG loading functions
fn load_mono_svg() -> svg::Handle {
    svg::Handle::from_memory(r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 80 60">
          <!-- Button background -->
          <rect x="0" y="0" width="80" height="60" rx="5" fill="\#2a2a2a" />
          <!-- Button surface -->
          <rect x="5" y="5" width="70" height="50" rx="4" fill="url(\#buttonGradient)" />
          <!-- LED Indicator -->
          <circle cx="15" cy="15" r="4" fill="\#33ff33" fill-opacity="0.2" />
          <!-- Label -->
          <text x="40" y="35" font-family="Arial" font-size="12" fill="\#e0e0e0" text-anchor="middle">MONO</text>
          <!-- Gradients -->
          <defs>
            <!-- Normal Button Gradient -->
            <linearGradient id="buttonGradient" x1="0%" y1="0%" x2="0%" y2="100%">
              <stop offset="0%" stop-color="\#4a4a4a" />
              <stop offset="100%" stop-color="\#3a3a3a" />
            </linearGradient>
          </defs>
        </svg>
    "#.as_bytes().to_vec())
}

fn load_stereo_svg_with_status(status: ButtonStatus) -> svg::Handle {
    let opacity = match status {
        ButtonStatus::Active => "0.8",
        ButtonStatus::Inactive => "0.2",
    };
    
    svg::Handle::from_memory(format!(r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 80 60">
          <!-- Button background -->
          <rect x="0" y="0" width="80" height="60" rx="5" fill="\#2a2a2a" />
          <!-- Button surface -->
          <rect x="5" y="5" width="70" height="50" rx="4" fill="url(#buttonGradient)" />
          <!-- LED Indicator -->
          <circle cx="15" cy="15" r="4" fill="\#33ff33" fill-opacity="{}" />
          <!-- Label -->
          <text x="40" y="35" font-family="Arial" font-size="12" fill="\#e0e0e0" text-anchor="middle">STEREO</text>
          <!-- Gradients -->
          <defs>
            <!-- Normal Button Gradient -->
            <linearGradient id="buttonGradient" x1="0%" y1="0%" x2="0%" y2="100%">
              <stop offset="0%" stop-color="\#4a4a4a" />
              <stop offset="100%" stop-color="\#3a3a3a" />
            </linearGradient>
          </defs>
        </svg>
    "#, opacity).as_bytes().to_vec())
}

fn load_stereo_svg() -> svg::Handle {
    svg::Handle::from_memory(r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 80 60">
          <!-- Button background -->
          <rect x="0" y="0" width="80" height="60" rx="5" fill="\#2a2a2a" />
          <!-- Button surface -->
          <rect x="5" y="5" width="70" height="50" rx="4" fill="url(\#buttonGradient)" />
          <!-- LED Indicator -->
          <circle cx="15" cy="15" r="4" fill="\#33ff33" fill-opacity="0.8" />
          <!-- Label -->
          <text x="40" y="35" font-family="Arial" font-size="12" fill="\#e0e0e0" text-anchor="middle">STEREO</text>
          <!-- Gradients -->
          <defs>
            <!-- Normal Button Gradient -->
            <linearGradient id="buttonGradient" x1="0%" y1="0%" x2="0%" y2="100%">
              <stop offset="0%" stop-color="l#4a4a4a" />
              <stop offset="100%" stop-color="l#3a3a3a" />
            </linearGradient>
          </defs>
        </svg>
    "#.as_bytes().to_vec())
}

fn load_mute_svg_with_status(status: ButtonStatus) -> svg::Handle {
    let opacity = match status {
        ButtonStatus::Active => "0.8",
        ButtonStatus::Inactive => "0.2",
    };
    
    svg::Handle::from_memory(format!(r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 80 60">
          <!-- Button background -->
          <rect x="0" y="0" width="80" height="60" rx="5" fill="\#2a2a2a" />
          <!-- Button surface -->
          <rect x="5" y="5" width="70" height="50" rx="4" fill="url(#buttonGradientRed)" />
          <!-- LED Indicator -->
          <circle cx="15" cy="15" r="4" fill="\#ff3333" fill-opacity="{}" />
          <!-- Label -->
          <text x="40" y="35" font-family="Arial" font-size="12" fill="\#e0e0e0" text-anchor="middle">MUTE</text>
          <!-- Gradients -->
          <defs>
            <!-- Red Button Gradient -->
            <linearGradient id="buttonGradientRed" x1="0%" y1="0%" x2="0%" y2="100%">
              <stop offset="0%" stop-color="\#4a3a3a" />
              <stop offset="100%" stop-color="\#3a2a2a" />
            </linearGradient>
          </defs>
        </svg>
    "#, opacity).as_bytes().to_vec())
}

fn load_mute_svg() -> svg::Handle {
    svg::Handle::from_memory(r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 80 60">
          <!-- Button background -->
          <rect x="0" y="0" width="80" height="60" rx="5" fill="\#2a2a2a" />
          <!-- Button surface -->
          <rect x="5" y="5" width="70" height="50" rx="4" fill="url(#buttonGradientRed)" />
          <!-- LED Indicator -->
          <circle cx="15" cy="15" r="4" fill="\#ff3333" fill-opacity="0.2" />
          <!-- Label -->
          <text x="40" y="35" font-family="Arial" font-size="12" fill="\#e0e0e0" text-anchor="middle">MUTE</text>
          <!-- Gradients -->
          <defs>
            <!-- Red Button Gradient -->
            <linearGradient id="buttonGradientRed" x1="0%" y1="0%" x2="0%" y2="100%">
              <stop offset="0%" stop-color="\#4a3a3a" />
              <stop offset="100%" stop-color="\#3a2a2a" />
            </linearGradient>
          </defs>
        </svg>
    "#.as_bytes().to_vec())
}

fn load_bypass_svg_with_status(status: ButtonStatus) -> svg::Handle {
    let opacity = match status {
        ButtonStatus::Active => "0.8",
        ButtonStatus::Inactive => "0.2",
    };
    
    svg::Handle::from_memory(format!(r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 80 60">
          <!-- Button background -->
          <rect x="0" y="0" width="80" height="60" rx="5" fill="\#2a2a2a" />
          <!-- Button surface -->
          <rect x="5" y="5" width="70" height="50" rx="4" fill="url(#buttonGradientYellow)" />
          <!-- LED Indicator -->
          <circle cx="15" cy="15" r="4" fill="\#ffaa33" fill-opacity="{}" />
          <!-- Label -->
          <text x="40" y="35" font-family="Arial" font-size="12" fill="\#e0e0e0" text-anchor="middle">BYPASS</text>
          <!-- Gradients -->
          <defs>
            <!-- Yellow Button Gradient -->
            <linearGradient id="buttonGradientYellow" x1="0%" y1="0%" x2="0%" y2="100%">
              <stop offset="0%" stop-color="\#4a4a3a" />
              <stop offset="100%" stop-color="\#3a3a2a" />
            </linearGradient>
          </defs>
        </svg>
    "#, opacity).as_bytes().to_vec())
}

fn load_bypass_svg() -> svg::Handle {
    svg::Handle::from_memory(r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 80 60">
          <!-- Button background -->
          <rect x="0" y="0" width="80" height="60" rx="5" fill="\#2a2a2a" />
          <!-- Button surface -->
          <rect x="5" y="5" width="70" height="50" rx="4" fill="url(#buttonGradientYellow)" />
          <!-- LED Indicator -->
          <circle cx="15" cy="15" r="4" fill="\#ffaa33" fill-opacity="0.2" />
          <!-- Label -->
          <text x="40" y="35" font-family="Arial" font-size="12" fill="\#e0e0e0" text-anchor="middle">BYPASS</text>
          <!-- Gradients -->
          <defs>
            <!-- Yellow Button Gradient -->
            <linearGradient id="buttonGradientYellow" x1="0%" y1="0%" x2="0%" y2="100%">
              <stop offset="0%" stop-color="\#4a4a3a" />
              <stop offset="100%" stop-color="\#3a3a2a" />
            </linearGradient>
          </defs>
        </svg>
    "#.as_bytes().to_vec())
}

fn load_knob_svg() -> svg::Handle {
    svg::Handle::from_memory(r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 80 60">
          <!-- Outer circle -->
          <circle cx="40" cy="35" r="30" fill="transparent" stroke="\#888888" stroke-width="2" />
          
          <!-- Inner circle -->
          <circle cx="40" cy="35" r="25" fill="url(\#gradient)" />
          
          <!-- Gradient -->
          <defs>
            <linearGradient id="gradient" x1="50%" y1="0%" x2="50%" y2="100%">
              <stop offset="0%" stop-color="\#ffcc00" />
              <stop offset="100%" stop-color="\#ff6600" />
            </linearGradient>
          </defs>
          
          <!-- Center dot -->
          <circle cx="40" cy="35" r="10" fill="\#ffffff" />
        </svg>
    "#.as_bytes().to_vec())
}

fn load_placeholder_svg() -> svg::Handle {
    svg::Handle::from_memory(r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 80 60">
          <rect x="0" y="0" width="80" height="60" rx="5" fill="\#444444" />
          <text x="40" y="35" font-family="Arial" font-size="12" fill="\#e0e0e0" text-anchor="middle">BUTTON</text>
        </svg>
    "#.as_bytes().to_vec())
}

