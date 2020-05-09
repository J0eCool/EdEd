/**IT_START**/

import "env" {
    func draw(s32);
    func sin(f32) -> f32;
}
export {
    func frame();
}

/**IT_END**/

// #include <math.h>

int t = 0;
float PI = 3.14159;
void frame() {
    t++;
    draw(100 * sin(t * PI / 60) + 200);

    // for (int i = 1; i <= n; ++i) {
    //     auto fizz = i % 3 == 0;
    //     auto buzz = i % 5 == 0;
    //     if (fizz && buzz) {
    //         log("FizzBuzz");
    //     } else if (fizz) {
    //         log("fizz");
    //     } else if (buzz) {
    //         log("buzz");
    //     } else {
    //         logInt(i);
    //     }
    // }
}
