use crate::mem;

// Enable graphics mode.
fn gr(enable: bool) {
    if (enable) {
        mem::read(0xC050); // text mode off
    } else {
        mem::read(0xC051); // text mode on
    }
}

// Toggle mixed graphics and text.
fn mixed(enable: bool) {
    if (enable) {
        mem::read(0xC053);
    } else {
        mem::read(0xC052);
    }
}

fn draw_pixel(p: Point, color: u8) {
    mem::write(coord_to_addr(p), color << 4 | color);
}

// Note that `rect` indices exlude the bottom-right point.
//
// A naive for-loop was too slow (maybe we're supposed to turn on optimizations
// in the compiler?) so we're using library routines instead.
//
// It's plenty fast, but it's using text-mode glyphs to draw the lines,
// e.g. "-------------" is a horizontal line, so it looks kinda glitchy in
// gr mode. But honestly, it's growing on me, so we'll keep it for now.
void draw_box(rect r) {
    i8 dx = r.bot_right.x - r.top_left.x;
    i8 dy = r.bot_right.y - r.top_left.y;

    // Make indices inclusive.
    r.bot_right = minus(r.bot_right, one_one);

    chlinexy(r.top_left.x, r.top_left.y, dx); // top
    chlinexy(r.top_left.x, r.bot_right.y, dx); // bottom
    cvlinexy(r.top_left.x, r.top_left.y, dy); // left
    cvlinexy(r.bot_right.x, r.top_left.y, dy); // right
}

// What is the memory address for this low-res pixel?
//
// Note that there's technically a "top-half" and a "bottom-half"
// to each of these "pixels". Each can hold a 4-bit color.
u16 coord_to_addr(point p) {
    u8 group;
    u16 base, offset;
    assert(p.x < 40);
    assert(p.y < 24);

    group = p.y / 8;
    switch (group) {
    case 0:
        base = 0x400;
        break;
    case 1:
        base = 0x428;
        break;
    case 2:
        base = 0x450;
        break;
    }

    offset = p.y % 8 * 0x80;

    return base + offset + p.x;
}

