import numpy as np
from PIL import Image
import time
import os

class Screen:
    def __init__(self):
        self.x = os.get_terminal_size().columns
        self.y = os.get_terminal_size().lines
        self.buffer = np.full((self.y, self.x), ' ', dtype=np.dtype('U1'))  # Initialize with spaces

    def img2ascii(self, img):
        img = img.convert('L')  # Convert image to grayscale
        arr = np.array(img, dtype=np.uint8)
        # Create a lookup table for mapping brightness values to characters
        lookup_table = np.empty(256, dtype=np.dtype('U1'))
        lookup_table[:100] = " "
        lookup_table[100:200] = "*"
        lookup_table[200:] = "#"
        # Map brightness values to characters using the lookup table
        char_array = lookup_table[arr]
        return char_array

    def render_sprite(self, file: str, size: tuple, location: tuple) -> None:
        with Image.open(file) as img:
            rgb = img.resize(size, Image.Resampling.NEAREST)
        sprite_ascii = self.img2ascii(rgb)

        # Check if sprite will fit
        assert location[0] + size[0] <= self.x
        assert location[1] + size[1] <= self.y

        for y in range(size[1]):
            for x in range(size[0]):
                self.buffer[location[1] + y][location[0] + x] = sprite_ascii[y][x]

    def update(self):
        for row in self.buffer:
            print(''.join(row))

scr = Screen()

def process_frame():
    time.sleep(1/15)
# F P S
csx = int(scr.x / 9) + 1
csy = int(scr.y / 3) + 1

scr.render_sprite("../sprites/f.jpg", (csx, csy), (0 * csx, 0 * csy))
scr.render_sprite("../sprites/p.jpg", (csx, csy), (1 * csx, 0 * csy))
scr.render_sprite("../sprites/s.jpg", (csx, csy), (2 * csx, 0 * csy))

scr.render_sprite("../sprites/0.jpg", (csx, csy), (4 * csx, 0 * csy))
scr.update()

while True:
    t0 = time.perf_counter()
    process_frame()
    t1 = time.perf_counter()
    
    elapsed = t1-t0
    fps = 1/elapsed
    fps = int(round(fps, 0))
    
    listed = list(str(fps))
    for i in range(len(listed)):
        scr.render_sprite(f"../sprites/{listed[i]}.jpg", (csx,csy), (csx*(i+4), 0 * csy))
    scr.update()
    
