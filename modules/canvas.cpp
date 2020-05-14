/**IT_START**/

import "render" {
    func allocImage() -> s32;
    func drawImage(s32);
    func updateImage(s32, s32, s32);
}
import "input" {
    func mouseIsDown() -> u1;
    func mouseWentDown() -> u1;
    func mouseX() -> s32;
    func mouseY() -> s32;
}
export {
    func init();
    func update();
}

/**IT_END**/

typedef unsigned char u8;

int t = 0;
float PI = 3.14159;
const int width = 16;
const int height = 16;
const int imageSize = width * height;
u8 imageData[imageSize];

// TODO: programmatically
int screenWidth = 400;
int screenHeight = 300;

int imageId = 0;

void init() {
    imageId = allocImage();
    for (int x = 0; x < width; ++x) {
        for (int y = 0; y < height; ++y) {
            imageData[x + y * width] = (x * x + y * y) / (width + height);
        }
    }
    updateImage(imageId, (int)imageData, imageSize);
}

u8 toPaint = 0xff;
void paint(int x, int y) {
    int i = x * width / screenWidth;
    int j = y * height / screenHeight;
    int idx = i + j * width;
    if (idx >= 0 && idx < width * height) {
        imageData[idx] = toPaint;
        updateImage(imageId, (int)imageData, imageSize);
    }
}

void update() {
    if (toPaint < 5) {
        toPaint = 0;
    } else {
        toPaint -= 5;
    }
    if (mouseWentDown()) {
        toPaint = 0xff;
    }
    if (mouseIsDown()) {
        paint(mouseX(), mouseY());
    }
    drawImage(imageId);
}
