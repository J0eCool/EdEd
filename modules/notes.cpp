/**IT_START**/

import "render" {
    func drawText(string);
}
import "input" {
    func keyWentDown(s8) -> u1;
}
export {
    func update();
}

/**IT_END**/

#include <string>

std::string text = "";

void update() {
    for (char c = 'a'; c < 'z'; ++c) {
        if (keyWentDown(c)) {
            text += c;
        }
    }

    drawText(text.c_str());
}
