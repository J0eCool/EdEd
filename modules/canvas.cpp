// A canvas is a window in to a texture-editing context

/**IT_START**/

import "render" {
    func allocImage() -> s32;
    func updateImage(s32, s32, s32);
}
import "input" {
    func mouseIsDown() -> u1;
    func mouseWentDown() -> u1;
    func mouseX() -> s32;
    func mouseY() -> s32;
}
// TO DO: rewrite canvas.cpp to USE Texture modules
// TO DO: fix C++ parser/generator as well
type Texture = import "texture" {
// import "texture" {
    func init(s32, s32);

    // type Color = struct { r: u8, g: u8, b: u8, a: u8 };
    // type Color = s32;
    // func getPixel(s32, s32) -> Color;
    // func setPixel(s32, s32, Color);
    func getPixel(s32, s32) -> s32;
    func setPixel(s32, s32, s32);
    func draw();
}
export {
    func init();
    func update();
}

/**IT_END**/

// TODO: autogenerate this
#define IMPORT(ns, n) __attribute__((import_module(ns), import_name(n)))
using _Texture = void*;
IMPORT("texture", "_construct") _Texture Texture_construct();
IMPORT("texture", "init") void init(_Texture, int, int);
IMPORT("texture", "getPixel") int getPixel(_Texture, int, int);
IMPORT("texture", "setPixel") void setPixel(_Texture, int, int, int);
IMPORT("texture", "draw") void draw(_Texture);
class Texture {
    _Texture data;
public:
    // Texture() : data(nullptr) {}
    Texture() : data(Texture_construct()) {}
    void init(int _1, int _2) {
        return ::init(data, _1, _2);
    }
    int getPixel(int _1, int _2) {
        return ::getPixel(data, _1, _2);
    }
    void setPixel(int _1, int _2, int _3) {
        return ::setPixel(data, _1, _2, _3);
    }
    void draw() {
        return ::draw(data);
    }
};

typedef unsigned char u8;

// TODO: programmatically
int screenWidth = 400;
int screenHeight = 300;

Texture tex;
const int width = 16;
const int height = 16;

void init() {
    tex = Texture();
    tex.init(width, height);
    for (int x = 0; x < width; ++x) {
        for (int y = 0; y < height; ++y) {
            tex.setPixel(x, y, 0);
        }
    }
}

void paint(int x, int y) {
    int i = x * width / screenWidth;
    int j = y * height / screenHeight;
    int color = 0xfff00fff; // TODO: see why this shows as white
    tex.setPixel(i, j, color);
}

void update() {
    if (mouseIsDown()) {
        paint(mouseX(), mouseY());
    }
    tex.draw();
}
