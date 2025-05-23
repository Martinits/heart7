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
pub const PROMPT_WINDOW:        Rect = Rect { x: 72.0,  y: 57.0,  w: 576.0, h: 266.0 };
pub const PROMPT_MSG:           Rect = Rect { x: 72.0,  y: 88.2,  w: 576.0, h: 52.0  };
pub const PROMPT_INPUT:         Rect = Rect { x: 187.2, y: 164.3, w: 345.6, h: 44.8  };
pub const PROMPT_INPUT_TEXT:    Rect = Rect { x: 194.1, y: 178.7, w: 331.8, h: 16.0  };
pub const PROMPT_BUTTON_1:      Rect = Rect { x: 293.8, y: 229.9, w: 132.5, h: 26.6  };
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
pub const PLAYER_TOP:    Rect = Rect { x: 176.0, y: 11.4,  w: 72.0, h: 45.6 };
pub const PLAYER_LEFT:   Rect = Rect { x: 21.6,  y: 114.0, w: 72.0, h: 45.6 };

// gaming
pub const MY_CARD_WIDTH: f64 = 60.0;
pub const MY_CARD_HEIGHT: f64 = 80.0;
pub const MY_CARD_OVERLAP_WIDTH: f64 = 20.0;
pub const MY_CARD_UP_HEIGHT: f64 = 15.0;
pub const MY_CARD_LEFT_START: Rect = Rect { x: 217.6, y: 285.0, w: MY_CARD_WIDTH, h: MY_CARD_HEIGHT };
pub const GAMING_BUTTON_PLAY: Rect = Rect { x: 126.8, y: 282.0, w: 72.0,  h: 34.2 };
pub const GAMING_BUTTON_HOLD: Rect = Rect { x: 126.8, y: 327.6, w: 72.0,  h: 34.2 };
pub const GAMING_MSG:         Rect = Rect { x: 210.4, y: 247.0, w: 309.6, h: 19.0 };
pub const GAMING_NEXT: [Rect; 4] = [
    Rect { x: 116.6, y: 247.0, w: 93.6, h: 19.0 },
    Rect { x: 612.0, y: 190.0, w: 93.6, h: 19.0 },
    Rect { x: 262.4, y: 45.4,  w: 93.6, h: 19.0 },
    Rect { x: 7.2,   y: 209.0, w: 93.6, h: 19.0 },
];
pub const GAMING_LAST: [Rect; 4] = [
    Rect { x:   0.0, y:   0.0, w:  0.0, h:  0.0 },
    Rect { x: 576.0, y: 133.0, w: CARD_V_WIDTH, h: CARD_V_HEIGHT },
    Rect { x: 432.0, y: 19.0,  w: CARD_V_WIDTH, h: CARD_V_HEIGHT },
    Rect { x: 107.4, y: 140.6, w: CARD_V_WIDTH, h: CARD_V_HEIGHT },
];

// card
pub const CARD_V_WIDTH: f64 = 22.0;
pub const CARD_V_HEIGHT: f64 = 40.0;
pub const CARD_H_WIDTH: f64 = 40.0;
pub const CARD_H_HEIGHT: f64 = 22.0;
pub const CARD_ICON_WIDTH:  f64 = 15.0;
pub const CARD_ICON_HEIGHT: f64 = 15.0;

// desk
pub const DESK_HOLD_NUM: [Rect; 4] = [
    Rect { x:   0.0, y:   0.0, w:  0.0, h:  0.0 },
    Rect { x: 612.0, y: 163.4, w: 93.6, h: 19.0 },
    Rect { x: 262.4, y: 18.4,  w: 93.6, h: 19.0 },
    Rect { x: 7.2,   y: 182.4, w: 93.6, h: 19.0 },
];
pub const DESK_MY_HOLD_BORDER: Rect = Rect { x: 532.8, y: 247.0, w: 180.0, h: 125.4 };
pub const DESK_MY_HOLD_TITLE:  Rect = Rect { x: 532.8, y: 249.5, w: 180.0, h: 23.8 };
pub const DESK_MY_HOLD_BOTTOM: Rect = Rect { x: 532.8, y: 273.3, w: 180.0, h: 99.1 };
pub const DESK_MY_HOLD_LINE1_START: Rect = Rect { x: 539.8, y: 277.0, w: 22.0, h: 40.0 };
pub const DESK_MY_HOLD_GAP_WIDTH: f64 = 2.0;
pub const DESK_MY_HOLD_GAP_HEIGHT: f64 = 8.0;
#[allow(unused)]
pub const DESK_CHAIN_GAP: f64 = 15.0;
pub const DESK_CHAIN_CARD_GAP: f64 = 10.0;
pub const DESK: Rect = Rect { x: 237.6, y: 83.6, w: 230.4, h: 154.0 };
pub const DESK_CHAIN: [Rect; 4] = [
    Rect { x: 250.3, y: 83.6, w: 40.0, h: 154.0 },
    Rect { x: 305.3, y: 83.6, w: 40.0, h: 154.0 },
    Rect { x: 360.3, y: 83.6, w: 40.0, h: 154.0 },
    Rect { x: 415.3, y: 83.6, w: 40.0, h: 154.0 },
];

// game result
pub const RESULT_HOLD_POINTS: [Rect; 4] = [
    Rect { x:   0.0, y:   0.0, w:  0.0, h:  0.0 },
    Rect { x: 610.8, y: 190.0, w: 96.0, h: 19.0 },
    Rect { x: 261.2, y: 45.4,  w: 96.0, h: 19.0 },
    Rect { x: 6.0,   y: 209.0, w: 96.0, h: 19.0 },
];
pub const RESULT_CONTINUE_BUTTON: Rect = Rect { x: 140.0, y: 301.6, w: 115.2, h: 38.0 };
pub const RESULT_MSG: Rect = Rect { x: 268.0, y: 311.6, w: 250.0, h: 20.0 };
pub const RESULT_HOLD_GAP_WIDTH: f64 = 2.0;
pub const RESULT_HOLD_GAP_HEIGHT: f64 = 8.0;
pub const RESULT_HOLD_LEFT_START:  Rect = Rect { x: 108.0, y: 114.0, w: 22.0, h: 40.0 };
pub const RESULT_HOLD_TOP_START:   Rect = Rect { x: 367.2, y: 19.0,  w: 22.0, h: 40.0 };
pub const RESULT_HOLD_RIGHT_START: Rect = Rect { x: 576.0, y: 95.0,  w: 22.0, h: 40.0 };
pub const RESULT_HOLD_LEFT_EMPTY:  Rect = Rect { x: 118.0, y: 154.0, w: 85.0, h: 20.0 };
pub const RESULT_HOLD_TOP_EMPTY:   Rect = Rect { x: 370.2, y: 30.0,  w: 85.0, h: 20.0 };
pub const RESULT_HOLD_RIGHT_EMPTY: Rect = Rect { x: 496.0, y: 145.0,  w: 85.0, h: 20.0 };
pub const RESULT_MSG_FMT_STRLEN: [f64; 3] = [90.7, 101.4, 193.0];
