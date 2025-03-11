```
import math

interface Displayable:
    public fn display() -> string

struct Coordinate:
    x: float
    y: float

Coordinate:
    fn distance():
        math.sqrt(self.x ^ 2 + self.y ^ 2)

struct Point from Coordinate:
    pass

Point:
    fn move(dx: float, dy: float):
        self.x += dx
        self.y += dy

Point as Displayable:
    fn display():
        f'({self.x}, {self.y})'

fn main():
    mut p = Point{x = 10, y = 10}
    p.move(10, 20)
    print(f'Point: {p}')
```