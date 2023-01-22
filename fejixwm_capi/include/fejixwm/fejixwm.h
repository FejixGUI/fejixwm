#ifndef FEJIXWM_H_
#define FEJIXWM_H_


#include <stdint.h>


typedef const char * fj_string_t;
typedef uint32_t fj_bool_t;
typedef uint32_t fj_status_t;

typedef void * fj_wm_t;
typedef uint32_t fj_wid_t;

typedef uint32_t fj_window_flags_t;
typedef uint32_t fj_event_type_t;

struct fj_wm_info;
typedef struct fj_wm_info fj_wm_info_t;

struct fj_window_info;
typedef struct fj_window_info fj_window_info_t;

struct fj_pixel_size;
typedef struct fj_pixel_size fj_pixel_size_t;

struct fj_resize_event;
typedef struct fj_resize_event fj_resize_event_t;

struct fj_event;
typedef struct fj_event fj_event_t;

typedef void (* fj_event_handler_t)(fj_event_t * event);


enum fj_status {
    FJ_OK = 0,
    FJ_ERR_UNSUPPORTED,
    FJ_ERR_PLATFORM_API,
    FJ_ERR_GRAPHICS_API,
    FJ_ERR_INTERNAL,
};

enum { FJ_WINDOW_EVENT_MASK = 1 << 15 };

enum fj_event_type {
    FJ_EVENT_WINDOW_CLOSE  = 1 | FJ_WINDOW_EVENT_MASK,
};

enum fj_window_flags {
    FJ_WINDOW_RESIZABLE = 0b00000001,
};

struct fj_pixel_size {
    uint32_t width;
    uint32_t height;
};

struct fj_resize_event {
    fj_pixel_size_t size;
};

struct fj_event {
    fj_event_type_t event_type;
    fj_wid_t window_id;

    union {
        fj_resize_event_t resize_event;
    };
};


struct fj_wm_info {
    fj_string_t name;
    fj_event_handler_t event_handler;
};


struct fj_window_info {
    fj_pixel_size_t size;
    fj_window_flags_t flags;
    fj_wid_t id;
};


fj_status_t fj_wm_new(fj_wm_t * wm, fj_wm_info_t * info);
void fj_wm_del(fj_wm_t wm);
void fj_wm_run(fj_wm_t wm);

fj_status_t fj_window_new(fj_wm_t wm, fj_window_info_t * info);
void fj_window_del(fj_wm_t wm, fj_wid_t wid);

// TODO Error messages, canvases, interfaces

// #ifdef FJ_MODULE_XXX
//      definitions...
// #endif

#endif // FEJIXWM_H_