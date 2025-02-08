use super::*;

pub fn ui_exit_menu(button_num: u32) {
    let menu_rect = get_canvas_rect().center_cut(Percent(40), Percent(80));
    draw_rounded_rect(&menu_rect, BORDER_LIGHT);

    match button_num {
        2 => {
            // let slices = menu_rect.center_cut_width(Percent(50)).cut_height([
            //     Percent(30),
            //     Percent(10),
            //     Percent(20),
            //     Percent(10),
            //     Percent(30),
            // ]);
            // warn!("Button rect {:?}", slices[1]);
            // warn!("Button rect {:?}", slices[3]);

            draw_button(&EM_BUTTON_2[0], "Back", true);
            draw_button(&EM_BUTTON_2[1], "Exit Program", true);
        }
        3 => {
            // let slices = menu_rect.center_cut_width(Percent(50)).cut_height([
            //     Percent(20),
            //     Percent(10),
            //     Percent(15),
            //     Percent(10),
            //     Percent(15),
            //     Percent(10),
            //     Percent(20),
            // ]);
            // warn!("Button rect {:?}", slices[1]);
            // warn!("Button rect {:?}", slices[3]);
            // warn!("Button rect {:?}", slices[5]);

            draw_button(&EM_BUTTON_3[0], "Back", true);
            draw_button(&EM_BUTTON_3[1], "Exit Room", true);
            draw_button(&EM_BUTTON_3[2], "Exit Program", true);
        }
        4 => {
            // let slices = menu_rect.center_cut_width(Percent(50)).cut_height([
            //     Percent(15),
            //     Percent(10),
            //     Percent(10),
            //     Percent(10),
            //     Percent(10),
            //     Percent(10),
            //     Percent(10),
            //     Percent(10),
            //     Percent(15),
            // ]);
            // warn!("Button rect {:?}", slices[1]);
            // warn!("Button rect {:?}", slices[3]);
            // warn!("Button rect {:?}", slices[5]);
            // warn!("Button rect {:?}", slices[7]);

            draw_button(&EM_BUTTON_4[0], "Back", true);
            draw_button(&EM_BUTTON_4[1], "Exit Game", true);
            draw_button(&EM_BUTTON_4[2], "Exit Room", true);
            draw_button(&EM_BUTTON_4[3], "Exit Program", true);
        }
        _ => panic!("Invalid buttom nums!"),
    }
}
