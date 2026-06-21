#include <psp2/ctrl.h>
#include <pthread.h>
#include <stdlib.h>
#include <string.h>
#include <vita2d.h>

#define UNUSED(x) __attribute__((unused)) x
#define PGF_FONT_PATH "ux0:app/SAVSYNC01/sce_sys/resources/font.pgf"
#define VITA_DISPLAY_WIDTH 960
#define VITA_DISPLAY_HEIGHT 544

// init vita2d context
void v2d_init();

// exit vita2d context
void v2d_exit();

void v2d_free_texture(void *data);

void *v2d_load_png(const char *path);
void *v2d_load_png_buf(const void *buf);
void *v2d_load_jpg(const char *path);
void *v2d_load_jpg_buf(const void *buf, unsigned long size);

void v2d_draw_texture(const void *texture, float x, float y);

void v2d_draw_texture_scale(const void *texture, float x, float y, float sx,
                            float sy);

void v2d_draw_text(int x, int y, unsigned int color, float scale,
                   const char *text);

int v2d_text_width(float scale, const char *text);

int v2d_text_height(float scale, const char *text);

unsigned int v2d_ctrl_peek_positive();

void *v2d_get_full_screenshot();

void *v2d_get_screenshot();

unsigned int v2d_color(int r, int g, int b, int a);
