// Reads event-based mouse input and provides a polling-based API

/**IT_START**/

export {
    func update();
    func onMouseEvent(s32, s32, s32);

    func mouseIsDown() -> u1;
    func mouseWentDown() -> u1;
    func mouseWentUp() -> u1;
    func mouseX() -> s32;
    func mouseY() -> s32;
}

/**IT_END**/

bool isDown = false;
bool wasDown = false;
int xPos = 0;
int yPos = 0;

void update() {
    wasDown = isDown;
}

// TODO: enums for events, maybe structure?
void onMouseEvent(int event, int x, int y) {
    xPos = x;
    yPos = y;
    switch (event) {
        case 0: { // move event
            break;
        }
        case 1: { // down event
            isDown = true;
            break;
        }
        case 2: { // up event
            isDown = false;
            break;
        }
    }
}

bool mouseIsDown() {
    return isDown;
}
bool mouseWentDown() {
    return isDown && !wasDown;
}
bool mouseWentUp() {
    return !isDown && wasDown;
}
int mouseX() {
    return xPos;
}
int mouseY() {
    return yPos;
}
