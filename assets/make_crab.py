"""Render a friendly crab mascot at 4x and downscale to 128x128, several frames."""
import sys
from PIL import Image, ImageDraw

OUT_DIR = sys.argv[1]
S = 4                    # supersample factor
SIZE = 128 * S
BODY = (232, 93, 42)     # orange-red
BODY_DARK = (180, 60, 25)
BODY_LIGHT = (250, 140, 80)
BLACK = (0, 0, 0)
WHITE = (255, 255, 255)
EYE_DARK = (30, 30, 30)


def px(v):
    return int(v * S)


def draw_crab(claw_lift, blink):
    img = Image.new("RGB", (SIZE, SIZE), BLACK)
    d = ImageDraw.Draw(img)
    cx, cy = 64, 70

    # legs: 3 per side, behind the body
    for side in (-1, 1):
        for i, (dx, dy, ex, ey) in enumerate([(26, 4, 44, 18), (28, 10, 46, 30), (26, 16, 42, 40)]):
            x0, y0 = cx + side * dx, cy + dy
            x1, y1 = cx + side * ex, cy + ey
            d.line([(px(x0), px(y0)), (px(x1), px(y1))], fill=BODY_DARK, width=px(4))
            # foot segment bends down
            d.line([(px(x1), px(y1)), (px(x1 + side * 2), px(y1 + 8))], fill=BODY_DARK, width=px(3.5))

    # claw arms
    for side in (-1, 1):
        ax0, ay0 = cx + side * 24, cy - 6
        ax1, ay1 = cx + side * 44, cy - 16 - claw_lift
        d.line([(px(ax0), px(ay0)), (px(ax1), px(ay1))], fill=BODY_DARK, width=px(6))
        # claw: big rounded pincer with a notch
        ccx, ccy = cx + side * 50, cy - 22 - claw_lift
        d.ellipse([px(ccx - 11), px(ccy - 11), px(ccx + 11), px(ccy + 11)], fill=BODY, outline=BODY_DARK, width=px(1.5))
        # notch to make it look like a pincer
        nx = ccx + side * 6
        d.polygon([(px(nx), px(ccy - 2)), (px(ccx + side * 14), px(ccy - 9)), (px(ccx + side * 14), px(ccy + 4))], fill=BLACK)
        # highlight
        d.ellipse([px(ccx - 6 - side * 1), px(ccy - 8), px(ccx - 1 - side * 1), px(ccy - 4)], fill=BODY_LIGHT)

    # body
    d.ellipse([px(cx - 30), px(cy - 20), px(cx + 30), px(cy + 20)], fill=BODY, outline=BODY_DARK, width=px(1.5))
    # body highlight
    d.ellipse([px(cx - 18), px(cy - 15), px(cx - 4), px(cy - 8)], fill=BODY_LIGHT)

    # eye stalks + eyes
    for side in (-1, 1):
        ex = cx + side * 11
        d.line([(px(ex), px(cy - 14)), (px(ex), px(cy - 28))], fill=BODY_DARK, width=px(3))
        if blink:
            d.line([(px(ex - 5), px(cy - 29)), (px(ex + 5), px(cy - 29))], fill=WHITE, width=px(2))
        else:
            d.ellipse([px(ex - 6), px(cy - 35), px(ex + 6), px(cy - 23)], fill=WHITE, outline=EYE_DARK, width=px(1))
            d.ellipse([px(ex - 2.5), px(cy - 31.5), px(ex + 2.5), px(cy - 26.5)], fill=EYE_DARK)
            d.ellipse([px(ex - 1.5), px(cy - 31), px(ex), px(cy - 29.5)], fill=WHITE)

    # smile
    d.arc([px(cx - 8), px(cy - 2), px(cx + 8), px(cy + 8)], start=15, end=165, fill=BODY_DARK, width=px(2))

    return img.resize((128, 128), Image.LANCZOS)


frames = [
    draw_crab(0, False),
    draw_crab(4, False),
    draw_crab(8, False),
    draw_crab(4, True),
]
for i, f in enumerate(frames):
    f.save(f"{OUT_DIR}/crab_{i}.png")
    with open(f"{OUT_DIR}/crab_{i}.rgb", "wb") as fh:
        fh.write(f.convert("RGB").tobytes())
print("ok", len(frames), "frames")
