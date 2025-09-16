// library bach khdmna canva https://macroquad.rs/docs/
//You are free to use any library to create and handle windows, rendering, keyboard and mouse events
// swlo bnadm la makanch possible nkhdmo b macroquad draw ndiro li khdmnaha f raid Drawing
use macroquad::prelude::*;

// radius dyal point

// smiya dyal window
#[macroquad::main("Chaikin")]

// main ra async kan htajoh 3la wed macroquad, next_frame, ... (bach program ib9a khdam smooth)
// 9raw 3la async athtajoha (wla ghir chi l3ibat khfaf 3la wed audit)
async fn main() {
    let radius: f32 = 5.0;
    let steps: usize = 7;
    let mut saved_points: Vec<Vec2> = Vec::new();
    let mut animate = false;
    let mut step = 0;
    let mut last_time = 0.0;
    loop {
        // kola frame kan ms7o background bach it3awdo itrsmo les line
        clear_background(BLACK);
        // kan hto saved points on rsmo circles
        for p in &saved_points {
            draw_circle(p.x, p.y, radius, GRAY);
        }
        // ghir key listeners ra baynin acg kidiro (check subject)
        if is_mouse_button_pressed(MouseButton::Left) {
            saved_points.push(mouse_position().into());
        }
        if is_key_pressed(KeyCode::Enter) && !saved_points.is_empty() && animate == false {
            animate = true;
            step = 0;
        }
        if is_key_pressed(KeyCode::Escape){
            break;
        }
        if is_key_pressed(KeyCode::R) {
            saved_points.clear();
            animate = false;
        }
        // hna kan bdaw nrsmo
        if animate && saved_points.len() > 1 {
            let mut points = saved_points.clone();
            // kn dwzto Chaikin algorithm algo kola mra 3la hsab step li waslin lihom
            for _ in 0..step {
                points = chaikin(&points);
            }
            // kan rsmo lines bin kola 2 points (chack algo ra ki kono bzaf)
            for i in 0..points.len() - 1 {
                draw_line(
                    points[i].x,
                    points[i].y,
                    points[i + 1].x,
                    points[i + 1].y,
                    2.0,
                    GOLD,
                );
            }
            // wra kola rasma kan checkiw saved time bach dakchi i ban mzyan
            let current_time = get_time();
            if current_time - last_time > 0.2 {
                step += 1;
                last_time = current_time;
                if step > steps {
                    step = 0;
                }
            }
        }
        // zid l frame jaya wra ma kilchi i trendra
        next_frame().await;
    }
}

// Chaikin algorithm
fn chaikin(points: &Vec<Vec2>) -> Vec<Vec2> {
    let mut new_points = Vec::new();
    // kan saviw start
    new_points.push(points[0]);
    // hna kan zido 2 points blast kola point 1 bach iwli curve smooth blast sharpe (zawiya m9ayma)
    for i in 0..points.len() - 1 {
        let p0 = points[i];
        let p1 = points[i + 1];
        // hna kan khdmo b 75% + 25% li hiya 100% bach les 2 point ijiw curved o 9rab (curve ma ki konch f straight line tester l3ibat tfhm ktr)
        let q = p0 * 0.75 + p1 * 0.25;
        let r = p0 * 0.25 + p1 * 0.75;
        // save 2 points jdad blast point 9dima
        new_points.push(q);
        new_points.push(r);
    }
    // kan zido end
    new_points.push(points[points.len() - 1]);
    // 3la slamtk nhar audit n failiw
    return new_points;
}
