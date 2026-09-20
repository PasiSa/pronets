use macroquad::prelude::*;

const ACCELERATION: f32 = 200.0; // Pixels per second squared.
const TURN_SPEED: f32 = 2.0; // Radians per second.

struct Car {
    // Screen position in pixels of the texture's top-left corner before rotation.
    pos: Vec2,
    // Rotation in radians; 0 points right, and positive angles turn clockwise.
    angle: f32,
    // Movement speed in pixels per second; negative values move the car backward.
    speed: f32,
    // Image used to draw the car on screen.
    texture: Texture2D,
}

#[macroquad::main("Racing")]
async fn main() {

     let mut car = Car {
        pos: vec2(400.0, 300.0),
        angle: 0.0,
        speed: 0.0,
        texture: load_texture("car.png").await.unwrap(),
    };

    loop {
        // Delta time: seconds since the last frame. Multiply rates by dt to keep
        // movement and rotation consistent across different frame rates.
        let dt = get_frame_time();

        // Acceleration / braking
        if is_key_down(KeyCode::Up) {
            car.speed += ACCELERATION * dt;
        }

        if is_key_down(KeyCode::Down) {
            car.speed -= ACCELERATION * dt;
        }

        if is_key_down(KeyCode::Left) {
            car.angle -= TURN_SPEED * dt;
        }
        if is_key_down(KeyCode::Right) {
            car.angle += TURN_SPEED * dt;
        }

        // Friction
        car.speed *= 0.99;

        // Convert the angle (in radians) into a unit vector: cos gives the x
        // component and sin gives the y component, indicating where the car moves.
        let direction = vec2(car.angle.cos(), car.angle.sin());

        // Move the car in the direction it's facing, scaled by its speed and the time
        // since the last frame.
        car.pos += direction * car.speed * dt;
        clear_background(DARKGREEN);

        // Draw the car texture at its screen position. WHITE preserves its original
        // colors, and DrawTextureParams rotates it by car.angle (in radians)
        // around its center, using defaults for the remaining drawing options.
        draw_texture_ex(
            &car.texture,
            car.pos.x,
            car.pos.y,
            WHITE,
            DrawTextureParams {
                rotation: car.angle,
                ..Default::default()
            },
        );

        // Yield control to Macroquad so it can display this frame and process
        // input, then resume the loop when the next frame is ready.
        next_frame().await;
    }
}
