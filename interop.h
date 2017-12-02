#include "stdint.h"
#include "stdlib.h"

void* rawspeed_metadata_init(const char* filename);
void rawspeed_metadata_delete(void* ptr);
void* rawspeed_rawimage_decode(const uint8_t* data, size_t size, const void* metadata_ptr);
void rawspeed_rawimage_delete(void* ptr);
uint8_t* rawspeed_rawimage_data(const void* ptr);
int rawspeed_rawimage_width(const void* ptr);
int rawspeed_rawimage_height(const void* ptr);
int rawspeed_rawimage_pitch(const void* ptr);
