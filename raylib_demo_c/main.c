#include "raylib.h"

int whatever_this_is(void *what) {
  
}

int main(void)
{
  InitWindow(800, 450, "C + raylib");
  while (!WindowShouldClose())
    {
      BeginDrawing();
      ClearBackground(RAYWHITE);
      DrawText("Hello World! From C this time", 190, 200, 20, BLACK);
      EndDrawing();
    }
  CloseWindow();
  return 0;
}
