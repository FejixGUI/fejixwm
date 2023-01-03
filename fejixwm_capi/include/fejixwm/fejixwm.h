#ifndef FEJIXWM_H_
#define FEJIXWM_H_


#include <stdint.h>


enum fj_status {
    FJ_OK       = 0,
    FJ_ERROR,
};

enum fj_event_type {
    FJ_EVENT_CLOSE  = 1,
};

enum fj_window_flags {
    FJ_WINDOW_RESIZABLE = 0b00000001,
};


typedef const char * fj_str_t;
typedef uint32_t fj_bool_t;
typedef uint32_t fj_status_t;

typedef void * fj_app_t;
typedef void * fj_app_ref_t;
typedef void * fj_window_t;
typedef void * fj_abstract_surface_t;

typedef uint32_t fj_window_id_t;
typedef uint32_t fj_window_flags_t;
typedef uint32_t fj_event_type_t;
typedef uint32_t fj_event_outcome_t;

typedef struct fj_pixel_size {
    uint32_t width;
    uint32_t height;
} fj_pixel_size_t;

typedef struct fj_resize_event {
    fj_pixel_size_t size;
} fj_resize_event_t;

typedef struct fj_event {
    fj_window_id_t window_id;
    fj_bool_t is_window_event;
    fj_event_type_t event_type;

    union {
        fj_resize_event_t resize_event;
    };
} fj_event_t;

typedef fj_event_outcome_t (* fj_event_handler_t)(fj_event_t * event);

typedef struct fj_window_params {
    fj_app_ref_t app;
    fj_pixel_size_t size;
    fj_window_flags_t flags;
    fj_window_id_t id;
} fj_window_params_t;


fj_app_t fj_app_new(fj_str_t name);
void fj_app_del(fj_app_t app);
fj_app_ref_t fj_app_get_ref(fj_app_t app);
fj_app_ref_t fj_app_ref_clone(fj_app_ref_t app_ref);
void fj_app_ref_del(fj_app_ref_t app_ref);
void fj_app_run(fj_app_t app, fj_event_handler_t event_handler);

fj_app_ref_t fj_window_get_app(fj_window_t window);
fj_status_t fj_window_get_size(fj_window_t window, fj_pixel_size_t * size);
fj_window_id_t fj_window_get_id(fj_window_t window);

fj_window_t fj_surface_get_window(fj_abstract_surface_t surface);
void fj_surface_get_size(fj_abstract_surface_t surface, fj_pixel_size_t * size);
fj_status_t fj_surface_resize(fj_abstract_surface_t surface, fj_pixel_size_t * size);


#ifdef FJ_MODULE_WINDOW_MANIP
    fj_status_t fj_window_set_title(fj_window_t window, fj_str_t title);
#endif


#endif // FEJIXWM_H_