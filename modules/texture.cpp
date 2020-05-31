/**IT_START**/

import "render" {
    func allocImage() -> s32;
    func updateImage(s32, s32, s32);
}
export {
    func init(s32, s32);

    // type Color = struct { r: u8, g: u8, b: u8, a: u8 };
    // type Color = s32;
    // func getPixel(s32, s32) -> Color;
    // func setPixel(s32, s32, Color);
    func getPixel(s32, s32) -> s32;
    func setPixel(s32, s32, s32);
}

/**IT_END**/

typedef unsigned char u8;

struct Color {
    u8 r, g, b, a;

    Color() : r(), g(), b(), a() { }
    Color(int c) : r(c), g(c), b(c), a(0xff) { }
    Color(int _r, int _g, int _b) : r(_r), g(_g), b(_b), a(0xff) { }
    Color(int _r, int _g, int _b, int _a) : r(_r), g(_g), b(_b), a(_a) { }

    // Not needed with IT support
    operator int() const {
        return r
            | (g << 8)
            | (b << 16)
            | (a << 24);
    }
    static Color fromInt(int v) {
        return Color(
            v & 0xff,
            (v & 0xff00) >> 8,
            (v & 0xff0000) >> 16,
            (v & 0xff000000) >> 24);
    }
};

int imageId = 0;
int w = 0;
int h = 0;
Color* texture = nullptr;

void init(int _w, int _h) {
    w = _w; h = _h;
    auto old = texture;
    texture = new Color[w * h];
    imageId = allocImage();
}

int getPixel(int x, int y) {
    return texture[x + w * y];
}
void setPixel(int x, int y, int color_) {
    Color color = Color::fromInt(color_);
    texture[x + w * y] = color;
}
