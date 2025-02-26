use nih_plug_iced::widget::svg;

pub fn load_knob_svg() -> svg::Handle {
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