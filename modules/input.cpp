// Reads event-based mouse input and provides a polling-based API

/**IT_START**/

export {
    func update();
    func onMouseEvent(s32, s32, s32);
    func onKeyEvent(s32, s32);

    func mouseIsDown() -> u1;
    func mouseWentDown() -> u1;
    func mouseWentUp() -> u1;
    func mouseX() -> s32;
    func mouseY() -> s32;

    func keyWentDown(s8) -> u1;
}

/**IT_END**/

// Mouse data
bool isMouseDown = false;
bool wasMouseDown = false;
int xPos = 0;
int yPos = 0;

// Keyboard data
const int numKeys = 256;
bool isKeyDown[numKeys];
bool wasKeyDown[numKeys];

void update() {
    wasMouseDown = isMouseDown;

    for (int i = 0; i < numKeys; ++i) {
        wasKeyDown[i] = isKeyDown[i];
    }
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
            isMouseDown = true;
            break;
        }
        case 2: { // up event
            isMouseDown = false;
            break;
        }
    }
}

void onKeyEvent(int eventId, int key) {
    if (key >= numKeys) return;
    switch (eventId) {
        case 0: // key down
            isKeyDown[key] = true;
            break;
        case 1:
            isKeyDown[key] = false;
            break;
    }
}

bool mouseIsDown() {
    return isMouseDown;
}
bool mouseWentDown() {
    return isMouseDown && !wasMouseDown;
}
bool mouseWentUp() {
    return !isMouseDown && wasMouseDown;
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
    return isKeyDown[key] && !wasKeyDown[key];
}
