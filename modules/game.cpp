/**IT_START**/

import "render" {
    func draw(s32);
    func sin(f32) -> f32;
}
export {
    func frame();
}

/**IT_END**/

int t = 0;
float PI = 3.14159;
void frame() {
    t++;
    draw(100 * sin(t * PI / 60) + 200);
}
