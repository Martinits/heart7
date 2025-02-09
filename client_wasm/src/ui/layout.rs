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
pub const PROMPT_MSG:      Rect = Rect { x: 72.0,  y: 88.2,  w: 576.0, h: 52.0  };
pub const PROMPT_INPUT:    Rect = Rect { x: 187.2, y: 164.3, w: 345.6, h: 44.8  };
pub const PROMPT_BUTTON_1: Rect = Rect { x: 293.8, y: 229.9, w: 132.5, h: 26.6  };
pub const PROMPT_BUTTON_2: [Rect; 2] = [
    Rect { x: 216.0, y: 229.9, w: 115.2, h: 26.6 },
    Rect { x: 388.8, y: 229.9, w: 115.2, h: 26.6 },
];

// room id
pub const ROOM_ID:    Rect = Rect { x: 10.0,  y: 40.0,  w: 125.0,  h: 20.0 };

// wait
pub const WAIT_CENTER_MSG:   Rect = Rect { x: 180.0, y: 161.5, w: 360.0, h: 57.0 };
pub const WAIT_READY_BUTTON: Rect = Rect { x: 295.2, y: 304.0, w: 129.6, h: 30.4 };
pub const WAIT_PLAYER_READY: [Rect; 4] = [
    Rect { x: 321.1, y: 304.0, w: 77.8, h: 30.4 },
    Rect { x: 612.0, y: 180.5, w: 72.0, h: 19.0 },
    Rect { x: 324.0, y: 38.0,  w: 72.0, h: 19.0 },
    Rect { x: 14.4,  y: 197.6, w: 72.0, h: 19.0 }
];

// player
pub const PLAYER_MYSELF: Rect = Rect { x: 36.0,  y: 285.0, w: 72.0, h: 57.0 };
pub const PLAYER_RIGHT:  Rect = Rect { x: 626.4, y: 95.0,  w: 72.0, h: 45.6 };
pub const PLAYER_TOP:    Rect = Rect { x: 216.0, y: 11.4,  w: 72.0, h: 45.6 };
pub const PLAYER_LEFT:   Rect = Rect { x: 21.6,  y: 114.0, w: 72.0, h: 45.6 };
