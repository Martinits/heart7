use crate::*;

// exit menu
pub const ESC_BUTTON:    Rect = Rect { x: 10.0,  y: 10.0,  w: 60.0,  h: 25.0 };
pub const EM_BUTTON_2:   [Rect; 2] = [
    Rect { x: 288.0, y: 129.2, w: 144.0, h: 30.4 },
    Rect { x: 288.0, y: 220.4, w: 144.0, h: 30.4 },
];
pub const EM_BUTTON_3:   [Rect; 3] = [
    Rect { x: 288.0, y: 98.8,  w: 144.0, h: 30.4 },
    Rect { x: 288.0, y: 174.8, w: 144.0, h: 30.4 },
    Rect { x: 288.0, y: 250.8, w: 144.0, h: 30.4 },
];
pub const EM_BUTTON_4:   [Rect; 4] = [
    Rect { x: 288.0, y: 83.6,  w: 144.0, h: 30.4 },
    Rect { x: 288.0, y: 144.4, w: 144.0, h: 30.4 },
    Rect { x: 288.0, y: 205.2, w: 144.0, h: 30.4 },
    Rect { x: 288.0, y: 266.0, w: 144.0, h: 30.4 },
];

// prompt window
pub const PROMPT_WINDOW:   Rect = Rect { x: 72.0,  y: 57.0,  w: 576.0, h: 266.0 };
pub const PROMPT_MSG:      Rect = Rect { x: 72.0,  y: 100.2, w: 576.0, h: 39.9  };
pub const PROMPT_INPUT:    Rect = Rect { x: 187.2, y: 164.3, w: 345.6, h: 44.8  };
pub const PROMPT_BUTTON_1: Rect = Rect { x: 293.8, y: 229.9, w: 132.5, h: 26.6  };
pub const PROMPT_BUTTON_2: [Rect; 2] = [
    Rect { x: 216.0, y: 229.9, w: 115.2, h: 26.6 },
    Rect { x: 388.8, y: 229.9, w: 115.2, h: 26.6 },
];
