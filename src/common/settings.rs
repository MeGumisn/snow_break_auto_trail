use once_cell::sync::Lazy;
use opencv::core::Mat;
use opencv::imgcodecs::{IMREAD_GRAYSCALE, imread};
struct ButtonRect {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

struct Template {
    rect: ButtonRect,
    img: Mat,
}

fn read_img_from_file(path: &str) -> Mat {
    let mat = imread(path, IMREAD_GRAYSCALE).unwrap();
    mat
}
// 主页面开始战斗按钮
const HOME_PAGE: ButtonRect = ButtonRect {
    left: 1500,
    top: 400,
    width: 420,
    height: 210,
};

// 悖论迷宫按钮
const MAZE_BUTTON: ButtonRect = ButtonRect {
    left: 1450,
    top: 230,
    width: 415,
    height: 150,
};
// 验证战场按钮
const BATTLE_GROUND: ButtonRect = ButtonRect {
    left: 10,
    top: 140,
    width: 300,
    height: 120,
};

// 增益试炼按钮
const TRAIL_BUTTON: ButtonRect = ButtonRect {
    left: 735,
    top: 480,
    width: 430,
    height: 240,
};
/* 试炼循环 */
// 厄险关卡按钮
const EXTREME_TRAIL: ButtonRect = ButtonRect {
    left: 1380,
    top: 275,
    width: 300,
    height: 130,
};
// 开始作战按钮
const START_BUTTON: ButtonRect = ButtonRect {
    left: 1600,
    top: 975,
    width: 325,
    height: 135,
};
// 确认 buff 按钮
const CONFIRM_BUFF_BUTTON: ButtonRect = ButtonRect {
    left: 750,
    top: 980,
    width: 450,
    height: 120,
};
// 单人buff 栏位
const BUFF_SLOT: ButtonRect = ButtonRect {
    left: 1170,
    top: 275,
    width: 570,
    height: 555,
};
// 确认单人buff 按钮
const BUFF_SLOT_CONFIRM: ButtonRect = ButtonRect {
    left: 1400,
    top: 985,
    width: 355,
    height: 100,
};
// 丢弃单人buff 按钮
const BUFF_SLOT_CANCEL: ButtonRect = ButtonRect {
    left: 100,
    top: 985,
    width: 355,
    height: 100,
};
// 确认丢弃BUFF按钮
const CONFIRM_CANCEL_BUTTON: ButtonRect = ButtonRect {
    left: 1275,
    top: 750,
    width: 415,
    height: 130,
};
// 退出按钮
const QUIT_BUTTON: ButtonRect = ButtonRect {
    left: 790,
    top: 970,
    width: 430,
    height: 120,
};

static TEMPLATE_HOME_PAGE: Lazy<Template> = Lazy::new(|| Template {
    rect: HOME_PAGE,
    img: read_img_from_file("assets/HomePage.png"),
});

static TEMPLATE_MAZE_BUTTON: Lazy<Template> = Lazy::new(|| Template {
    rect: MAZE_BUTTON,
    img: read_img_from_file("assets/MazeButton.png"),
});

static TEMPLATE_BATTLE_GROUND: Lazy<Template> = Lazy::new(|| Template {
    rect: BATTLE_GROUND,
    img: read_img_from_file("assets/BattleGround.png"),
});

static TEMPLATE_TRAIL_BUTTON: Lazy<Template> = Lazy::new(|| Template {
    rect: TRAIL_BUTTON,
    img: read_img_from_file("assets/TrailButton.png"),
});

static TEMPLATE_EXTREME_TRAIL: Lazy<Template> = Lazy::new(|| Template {
    rect: EXTREME_TRAIL,
    img: read_img_from_file("assets/ExtremeTrail.png"),
});

static TEMPLATE_START_BUTTON: Lazy<Template> = Lazy::new(|| Template {
    rect: START_BUTTON,
    img: read_img_from_file("assets/StartButton.png"),
});

static TEMPLATE_CONFIRM_BUFF_BUTTON: Lazy<Template> = Lazy::new(|| Template {
    rect: CONFIRM_BUFF_BUTTON,
    img: read_img_from_file("assets/ConfirmBuffButton.png"),
});

static TEMPLATE_BUFF_SLOT: Lazy<Template> = Lazy::new(|| Template {
    rect: BUFF_SLOT,
    img: read_img_from_file("assets/BuffSlot.png"),
});

static TEMPLATE_BUFF_SLOT_CONFIRM: Lazy<Template> = Lazy::new(|| Template {
    rect: BUFF_SLOT_CONFIRM,
    img: read_img_from_file("assets/BuffSlotConfirm.png"),
});

static TEMPLATE_BUFF_SLOT_CANCEL: Lazy<Template> = Lazy::new(|| Template {
    rect: BUFF_SLOT_CANCEL,
    img: read_img_from_file("assets/BuffSlotCancel.png"),
});

static TEMPLATE_CONFIRM_CANCEL_BUTTON: Lazy<Template> = Lazy::new(|| Template {
    rect: CONFIRM_CANCEL_BUTTON,
    img: read_img_from_file("assets/ConfirmCancelButton.png"),
});

static TEMPLATE_QUIT_BUTTON: Lazy<Template> = Lazy::new(|| Template {
    rect: QUIT_BUTTON,
    img: read_img_from_file("assets/QuitButton.png"),
});

#[cfg(test)]
mod tests {
    use super::*;
    use opencv::highgui::{imshow, wait_key};
    #[test]
    fn test_templates() {
        imshow("template_home_page", &TEMPLATE_HOME_PAGE.img)
            .expect("Failed to show home page template");
        wait_key(0).expect("Failed to wait key because of errors");
    }
}
