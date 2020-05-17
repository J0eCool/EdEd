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

    func keyWentDown(s8) -> u1;
}

/**IT_END**/

bool isDown = false;
bool wasDown = false;
int xPos = 0;
int yPos = 0;

void update() {
    wasDown = isDown;
}

// ----------------
// Mouse input
// TODO: enums for events, maybe structure?
void onMouseEvent(int eventId, int x, int y) {
    xPos = x;
    yPos = y;
    switch (eventId) {
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

// -------------
// Keyboard input
bool keyWentDown(char key) {
    // TODO: logic
    return false;
}
